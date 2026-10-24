//! HEB-C Canonical Discrete Reversible Ternary Physics Engine.
//! Strict integer-only arithmetic, D3Q27 lattice gas, 288 involutive collision gates,
//! and 23-billion virtual Q-trit addressing space.

#![deny(warnings)]
#![deny(clippy::all)]

pub mod audit;
pub mod fabric;
pub mod host8_fixture;
pub mod receipts;
pub mod server;
pub mod sha256;

pub const L: usize = 24;
pub const V: usize = L * L * L; // 13,824 rooms
pub const K: usize = 27; // 27 velocity channels per room
pub const N: usize = V * K; // 373,248 Q-trits per window
pub const GATES_PER_CELL: usize = 32;
pub const NUM_COLLISION_GATES: usize = 288;
pub const VIRTUAL_PORTALS: u64 = 23_000_000_000;
pub const PROBE_STRIDE: u64 = 1_000_003;
pub const PROBES_PER_STEP: usize = 8_192;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub e: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollisionGate {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
    pub lut: [u8; 81],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntegerInvariants {
    pub mass: i64,
    pub px: i64,
    pub py: i64,
    pub pz: i64,
    pub energy_2x: i64,
}

/// Generate canonical D3Q27 velocity vectors sorted in lexicographic order.
#[must_use]
pub fn make_velocities() -> [Velocity; K] {
    let mut v = [Velocity {
        x: 0,
        y: 0,
        z: 0,
        e: 0,
    }; K];
    let mut idx = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                v[idx] = Velocity {
                    x,
                    y,
                    z,
                    e: x * x + y * y + z * z,
                };
                idx += 1;
            }
        }
    }
    v
}

#[inline]
#[must_use]
pub const fn encode_4trits(a: u8, b: u8, c: u8, d: u8) -> usize {
    (a as usize) + 3 * (b as usize) + 9 * (c as usize) + 27 * (d as usize)
}

#[inline]
#[must_use]
pub const fn decode_4trits(code: usize) -> [u8; 4] {
    [
        (code % 3) as u8,
        ((code / 3) % 3) as u8,
        ((code / 9) % 3) as u8,
        ((code / 27) % 3) as u8,
    ]
}

/// Construct 288 canonical involutive collision gates.
#[must_use]
pub fn make_collisions(v: &[Velocity; K]) -> Vec<CollisionGate> {
    use std::collections::BTreeMap;

    let mut pairs: BTreeMap<(i32, i32, i32, i32), Vec<[usize; 2]>> = BTreeMap::new();
    for a in 0..K {
        for b in (a + 1)..K {
            let key = (
                v[a].x + v[b].x,
                v[a].y + v[b].y,
                v[a].z + v[b].z,
                v[a].e + v[b].e,
            );
            pairs.entry(key).or_default().push([a, b]);
        }
    }

    let mut quads: Vec<[usize; 4]> = Vec::new();
    for ps in pairs.values() {
        for i in 0..ps.len() {
            for j in (i + 1)..ps.len() {
                let mut q = [ps[i][0], ps[i][1], ps[j][0], ps[j][1]];
                q.sort_unstable();
                let mut distinct = true;
                for m in 0..3 {
                    if q[m] == q[m + 1] {
                        distinct = false;
                        break;
                    }
                }
                if distinct {
                    quads.push(q);
                }
            }
        }
    }
    quads.sort_unstable();
    quads.dedup();

    const ALL_ONES_CODE: usize = 40;
    let mut out = Vec::with_capacity(NUM_COLLISION_GATES);

    for q in quads {
        let mut lut = [0u8; 81];
        for (code, item) in lut.iter_mut().enumerate() {
            *item = code as u8;
        }

        let mut cls: BTreeMap<(i32, i32, i32, i32, i32), Vec<usize>> = BTreeMap::new();
        for code in 0..81 {
            let d = decode_4trits(code);
            let mut m = 0i32;
            let mut px = 0i32;
            let mut py = 0i32;
            let mut pz = 0i32;
            let mut e = 0i32;
            for j in 0..4 {
                let val = d[j] as i32;
                m += val;
                px += val * v[q[j]].x;
                py += val * v[q[j]].y;
                pz += val * v[q[j]].z;
                e += val * v[q[j]].e;
            }
            cls.entry((m, px, py, pz, e)).or_default().push(code);
        }

        for codes in cls.values() {
            let filtered: Vec<usize> = codes
                .iter()
                .copied()
                .filter(|&c| c != ALL_ONES_CODE)
                .collect();
            for chunk in filtered.chunks_exact(2) {
                lut[chunk[0]] = chunk[1] as u8;
                lut[chunk[1]] = chunk[0] as u8;
            }
        }

        let mut active = false;
        for (code, &dest) in lut.iter().enumerate() {
            if dest != code as u8 {
                active = true;
                break;
            }
        }
        if active {
            out.push(CollisionGate {
                a: q[0],
                b: q[1],
                c: q[2],
                d: q[3],
                lut,
            });
        }
    }
    out
}

#[inline]
#[must_use]
pub const fn idx3d(x: usize, y: usize, z: usize) -> usize {
    (x * L + y) * L + z
}

/// Precompute integer streaming destination lookup table.
#[must_use]
pub fn make_stream_destinations(v: &[Velocity; K]) -> Vec<usize> {
    let mut dest = vec![0usize; N];
    for x in 0..L {
        for y in 0..L {
            for z in 0..L {
                let src_cell = idx3d(x, y, z);
                for (k, vel) in v.iter().enumerate() {
                    let nx = ((x as i32 + vel.x + L as i32) % L as i32) as usize;
                    let ny = ((y as i32 + vel.y + L as i32) % L as i32) as usize;
                    let nz = ((z as i32 + vel.z + L as i32) % L as i32) as usize;
                    let dst_cell = idx3d(nx, ny, nz);
                    dest[src_cell * K + k] = dst_cell * K + k;
                }
            }
        }
    }
    dest
}

