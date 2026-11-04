//! Actual variable-size D3Q27 simulation, 24^3, 48^3 and 96^3.
//! State contains 27 physical trits per cell. Three exact update steps,
//! integrity checks and a reversible comparator with the original 24^3 kernel.
//! The 3^16 HTTP-0 address catalog is separate from this compute engine.
#![forbid(unsafe_code)]

use crate::{
    compute_integer_invariants, make_collisions, make_stream_destinations, make_velocities,
    step_universe_backward, step_universe_forward, unpack_room_stubs, CollisionGate,
    IntegerInvariants, Velocity, K, L,
};
use std::time::Instant;

const GATE_SHA: &str = "a7fca71ac8a85fd3acbe4188e9c246e98e4beeeaa459f3e77d40c0342aa4bb12";
const SEED: u64 = 0x4d59_5448_4f53_3831;

#[derive(Debug)]
pub struct ScaledEngine {
    pub side: usize,
    state: Vec<u8>,
    next: Vec<u8>,
    plus: Vec<usize>,
    minus: Vec<usize>,
    velocities: [Velocity; K],
    gates: Vec<CollisionGate>,
}

fn validate_size(side: usize) -> Result<usize, String> {
    if !matches!(side, 24 | 48 | 96) {
        return Err("supported finite computation side lengths are 24, 48 and 96".into());
    }
    side.checked_mul(side)
        .and_then(|v| v.checked_mul(side))
        .and_then(|v| v.checked_mul(K))
        .ok_or_else(|| "finite lattice channel count overflow".into())
}

/// Use the exact original canonical xorshift room stubs, decoded as 27 trits.
/// The first 24^3 cells are byte-identical to the existing 24^3 engine.
#[must_use]
pub fn seeded_occupancies(side: usize) -> Vec<u8> {
    let cells = side * side * side;
    let mut result = vec![0u8; cells * K];
    let mut seed = SEED;
    for room in result.chunks_exact_mut(K) {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let mut stub = seed;
        for channel in room {
            *channel = (stub % 3) as u8;
            stub /= 3;
        }
    }
    result
}

fn registered_gate_sha(gates: &[CollisionGate]) -> String {
    let mut bytes = Vec::with_capacity(gates.len() * 85);
    for gate in gates {
        bytes.extend_from_slice(&[gate.a as u8, gate.b as u8, gate.c as u8, gate.d as u8]);
        bytes.extend_from_slice(&gate.lut);
    }
    crate::sha256::hash_bytes(&bytes)
}

impl ScaledEngine {
    pub fn new(side: usize) -> Result<Self, String> {
        let n = validate_size(side)?;
        let velocities = make_velocities();
        let gates = make_collisions(&velocities);
        if gates.len() != 288 || registered_gate_sha(&gates) != GATE_SHA {
            return Err("collision gates differ from the pinned 96^3 Taylor-Green LUT".into());
        }
        let plus: Vec<usize> = (0..side).map(|i| (i + 1) % side).collect();
        let minus: Vec<usize> = (0..side).map(|i| (i + side - 1) % side).collect();
        let state = seeded_occupancies(side);
        if state.len() != n || state.iter().any(|&v| v > 2) {
            return Err("nontrit or wrong-size initial state".into());
        }
        Ok(Self {
            side,
            next: vec![0u8; n],
            state,
            plus,
            minus,
            velocities,
            gates,
        })
    }

    /// Only ternary input of the canonical grid width can replace the state.
    /// This prevents malformed external state bytes reaching LUT indexing.
    pub fn replace_state(&mut self, bytes: &[u8]) -> Result<(), String> {
        if bytes.len() != self.state.len() {
            return Err("source channel count does not match physical grid".into());
        }
        if bytes.iter().any(|&n| n > 2) {
            return Err("nontrit source occupancy rejected".into());
        }
        self.state.copy_from_slice(bytes);
        Ok(())
    }

    #[must_use]
    pub fn state_bytes(&self) -> &[u8] {
        &self.state
    }

    /// Exact integer invariants for an arbitrary finite cube side.
    #[must_use]
    pub fn invariants(&self) -> IntegerInvariants {
        let mut totals = IntegerInvariants {
            mass: 0,
            px: 0,
            py: 0,
            pz: 0,
            energy_2x: 0,
        };
        for room in self.state.chunks_exact(K) {
            for (channel, vel) in room.iter().zip(self.velocities.iter()) {
                let n = i64::from(*channel);
                totals.mass += n;
                totals.px += n * i64::from(vel.x);
                totals.py += n * i64::from(vel.y);
                totals.pz += n * i64::from(vel.z);
                totals.energy_2x += n * i64::from(vel.e);
            }
        }
        totals
    }

    /// Compute a destination coordinate on-the-fly instead of materializing
    /// a 23,887,872-element usize table (~191MB on a 64-bit platform).
    fn stream(&mut self, inverse: bool) {
        let side = self.side;
        let side2k = side * side * K;
        let sidek = side * K;
        for x in 0..side {
            let xx = [self.minus[x] * side2k, x * side2k, self.plus[x] * side2k];
            for y in 0..side {
                let yy = [self.minus[y] * sidek, y * sidek, self.plus[y] * sidek];
                for z in 0..side {
                    let zz = [self.minus[z] * K, z * K, self.plus[z] * K];
                    let src = ((x * side + y) * side + z) * K;
                    for (channel, vel) in self.velocities.iter().enumerate() {
                        let dest = xx[(vel.x + 1) as usize]
                            + yy[(vel.y + 1) as usize]
                            + zz[(vel.z + 1) as usize]
                            + channel;
                        if inverse {
                            self.next[src + channel] = self.state[dest];
                        } else {
                            self.next[dest] = self.state[src + channel];
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut self.state, &mut self.next);
    }

    fn collide(&mut self, tick: usize, inverse: bool) {
        let count = self.gates.len();
        for cell in 0..(self.side * self.side * self.side) {
            let base = cell * K;
            for index in 0..32 {
                let j = if inverse { 31 - index } else { index };
                let g = &self.gates[(tick * 32 + cell * 17 + j * 37) % count];
                let code = crate::encode_4trits(
                    self.state[base + g.a],
                    self.state[base + g.b],
                    self.state[base + g.c],
                    self.state[base + g.d],
                );
                let d = crate::decode_4trits(usize::from(g.lut[code]));
                self.state[base + g.a] = d[0];
                self.state[base + g.b] = d[1];
                self.state[base + g.c] = d[2];
                self.state[base + g.d] = d[3];
            }
        }
    }

    pub fn forward(&mut self, tick: usize) {
        self.collide(tick, false);
        self.stream(false);
    }

    pub fn backward(&mut self, tick: usize) {
        self.stream(true);
        self.collide(tick, true);
    }

    #[must_use]
    pub fn physical_bytes(&self) -> usize {
        self.state.len() + self.next.len()
    }
}

/// Alternative coordinate formula, modeled on the pinned Taylor-Green
/// zcutoff source. Checks the first 96^3 step with distinct streaming code.
/// The gate table is the same pinned canonical update in both implementations.
#[must_use]
pub fn direct_coordinate_reference_step(
    source: &[u8],
    side: usize,
    tick: usize,
    gates: &[CollisionGate],
    velocities: &[Velocity; K],
) -> Vec<u8> {
    let mut state = source.to_vec();
    for cell in 0..(side * side * side) {
        let base = cell * K;
        for j in 0..32 {
            let gate = &gates[(tick * 32 + cell * 17 + j * 37) % gates.len()];
            let code = crate::encode_4trits(
                state[base + gate.a],
                state[base + gate.b],
                state[base + gate.c],
                state[base + gate.d],
            );
            let digits = crate::decode_4trits(usize::from(gate.lut[code]));
            for (k, &channel) in [gate.a, gate.b, gate.c, gate.d].iter().enumerate() {
                state[base + channel] = digits[k];
            }
        }
    }
    let mut streamed = vec![0u8; state.len()];
    let m = side as i64;
    for x in 0..side {
        for y in 0..side {
            for z in 0..side {
                let from = ((x * side + y) * side + z) * K;
                for (k, vel) in velocities.iter().enumerate() {
                    let nx = (x as i64 + i64::from(vel.x)).rem_euclid(m) as usize;
                    let ny = (y as i64 + i64::from(vel.y)).rem_euclid(m) as usize;
                    let nz = (z as i64 + i64::from(vel.z)).rem_euclid(m) as usize;
                    streamed[((nx * side + ny) * side + nz) * K + k] = state[from + k];
                }
            }
        }
    }
    streamed
}

#[must_use]
pub fn gate_sha() -> String {
    registered_gate_sha(&make_collisions(&make_velocities()))
}

pub fn run_benchmark(side: usize, ticks: usize) -> Result<String, String> {
    validate_size(side)?;
    if ticks == 0 || ticks > 243 {
        return Err("benchmark ticks must be from 1 through 243".into());
    }
    let mut engine = ScaledEngine::new(side)?;
    let initial = engine.state.clone();
    let baseline_invariants = engine.invariants();
    let init_sha = crate::sha256::hash_bytes(&initial);
    let gates_sha = gate_sha();

    let mut legacy = None;
    let mut legacy_buffer = None;
    let mut legacy_dest = None;
    if side == L {
        let v = make_velocities();
        let stubs = crate::generate_synthetic_stubs();
        let mut old_state = vec![0u8; crate::N];
        unpack_room_stubs(&stubs, &mut old_state);
        if old_state != initial || compute_integer_invariants(&old_state, &v) != baseline_invariants
        {
            return Err("old 24^3 deterministic initial state mismatch".into());
        }
        legacy = Some(old_state);
        legacy_buffer = Some(vec![0u8; crate::N]);
        legacy_dest = Some(make_stream_destinations(&v));
    }

    let mut text = format!(
        "ENGINE96_HEADER|side={side}|cells={}|channels={}|ticks={ticks}|gate_sha256={gates_sha}|initial_sha256={init_sha}|physical_state_bytes={}|physical_two_buffer_bytes={}|virtual_addresses=UNCHANGED|source=CANONICAL_D3Q27|json=0\n",
        side * side * side,
        engine.state.len(),
        engine.state.len(),
        engine.physical_bytes()
    );
    let mut elapsed_new_ns = 0u128;
    let mut elapsed_old_ns = 0u128;
    let mut reference_first = None;
    if side == 96 {
        // Record a direct independent coordinate-streaming comparison.
        reference_first = Some(direct_coordinate_reference_step(
            &engine.state,
            side,
            0,
            &engine.gates,
            &engine.velocities,
        ));
    }

    for tick in 0..ticks {
        let before = engine.state.clone();
        let now = Instant::now();
        engine.forward(tick);
        let ns = now.elapsed().as_nanos();
        elapsed_new_ns += ns;

        if engine.invariants() != baseline_invariants || engine.state.iter().any(|&s| s > 2) {
            return Err(format!(
                "conservation or occupancy invariant failed at tick {tick}"
            ));
        }
        if before == engine.state {
            return Err(format!("no state movement at tick {tick}"));
        }
        if let Some(expected) = reference_first.take() {
            if engine.state != expected {
                return Err("96^3 direct-coordinate first-step comparator mismatch".into());
            }
        }
        if let (Some(state), Some(scratch), Some(dest)) =
            (&mut legacy, &mut legacy_buffer, &legacy_dest)
        {
            let start = Instant::now();
            step_universe_forward(state, scratch, &engine.gates, dest, tick);
            elapsed_old_ns += start.elapsed().as_nanos();
            if *state != engine.state {
                return Err(format!("old/new 24^3 update mismatch at tick {tick}"));
            }
        }
        text.push_str(&format!(
            "ENGINE96_TICK|side={side}|t={}|elapsed_ns={ns}|state_sha256={}|movement=PASS|conservation=PASS|legacy_byte_equal={}|json=0\n",
            tick + 1,
            crate::sha256::hash_bytes(&engine.state),
            if side == 24 { "PASS" } else { "NA" }
        ));
    }

    let final_sha = crate::sha256::hash_bytes(&engine.state);
    for tick in (0..ticks).rev() {
        engine.backward(tick);
    }
    if engine.state != initial || engine.invariants() != baseline_invariants {
        return Err("forward/backward Loschmidt byte recovery failed".into());
    }
    if let (Some(state), Some(scratch), Some(dest)) =
        (&mut legacy, &mut legacy_buffer, &legacy_dest)
    {
        for tick in (0..ticks).rev() {
            step_universe_backward(state, scratch, &engine.gates, dest, tick);
        }
        if *state != initial {
            return Err("original 24^3 backward Loschmidt recovery failed".into());
        }
    }
    text.push_str(&format!(
        "ENGINE96_RESULT|side={side}|channels={}|ticks={ticks}|forward_ns={elapsed_new_ns}|old24_ns={elapsed_old_ns}|final_sha256={final_sha}|return_sha256={}|collision_gate_pin=PASS|conservation=PASS|reversal=PASS|old24_byte_equal={}|reference96_first_step={}|computed_physical_channels={}|status=PASS|json=0\n",
        side * side * side * K,
        crate::sha256::hash_bytes(&engine.state),
        if side == 24 { "PASS" } else { "NA" },
        if side == 96 { "PASS" } else { "NA" },
        side * side * side * K
    ));
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_24_stubs_and_gates_match_original() {
        assert_eq!(gate_sha(), GATE_SHA);
        let source = crate::generate_synthetic_stubs();
        let mut legacy = vec![0u8; crate::N];
        unpack_room_stubs(&source, &mut legacy);
        assert_eq!(seeded_occupancies(24), legacy);
    }

    #[test]
    fn finite_size_validation_and_memory() {
        for (side, expected_channels) in [(24, 373_248), (48, 2_985_984), (96, 23_887_872)] {
            assert_eq!(validate_size(side), Ok(expected_channels));
            assert_eq!(2 * expected_channels, 2 * seeded_occupancies(side).len());
        }
        assert!(validate_size(0).is_err());
        assert!(validate_size(23).is_err());
        assert!(validate_size(97).is_err());
        assert!(run_benchmark(24, 0).is_err());
        assert!(run_benchmark(24, 244).is_err());
    }

    #[test]
    fn malformed_external_states_fail_closed() {
        let mut engine = ScaledEngine::new(24).expect("gate pin");
        let pristine = engine.state.clone();
        assert!(engine.replace_state(&[0, 1, 2]).is_err());
        let mut corrupted = pristine.clone();
        corrupted[0] = 3;
        assert!(engine.replace_state(&corrupted).is_err());
        assert_eq!(engine.state, pristine);
        assert!(engine.replace_state(&pristine).is_ok());
    }

    #[test]
    fn original_24_one_step_is_identical() {
        let mut scaled = ScaledEngine::new(24).expect("canonical gates");
        let mut legacy = scaled.state.clone();
        let mut scratch = vec![0u8; crate::N];
        let dst = make_stream_destinations(&scaled.velocities);
        step_universe_forward(&mut legacy, &mut scratch, &scaled.gates, &dst, 0);
        scaled.forward(0);
        assert_eq!(legacy, scaled.state);
        scaled.backward(0);
        assert_eq!(scaled.state, seeded_occupancies(24));
    }
}