/// Compute exact integer invariants over the Q-trit lattice.
#[must_use]
pub fn compute_integer_invariants(state: &[u8], v: &[Velocity; K]) -> IntegerInvariants {
    let mut mass = 0i64;
    let mut px = 0i64;
    let mut py = 0i64;
    let mut pz = 0i64;
    let mut energy_2x = 0i64;

    for cell in 0..V {
        let base = cell * K;
        for (k, vel) in v.iter().enumerate() {
            let q = state[base + k] as i64;
            mass += q;
            px += q * vel.x as i64;
            py += q * vel.y as i64;
            pz += q * vel.z as i64;
            energy_2x += q * vel.e as i64;
        }
    }

    IntegerInvariants {
        mass,
        px,
        py,
        pz,
        energy_2x,
    }
}

/// Forward self-stepping transition of the discrete D3Q27 ternary universe.
pub fn step_universe_forward(
    state: &mut [u8],
    next_state: &mut [u8],
    gates: &[CollisionGate],
    dest: &[usize],
    tick: usize,
) {
    let total_g = gates.len();

    // 1. Involutive Collision Stage
    for cell in 0..V {
        let base = cell * K;
        for j in 0..GATES_PER_CELL {
            let which = (tick * GATES_PER_CELL + cell * 17 + j * 37) % total_g;
            let g = &gates[which];
            let code = encode_4trits(
                state[base + g.a],
                state[base + g.b],
                state[base + g.c],
                state[base + g.d],
            );
            let mapped = g.lut[code] as usize;
            let d = decode_4trits(mapped);
            state[base + g.a] = d[0];
            state[base + g.b] = d[1];
            state[base + g.c] = d[2];
            state[base + g.d] = d[3];
        }
    }

    // 2. Spatial Streaming Stage
    for (src_idx, &dst_idx) in dest.iter().enumerate() {
        next_state[dst_idx] = state[src_idx];
    }
    state.copy_from_slice(next_state);
}

/// Backward Loschmidt step for reversibility verification.
pub fn step_universe_backward(
    state: &mut [u8],
    prev_state: &mut [u8],
    gates: &[CollisionGate],
    dest: &[usize],
    tick: usize,
) {
    let total_g = gates.len();

    // 1. Un-stream (reverse streaming)
    for (src_idx, &dst_idx) in dest.iter().enumerate() {
        prev_state[src_idx] = state[dst_idx];
    }
    state.copy_from_slice(prev_state);

    // 2. Un-collide (gates are involutive: U = U^-1)
    for cell in 0..V {
        let base = cell * K;
        for j in (0..GATES_PER_CELL).rev() {
            let which = (tick * GATES_PER_CELL + cell * 17 + j * 37) % total_g;
            let g = &gates[which];
            let code = encode_4trits(
                state[base + g.a],
                state[base + g.b],
                state[base + g.c],
                state[base + g.d],
            );
            let mapped = g.lut[code] as usize;
            let d = decode_4trits(mapped);
            state[base + g.a] = d[0];
            state[base + g.b] = d[1];
            state[base + g.c] = d[2];
            state[base + g.d] = d[3];
        }
    }
}

/// Decode 8-byte process rooms (u64 stubs) into 27 velocity Q-trits.
pub fn unpack_room_stubs(stubs: &[u64], state: &mut [u8]) {
    for (cell, &stub) in stubs.iter().enumerate() {
        let mut val = stub;
        let base = cell * K;
        for k in 0..K {
            state[base + k] = (val % 3) as u8;
            val /= 3;
        }
    }
}

/// Generate canonical synthetic 8-byte stubs (deterministic seed: 0x4d5954484f533831).
#[must_use]
pub fn generate_synthetic_stubs() -> Vec<u64> {
    let mut stubs = vec![0u64; V];
    let mut x: u64 = 0x4d59_5448_4f53_3831;
    for slot in stubs.iter_mut() {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *slot = x;
    }
    stubs
}

/// Sample virtual Q-trit coordinate via coprimality stride across 23B slots.
#[inline]
#[must_use]
pub const fn sample_virtual_portal(ordinal: u64) -> (u64, usize, usize) {
    let virtual_id = (ordinal.wrapping_mul(PROBE_STRIDE)) % VIRTUAL_PORTALS;
    let room = (virtual_id % (V as u64)) as usize;
    let channel = ((virtual_id / (V as u64)) % (K as u64)) as usize;
    (virtual_id, room, channel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocities_count_and_moments() {
        let v = make_velocities();
        assert_eq!(v.len(), 27);
        let mut sum_x = 0;
        let mut sum_y = 0;
        let mut sum_z = 0;
        for vel in &v {
            sum_x += vel.x;
            sum_y += vel.y;
            sum_z += vel.z;
        }
        assert_eq!(sum_x, 0);
        assert_eq!(sum_y, 0);
        assert_eq!(sum_z, 0);
    }

    #[test]
    fn test_collision_gates_involutive_and_count() {
        let v = make_velocities();
        let gates = make_collisions(&v);
        assert_eq!(gates.len(), 288);
        for g in &gates {
            for code in 0..81 {
                let mapped = g.lut[code] as usize;
                let mapped_back = g.lut[mapped] as usize;
                assert_eq!(mapped_back, code, "Gate must be involutive U^2 = I");
            }
        }
    }

    #[test]
    fn test_virtual_address_coprimality() {
        fn gcd(mut a: u64, mut b: u64) -> u64 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }
        assert_eq!(gcd(PROBE_STRIDE, VIRTUAL_PORTALS), 1);
    }
}
