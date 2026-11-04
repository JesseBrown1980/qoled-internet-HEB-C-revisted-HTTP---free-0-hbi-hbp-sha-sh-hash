//! Qutrits in powers of 3 on HTTP-0. The register is the 96^3 engine's 23,887,872 channels, one three-level unit each.
//!
//! The lattice circuit is a Fourier layer on every qutrit, the qutrit T gate diag(1, z, z^8) with z^9 = 1 on every qutrit,
//! CZ3 gates along each room's ring and along each channel's streaming direction, then ticks of the engine, whose
//! collision and streaming are a reversible map of basis states. So the amplitude of an output string b is
//! 3^(-n/2) * z^phase(U^-t(b)), and one read takes the engine run backward and one sum over the gates.
//!
//! Three more objects sit beside it: a ladder of rings of 3^1 .. 3^16 qutrits, the top rung sized by HTTP-0's 16-trit
//! namespace; a full state vector on 16 qutrits, one amplitude per 16-trit address, at scale 2^60; and a window on the
//! lattice where a second Fourier layer on m qutrits sums 3^m branches of the 23,887,872-qutrit state.
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]

use crate::scale96::{seeded_occupancies, ScaledEngine};
use crate::{make_collisions, make_velocities, Velocity, K};
use std::time::Instant;

const T_PHASE: [u64; 3] = [0, 1, 8];
const TICKS: usize = 3;
const FLIP_SITES: usize = 81;
const SEED_B: u64 = 0x4854_5450_3042_5232;
const SEED_SITES: u64 = 0x4854_5450_3053_4954;
const SEED_RING: u64 = 0x4854_5450_3052_4e47;
const SEED_POWERS: u64 = 0x4854_5450_3050_4f57;
const SCALE_BITS: u32 = 60;
const HALF: i128 = 1 << (SCALE_BITS - 1);
const STATE_READS: [usize; 12] = [1, 2, 3, 4, 6, 8, 12, 16, 20, 24, 32, 40];
const WINDOWS: [usize; 5] = [3, 4, 8, 12, 16];

type C = (i64, i64);

#[inline]
fn xorshift(x: &mut u64) -> u64 {
    *x ^= *x << 13;
    *x ^= *x >> 7;
    *x ^= *x << 17;
    *x
}

/// Trits from 64-bit draws, 27 per draw, the way the engine seeds a room.
fn trit_stream(n: usize, seed: u64) -> Vec<u8> {
    let mut out = vec![0u8; n];
    let mut x = seed;
    for chunk in out.chunks_mut(K) {
        let mut stub = xorshift(&mut x);
        for t in chunk {
            *t = (stub % 3) as u8;
            stub /= 3;
        }
    }
    out
}

/// A phase change from one trit going old -> new, with its neighbours' sum.
fn flip_delta(old: u8, new: u8, neighbours: u64) -> u64 {
    let t = (T_PHASE[usize::from(new)] + 9 - T_PHASE[usize::from(old)]) % 9;
    let d = (u64::from(new) + 3 - u64::from(old)) % 3;
    (t + 3 * ((d * (neighbours % 3)) % 3)) % 9
}

/// Flip sites: every site when there are few, or the first, the last and drawn ones.
fn flip_sites(n: usize) -> Vec<(usize, u8)> {
    let mut x = SEED_SITES;
    let mut out = Vec::new();
    if n <= FLIP_SITES {
        for site in 0..n {
            out.push((site, 1 + (xorshift(&mut x) % 2) as u8));
        }
        return out;
    }
    for &site in &[0, 1, 2, 81, 82, 83, n - 1] {
        out.push((site, 1 + (xorshift(&mut x) % 2) as u8));
    }
    while out.len() < FLIP_SITES {
        let site = (xorshift(&mut x) % n as u64) as usize;
        out.push((site, 1 + (xorshift(&mut x) % 2) as u8));
    }
    out
}

/// The lattice's flip sites: flip_sites' rooms, with the drawn sites spread over the channels, so all 27 are read.
fn lattice_flip_sites(n: usize) -> Vec<(usize, u8)> {
    flip_sites(n)
        .into_iter()
        .enumerate()
        .map(|(i, (site, d))| {
            if i < 7 {
                (site, d)
            } else {
                (site - site % K + (i - 7) % K, d)
            }
        })
        .collect()
}

fn sha(bytes: &[u8]) -> String {
    crate::sha256::hash_bytes(bytes)
}

fn trits16(label: usize) -> String {
    let mut text = [b'0'; 16];
    let mut v = label;
    for slot in text.iter_mut().rev() {
        *slot = b'0' + (v % 3) as u8;
        v /= 3;
    }
    String::from_utf8_lossy(&text).into_owned()
}

#[derive(Clone, Copy, Default)]
struct PhaseSum {
    fourier_gates: u64,
    t_gates: u64,
    ring_edges: u64,
    stream_edges: u64,
    t_sum: u64,
    cz_sum: u64,
}

impl PhaseSum {
    fn mod_9(&self) -> u64 {
        (self.t_sum + 3 * self.cz_sum) % 9
    }
}

struct Lattice {
    side: usize,
    velocities: [Velocity; K],
}

impl Lattice {
    fn new(side: usize) -> Self {
        Self {
            side,
            velocities: make_velocities(),
        }
    }

    fn rooms(&self) -> usize {
        self.side * self.side * self.side
    }

    fn qutrits(&self) -> usize {
        self.rooms() * K
    }

    fn moving(&self, k: usize) -> bool {
        let v = self.velocities[k];
        v.x != 0 || v.y != 0 || v.z != 0
    }

    fn room_coordinates(&self, i: usize) -> (usize, usize, usize) {
        let room = i / K;
        (
            room / (self.side * self.side),
            (room / self.side) % self.side,
            room % self.side,
        )
    }

    /// The same channel one room along its velocity, or back against it.
    fn along(&self, i: usize, forward: bool) -> usize {
        let k = i % K;
        let (x, y, z) = self.room_coordinates(i);
        let v = self.velocities[k];
        let m = self.side as i64;
        let shift = |c: usize, d: i32| -> usize {
            let d = if forward { i64::from(d) } else { -i64::from(d) };
            ((c as i64 + d + m) % m) as usize
        };
        let (a, b, c) = (shift(x, v.x), shift(y, v.y), shift(z, v.z));
        ((a * self.side + b) * self.side + c) * K + k
    }

    fn neighbours(&self, i: usize) -> ([usize; 4], usize) {
        let k = i % K;
        let base = i - k;
        let mut ids = [base + (k + 1) % K, base + (k + K - 1) % K, 0, 0];
        if self.moving(k) {
            ids[2] = self.along(i, true);
            ids[3] = self.along(i, false);
            (ids, 4)
        } else {
            (ids, 2)
        }
    }

    /// `bad_wiring` drops the ring edge between channels 0 and 1 of each room. It is the control.
    fn neighbour_sum(&self, state: &[u8], i: usize, bad_wiring: bool) -> u64 {
        let (ids, count) = self.neighbours(i);
        let k = i % K;
        let dropped = if bad_wiring && k <= 1 {
            Some(i - k + (1 - k))
        } else {
            None
        };
        ids[..count]
            .iter()
            .filter(|&&j| Some(j) != dropped)
            .map(|&j| u64::from(state[j]))
            .sum()
    }

    /// The whole circuit's phase on one branch, gate by gate, with tables for the rooms, a separate route from `along`.
    fn phase_sum(&self, state: &[u8]) -> PhaseSum {
        let side = self.side;
        let plus: Vec<usize> = (0..side).map(|i| (i + 1) % side).collect();
        let minus: Vec<usize> = (0..side).map(|i| (i + side - 1) % side).collect();
        let hop = |c: usize, d: i32| -> usize {
            match d {
                1 => plus[c],
                -1 => minus[c],
                _ => c,
            }
        };
        let mut sum = PhaseSum::default();
        let mut room = 0usize;
        for x in 0..side {
            for y in 0..side {
                for z in 0..side {
                    let base = room * K;
                    for (k, v) in self.velocities.iter().enumerate() {
                        let a = u64::from(state[base + k]);
                        sum.fourier_gates += 1;
                        sum.t_gates += 1;
                        sum.t_sum += T_PHASE[a as usize];
                        sum.cz_sum += a * u64::from(state[base + (k + 1) % K]);
                        sum.ring_edges += 1;
                        if v.x != 0 || v.y != 0 || v.z != 0 {
                            let target = (hop(x, v.x) * side + hop(y, v.y)) * side + hop(z, v.z);
                            sum.cz_sum += a * u64::from(state[target * K + k]);
                            sum.stream_edges += 1;
                        }
                    }
                    room += 1;
                }
            }
        }
        sum
    }
}

struct Checks {
    failed: u64,
}

impl Checks {
    fn record(&mut self, name: &str, ok: bool) {
        if !ok {
            self.failed += 1;
            println!("QUTRIT_CHECK_FAILED|name={name}|json=0");
        }
    }
}

fn stamp(label: &str, start: Instant) {
    eprintln!("TIMING|stage={label}|ms={}", start.elapsed().as_millis());
}

struct BranchRead {
    input: Vec<u8>,
    output: Vec<u8>,
    phase: u64,
}

fn read_branch(
    name: &str,
    lattice: &Lattice,
    input: Vec<u8>,
    checks: &mut Checks,
) -> Result<BranchRead, String> {
    let start = Instant::now();
    let mut engine = ScaledEngine::new(lattice.side)?;
    engine.replace_state(&input)?;
    let invariants = engine.invariants();
    let mut conserved = true;
    for tick in 0..TICKS {
        engine.forward(tick);
        conserved &= engine.invariants() == invariants;
    }
    let output = engine.state_bytes().to_vec();
    stamp(&format!("branch_{name}_forward"), start);
    let read_start = Instant::now();
    let mut reader = ScaledEngine::new(lattice.side)?;
    reader.replace_state(&output)?;
    for tick in (0..TICKS).rev() {
        reader.backward(tick);
    }
    let exact = reader.state_bytes() == input.as_slice();
    let read_phase = lattice.phase_sum(reader.state_bytes());
    stamp(&format!("branch_{name}_read_one_amplitude"), read_start);
    let phase = lattice.phase_sum(&input);
    let same = phase.mod_9() == read_phase.mod_9();
    println!(
        "QUTRIT_BRANCH|name={name}|input_sha256={}|output_sha256={}|phase_mod_9={}|phase_read_from_the_output_alone={}|t_sum={}|cz_sum={}|reversal_exact={}|invariants_conserved={}|mass={}|px={}|py={}|pz={}|energy_2x={}|probability=3^-{}|json=0",
        sha(&input),
        sha(&output),
        phase.mod_9(),
        read_phase.mod_9(),
        phase.t_sum,
        phase.cz_sum,
        u8::from(exact),
        u8::from(conserved),
        invariants.mass,
        invariants.px,
        invariants.py,
        invariants.pz,
        invariants.energy_2x,
        lattice.qutrits()
    );
    checks.record("reversal_exact", exact);
    checks.record("the_ticks_moved_the_state", output != input);
    checks.record("invariants_conserved", conserved);
    checks.record("phase_read_from_the_output", same);
    Ok(BranchRead {
        input,
        output,
        phase: phase.mod_9(),
    })
}

fn lattice_flips(
    lattice: &Lattice,
    branch: &BranchRead,
    bad_wiring: bool,
    checks: &mut Checks,
) -> Result<(), String> {
    let start = Instant::now();
    let mut state = branch.input.clone();
    let n = lattice.qutrits();
    let sites = lattice_flip_sites(n);
    let mut held = 0usize;
    for &(site, d) in &sites {
        let old = state[site];
        let new = (old + d) % 3;
        let predicted = flip_delta(old, new, lattice.neighbour_sum(&state, site, bad_wiring));
        state[site] = new;
        let full = lattice.phase_sum(&state).mod_9();
        state[site] = old;
        if (branch.phase + predicted) % 9 == full {
            held += 1;
        }
    }
    let (mut weighted, mut degree) = (0u64, 0u64);
    for (i, &value) in state.iter().enumerate() {
        weighted += u64::from(value) * lattice.neighbour_sum(&state, i, bad_wiring);
        degree += lattice.neighbours(i).1 as u64;
    }
    let sum = lattice.phase_sum(&state);
    let sum_rule = weighted == 2 * sum.cz_sum;
    let degree_rule = degree == 2 * (sum.ring_edges + sum.stream_edges);
    println!(
        "QUTRIT_FLIPS|sites={}|held={held}|sum_of_trit_times_neighbours={weighted}|twice_the_cz_sum={}|sum_rule={}|degree_rule={}|json=0",
        sites.len(),
        2 * sum.cz_sum,
        u8::from(sum_rule),
        u8::from(degree_rule)
    );
    checks.record("flips_held", held == sites.len());
    checks.record("sum_rule", sum_rule);
    checks.record("degree_rule", degree_rule);
    stamp("lattice_flips", start);
    Ok(())
}

fn light_cone(
    lattice: &Lattice,
    a: &BranchRead,
    c: &BranchRead,
    site: usize,
    checks: &mut Checks,
) -> Result<(), String> {
    let (x0, y0, z0) = lattice.room_coordinates(site);
    let side = lattice.side;
    let distance = |p: usize, q: usize| -> usize {
        let d = p.abs_diff(q);
        d.min(side - d)
    };
    let (mut differ, mut outside, mut radius) = (0u64, 0u64, 0usize);
    for (i, (p, q)) in a.output.iter().zip(c.output.iter()).enumerate() {
        if p != q {
            differ += 1;
            let (x, y, z) = lattice.room_coordinates(i);
            let far = distance(x, x0).max(distance(y, y0)).max(distance(z, z0));
            radius = radius.max(far);
            if far > TICKS {
                outside += 1;
            }
        }
    }
    println!(
        "QUTRIT_LIGHT_CONE|flipped_site={site}|ticks={TICKS}|channels_that_differ={differ}|ceiling={}|largest_room_distance={radius}|outside_the_cone={outside}|json=0",
        (2 * TICKS + 1) * (2 * TICKS + 1) * (2 * TICKS + 1) * K
    );
    checks.record("light_cone", outside == 0 && differ > 0);
    Ok(())
}

fn run_lattice(side: usize, bad_wiring: bool, checks: &mut Checks) -> Result<Vec<u8>, String> {
    let lattice = Lattice::new(side);
    let start = Instant::now();
    let gates = make_collisions(&lattice.velocities);
    let bijective = gates
        .iter()
        .filter(|g| {
            let mut seen = [false; 81];
            for &v in g.lut.iter() {
                seen[usize::from(v)] = true;
            }
            seen.iter().all(|&s| s)
        })
        .count();
    let seeded = seeded_occupancies(side);
    let counts = lattice.phase_sum(&seeded);
    println!(
        "QUTRIT_LATTICE|side={side}|qutrits={}|rooms={}|hilbert_dimension=3^{}|fourier_gates={}|t3_gates={}|cz3_room_ring={}|cz3_along_streaming={}|cz3_total={}|four_qutrit_collision_gates_per_tick={}|collision_tables_that_are_permutations={bijective}|ticks={TICKS}|register_bytes={}|engine_two_buffer_bytes={}|json=0",
        lattice.qutrits(),
        lattice.rooms(),
        lattice.qutrits(),
        counts.fourier_gates,
        counts.t_gates,
        counts.ring_edges,
        counts.stream_edges,
        counts.ring_edges + counts.stream_edges,
        lattice.rooms() * 32,
        lattice.qutrits(),
        2 * lattice.qutrits()
    );
    checks.record("collision_tables", bijective == gates.len());
    stamp("lattice_counts", start);
    let site = (((side / 2) * side + side / 2) * side + side / 2) * K;
    let a = read_branch("A", &lattice, seeded, checks)?;
    let b = read_branch(
        "B",
        &lattice,
        trit_stream(lattice.qutrits(), SEED_B),
        checks,
    )?;
    drop(b);
    let mut flipped = a.input.clone();
    flipped[site] = (flipped[site] + 1) % 3;
    let c = read_branch("C_A_with_one_trit_changed", &lattice, flipped, checks)?;
    light_cone(&lattice, &a, &c, site, checks)?;
    lattice_flips(&lattice, &a, bad_wiring, checks)?;
    Ok(a.input)
}

fn ring_phase(state: &[u8]) -> (u64, u64) {
    let n = state.len();
    let (mut t, mut cz) = (0u64, 0u64);
    for (i, &value) in state.iter().enumerate() {
        t += T_PHASE[usize::from(value)];
        cz += u64::from(value) * u64::from(state[if i + 1 == n { 0 } else { i + 1 }]);
    }
    (t, cz)
}

fn run_ladder(checks: &mut Checks) -> Result<(), String> {
    let mut n = 1usize;
    for power in 1..=16u32 {
        n *= 3;
        let start = Instant::now();
        let mut state = trit_stream(n, SEED_RING);
        let (t, cz) = ring_phase(&state);
        let phase = (t + 3 * cz) % 9;
        let sites = flip_sites(n);
        let mut held = 0usize;
        for &(site, d) in &sites {
            let old = state[site];
            let new = (old + d) % 3;
            let left = state[(site + n - 1) % n];
            let right = state[(site + 1) % n];
            let predicted = flip_delta(old, new, u64::from(left) + u64::from(right));
            state[site] = new;
            let (t2, cz2) = ring_phase(&state);
            state[site] = old;
            if (phase + predicted) % 9 == (t2 + 3 * cz2) % 9 {
                held += 1;
            }
        }
        println!(
            "QUTRIT_RING|power={power}|qutrits={n}|hilbert_dimension=3^{n}|fourier_gates={n}|t3_gates={n}|cz3_ring={n}|branch_sha256={}|phase_mod_9={phase}|t_sum={t}|cz_sum={cz}|flips={}|held={held}|probability=3^-{n}|json=0",
            sha(&state),
            sites.len()
        );
        checks.record("ring_flips", held == sites.len());
        if power == 16 {
            let capacity = crate::ym43::CAPACITY as usize;
            checks.record("ring_top_is_the_namespace", n == capacity);
            for &label in &[0usize, 80, 81, 82, 83, 84, n - 1] {
                let slot = if crate::ym43::decode(label as u64).is_some() {
                    "ym"
                } else if crate::bridge43::decode(label as u64).is_some() {
                    "clay"
                } else {
                    "empty"
                };
                println!(
                    "QUTRIT_RING_LABEL|label={label}|address_trits={}|http0_slot={slot}|trit={}|json=0",
                    trits16(label),
                    state[label]
                );
            }
        }
        stamp(&format!("ring_power_{power}"), start);
    }
    Ok(())
}

// ---- Amplitudes at scale 2^60, integers throughout.

#[inline]
fn mul(a: i64, b: i64) -> i64 {
    ((i128::from(a) * i128::from(b) + HALF) >> SCALE_BITS) as i64
}

#[inline]
fn mul_half(a: i64, b: i64) -> i64 {
    ((i128::from(a) * i128::from(b) + (1i128 << SCALE_BITS)) >> (SCALE_BITS + 1)) as i64
}

#[inline]
fn cmul(a: C, w: C) -> C {
    let re = i128::from(a.0) * i128::from(w.0) - i128::from(a.1) * i128::from(w.1);
    let im = i128::from(a.0) * i128::from(w.1) + i128::from(a.1) * i128::from(w.0);
    (
        ((re + HALF) >> SCALE_BITS) as i64,
        ((im + HALF) >> SCALE_BITS) as i64,
    )
}

fn isqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = 1u128 << ((129 - n.leading_zeros()) / 2);
    loop {
        let y = (x + n / x) / 2;
        if y >= x {
            return x;
        }
        x = y;
    }
}

struct Constants {
    root: i64,
    zeta: [C; 9],
}

impl Constants {
    /// 1/sqrt(3) and the ninth roots of unity, from integer square roots and the root of 8c^3 - 6c + 1 near 3/4.
    fn new() -> Self {
        const S: u32 = 62;
        let mul62 = |a: i128, b: i128| -> i128 { (a * b + (1i128 << (S - 1))) >> S };
        let f = |c: i128| -> i128 { 8 * mul62(mul62(c, c), c) - 6 * c + (1i128 << S) };
        let (mut lo, mut hi) = (3i128 << (S - 2), 25i128 << (S - 5));
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if f(mid) < 0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let c1 = ((lo + 2) >> 2) as i64;
        let one = 1i64 << SCALE_BITS;
        let s1 = isqrt((1u128 << 120) - (c1 as u128) * (c1 as u128)) as i64;
        let c2 = 2 * mul(c1, c1) - one;
        let s2 = 2 * mul(c1, s1);
        let c3 = -(1i64 << (SCALE_BITS - 1));
        let s3 = isqrt(3u128 << 118) as i64;
        let c4 = 2 * mul(c2, c2) - one;
        let s4 = 2 * mul(c2, s2);
        Self {
            root: isqrt((1u128 << 120) / 3) as i64,
            zeta: [
                (one, 0),
                (c1, s1),
                (c2, s2),
                (c3, s3),
                (c4, s4),
                (c4, -s4),
                (c3, -s3),
                (c2, -s2),
                (c1, -s1),
            ],
        }
    }
}

/// The Fourier gate on the qutrit of this stride: x' = (x + y + z)/sqrt 3, y' = (x + wy + w^2 z)/sqrt 3,
/// z' = (x + w^2 y + wz)/sqrt 3. `conjugated` exchanges y' and z'. It is the control.
fn fourier(state: &mut [C], stride: usize, root: i64, conjugated: bool) {
    for group in state.chunks_exact_mut(stride * 3) {
        let (first, others) = group.split_at_mut(stride);
        let (second, third) = others.split_at_mut(stride);
        for ((x, y), z) in first
            .iter_mut()
            .zip(second.iter_mut())
            .zip(third.iter_mut())
        {
            let (a, b, c) = (*x, *y, *z);
            let s = (b.0 + c.0, b.1 + c.1);
            let d = (b.0 - c.0, b.1 - c.1);
            let e = (mul_half(2 * a.0 - s.0, root), mul_half(2 * a.1 - s.1, root));
            *x = (mul(a.0 + s.0, root), mul(a.1 + s.1, root));
            let up = (e.0 - (d.1 >> 1), e.1 + (d.0 >> 1));
            let down = (e.0 + (d.1 >> 1), e.1 - (d.0 >> 1));
            if conjugated {
                *y = down;
                *z = up;
            } else {
                *y = up;
                *z = down;
            }
        }
    }
}

/// T^k = diag(1, z^k, z^-k) on the qutrit of this stride.
fn t_power(state: &mut [C], stride: usize, w: C) {
    let back = (w.0, -w.1);
    for group in state.chunks_exact_mut(stride * 3) {
        let (_, others) = group.split_at_mut(stride);
        let (second, third) = others.split_at_mut(stride);
        for a in second.iter_mut() {
            *a = cmul(*a, w);
        }
        for a in third.iter_mut() {
            *a = cmul(*a, back);
        }
    }
}

/// CZ3 = w^(ab) on two qutrits, with strides lo < hi.
fn cz3(state: &mut [C], lo: usize, hi: usize, w1: C, w2: C) {
    for big in state.chunks_exact_mut(hi * 3) {
        for (h, part) in big.chunks_exact_mut(hi).enumerate().skip(1) {
            for small in part.chunks_exact_mut(lo * 3) {
                for (l, run) in small.chunks_exact_mut(lo).enumerate().skip(1) {
                    let w = if (h * l) % 3 == 1 { w1 } else { w2 };
                    for a in run.iter_mut() {
                        *a = cmul(*a, w);
                    }
                }
            }
        }
    }
}

struct Reading {
    norm_off_1e15: u128,
    norm_above: u8,
    xeb_ppm: u128,
    above_ppm: [u128; 3],
}

fn read_state(state: &[C]) -> Reading {
    let d = state.len() as u128;
    let (mut norm, mut sum_p, mut sum_p2) = (0u128, 0u128, 0u128);
    let mut above = [0u128; 3];
    let one = 1u128 << SCALE_BITS;
    for &(re, im) in state {
        let square = (i128::from(re) * i128::from(re) + i128::from(im) * i128::from(im)) as u128;
        norm += square;
        let p = square >> SCALE_BITS;
        sum_p += p;
        sum_p2 += p * p;
        let scaled = p * d;
        for (j, slot) in above.iter_mut().enumerate() {
            if scaled > (j as u128 + 1) * one {
                *slot += 1;
            }
        }
    }
    let unit = 1u128 << 120;
    let (norm_above, off) = if norm >= unit {
        (1, norm - unit)
    } else {
        (0, unit - norm)
    };
    let low = (sum_p >> 8).max(1);
    Reading {
        norm_off_1e15: ((off >> 40) * 1_000_000_000_000_000) >> 80,
        norm_above,
        // Good for a statistic up to 2^24. A basis state on 3^16 wraps, and the run's largest reading is 65.98.
        xeb_ppm: ((sum_p2 >> 16) * d / low) * 1_000_000 / low,
        above_ppm: [
            above[0] * 1_000_000 / d,
            above[1] * 1_000_000 / d,
            above[2] * 1_000_000 / d,
        ],
    }
}

fn state_sha(state: &[C]) -> String {
    let mut bytes = Vec::with_capacity(state.len() * 16);
    for &(re, im) in state {
        bytes.extend_from_slice(&re.to_le_bytes());
        bytes.extend_from_slice(&im.to_le_bytes());
    }
    sha(&bytes)
}

fn grid_edges(rows: usize, cols: usize, pattern: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    let start = pattern / 2;
    if pattern % 2 == 0 {
        for r in 0..rows {
            let mut c = start;
            while c + 1 < cols {
                edges.push((r * cols + c, r * cols + c + 1));
                c += 2;
            }
        }
    } else {
        for c in 0..cols {
            let mut r = start;
            while r + 1 < rows {
                edges.push((r * cols + c, (r + 1) * cols + c));
                r += 2;
            }
        }
    }
    edges
}

fn print_read(tag: &str, cycle: usize, when: &str, m: usize, reading: &Reading) {
    println!(
        "{tag}|qutrits={m}|cycle={cycle}|when={when}|norm_off_1e-15={}|norm_above_one={}|xeb_ppm={}|dp_above_1_ppm={}|dp_above_2_ppm={}|dp_above_3_ppm={}|json=0",
        reading.norm_off_1e15,
        reading.norm_above,
        reading.xeb_ppm,
        reading.above_ppm[0],
        reading.above_ppm[1],
        reading.above_ppm[2]
    );
}

/// A circuit on a rows x cols grid: each cycle a Fourier layer, a drawn power of T on each qutrit, one of four CZ3 patterns.
fn evolve(rows: usize, cols: usize, cycles: usize, conjugated: bool, k: &Constants) -> Vec<C> {
    let m = rows * cols;
    let d = 3usize.pow(m as u32);
    let strides: Vec<usize> = (0..m).map(|q| 3usize.pow(q as u32)).collect();
    let mut state: Vec<C> = vec![(0, 0); d];
    state[0] = (1i64 << SCALE_BITS, 0);
    let mut x = SEED_POWERS;
    let (mut fourier_count, mut t_count, mut t_zero, mut cz_count) = (0u64, 0u64, 0u64, 0u64);
    for cycle in 1..=cycles {
        let start = Instant::now();
        for &stride in &strides {
            fourier(&mut state, stride, k.root, conjugated);
            fourier_count += 1;
        }
        if cycle == 1 {
            print_read(
                "QUTRIT_STATE",
                1,
                "after_the_first_fourier_layer",
                m,
                &read_state(&state),
            );
        }
        for &stride in &strides {
            let power = (xorshift(&mut x) % 9) as usize;
            t_count += 1;
            if power == 0 {
                t_zero += 1;
            } else {
                t_power(&mut state, stride, k.zeta[power]);
            }
        }
        for &(p, q) in &grid_edges(rows, cols, (cycle - 1) % 4) {
            cz3(&mut state, strides[p], strides[q], k.zeta[3], k.zeta[6]);
            cz_count += 1;
        }
        if STATE_READS.contains(&cycle) || cycle == cycles {
            print_read(
                "QUTRIT_STATE",
                cycle,
                "end_of_cycle",
                m,
                &read_state(&state),
            );
        }
        stamp(&format!("state_{m}_cycle_{cycle}"), start);
    }
    println!(
        "QUTRIT_STATE_GATES|qutrits={m}|amplitudes={d}|cycles={cycles}|fourier={fourier_count}|t_powers={t_count}|of_them_power_zero={t_zero}|cz3={cz_count}|state_bytes={}|json=0",
        d * 16
    );
    state
}

fn labels_of(d: usize) -> Vec<usize> {
    [0usize, 1, 2, 81, 82, 83, d - 1]
        .iter()
        .copied()
        .filter(|&l| l < d)
        .collect()
}

fn run_state(rows: usize, cols: usize, cycles: usize) -> Result<(), String> {
    let k = Constants::new();
    let state = evolve(rows, cols, cycles, false, &k);
    for label in labels_of(state.len()) {
        println!(
            "QUTRIT_STATE_AMP|qutrits={}|address_trits={}|label={label}|re_at_2^60={}|im_at_2^60={}|json=0",
            rows * cols,
            trits16(label),
            state[label].0,
            state[label].1
        );
    }
    println!(
        "QUTRIT_STATE_SEAL|qutrits={}|state_sha256={}|json=0",
        rows * cols,
        state_sha(&state)
    );
    Ok(())
}

/// A small circuit with every amplitude printed, for the separate exact route. It prints the constants too.
fn run_small(rows: usize, cols: usize, cycles: usize, conjugated: bool) -> Result<(), String> {
    let k = Constants::new();
    println!("QUTRIT_ROOT|inverse_sqrt_3_at_2^60={}|json=0", k.root);
    for (power, z) in k.zeta.iter().enumerate() {
        println!(
            "QUTRIT_ZETA|power={power}|cos_at_2^60={}|sin_at_2^60={}|json=0",
            z.0, z.1
        );
    }
    let state = evolve(rows, cols, cycles, conjugated, &k);
    for (label, a) in state.iter().enumerate() {
        println!(
            "QUTRIT_SMALL_AMP|rows={rows}|cols={cols}|cycles={cycles}|label={label}|re_at_2^60={}|im_at_2^60={}|json=0",
            a.0, a.1
        );
    }
    Ok(())
}

/// On branch `base`, the first m channels of room 0 get a second Fourier layer. The other qutrits stay on the branch.
fn run_window(
    lattice: &Lattice,
    base: &[u8],
    m: usize,
    k: &Constants,
    checks: &mut Checks,
) -> Result<(), String> {
    let build_start = Instant::now();
    let d = 3usize.pow(m as u32);
    let mut fields = vec![0u64; m];
    let mut inside: Vec<(usize, usize)> = Vec::new();
    for (i, field) in fields.iter_mut().enumerate() {
        let (ids, count) = lattice.neighbours(i);
        for &j in &ids[..count] {
            if j < m {
                if i < j {
                    inside.push((i, j));
                }
            } else {
                *field += u64::from(base[j]);
            }
        }
    }
    let mut rest = base.to_vec();
    rest[..m].fill(0);
    let rest_phase = lattice.phase_sum(&rest).mod_9();
    let local = |digits: &[u8]| -> u64 {
        let mut phase = 0u64;
        for (&w, &field) in digits.iter().zip(fields.iter()) {
            phase += T_PHASE[usize::from(w)] + 3 * u64::from(w) * field;
        }
        for &(a, b) in &inside {
            phase += 3 * u64::from(digits[a]) * u64::from(digits[b]);
        }
        phase % 9
    };
    let mut checked = 0u64;
    let mut matched = 0u64;
    if m == 3 {
        let mut digits = vec![0u8; m];
        for _ in 0..d {
            rest[..m].copy_from_slice(&digits);
            checked += 1;
            if lattice.phase_sum(&rest).mod_9() == (rest_phase + local(&digits)) % 9 {
                matched += 1;
            }
            for t in digits.iter_mut() {
                *t += 1;
                if *t < 3 {
                    break;
                }
                *t = 0;
            }
        }
        checks.record("window_phase", checked == matched);
    }
    drop(rest);
    let magnitude = isqrt((1u128 << 120) / d as u128) as i64;
    let scaled: Vec<C> = k
        .zeta
        .iter()
        .map(|z| (mul(z.0, magnitude), mul(z.1, magnitude)))
        .collect();
    let mut state: Vec<C> = vec![(0, 0); d];
    let mut digits = vec![0u8; m];
    for amplitude in state.iter_mut() {
        *amplitude = scaled[local(&digits) as usize];
        for t in digits.iter_mut() {
            *t += 1;
            if *t < 3 {
                break;
            }
            *t = 0;
        }
    }
    stamp(&format!("window_{m}_build"), build_start);
    let sum_start = Instant::now();
    let flat = read_state(&state);
    for q in 0..m {
        fourier(&mut state, 3usize.pow(q as u32), k.root, false);
    }
    stamp(&format!("window_{m}_sum_all_branches"), sum_start);
    let reading = read_state(&state);
    let text: Vec<String> = fields.iter().map(u64::to_string).collect();
    let pairs: Vec<String> = inside.iter().map(|(a, b)| format!("{a}-{b}")).collect();
    println!(
        "QUTRIT_WINDOW|m={m}|register_qutrits={}|amplitudes={d}|terms_in_each_amplitude={d}|rest_phase_mod_9={rest_phase}|fields={}|edges_inside={}|full_recomputations={checked}|of_them_equal={matched}|xeb_before_the_second_layer_ppm={}|xeb_ppm={}|norm_off_1e-15={}|norm_above_one={}|dp_above_1_ppm={}|dp_above_2_ppm={}|dp_above_3_ppm={}|state_sha256={}|json=0",
        lattice.qutrits(),
        text.join(","),
        pairs.join(","),
        flat.xeb_ppm,
        reading.xeb_ppm,
        reading.norm_off_1e15,
        reading.norm_above,
        reading.above_ppm[0],
        reading.above_ppm[1],
        reading.above_ppm[2],
        state_sha(&state)
    );
    for label in labels_of(d) {
        println!(
            "QUTRIT_WINDOW_AMP|m={m}|label={label}|re_at_2^60={}|im_at_2^60={}|json=0",
            state[label].0, state[label].1
        );
    }
    Ok(())
}

fn finish(checks: &Checks, label: &str) -> Result<(), String> {
    println!("QUTRIT_DONE|run={label}|failed={}|json=0", checks.failed);
    if checks.failed == 0 {
        Ok(())
    } else {
        Err(format!("{} qutrit checks failed", checks.failed))
    }
}

/// `all`, `lattice [24|48|96] [mutate=bad_wiring]`, `ladder`, `state ROWS COLS CYCLES`, or
/// `small ROWS COLS CYCLES [mutate=conjugated_fourier]`.
pub fn run(args: &[String]) -> Result<(), String> {
    let usage = "usage: --qutrit all | lattice [24|48|96] [mutate=bad_wiring] | ladder | state ROWS COLS CYCLES | small ROWS COLS CYCLES [mutate=conjugated_fourier]";
    let number = |i: usize| -> Result<usize, String> {
        args.get(i)
            .ok_or_else(|| usage.to_string())?
            .parse::<usize>()
            .map_err(|_| usage.to_string())
    };
    let mut checks = Checks { failed: 0 };
    match args.first().map(String::as_str) {
        Some("all") => {
            let side = 96;
            let base = run_lattice(side, false, &mut checks)?;
            run_ladder(&mut checks)?;
            let k = Constants::new();
            let lattice = Lattice::new(side);
            for &m in &WINDOWS {
                run_window(&lattice, &base, m, &k, &mut checks)?;
            }
            drop(base);
            run_state(4, 4, 40)?;
            finish(&checks, "all")
        }
        Some("lattice") => {
            let side = args
                .get(1)
                .filter(|a| !a.starts_with("mutate"))
                .map_or(Ok(96), |a| {
                    a.parse::<usize>().map_err(|_| usage.to_string())
                })?;
            if !matches!(side, 24 | 48 | 96) {
                return Err(usage.into());
            }
            let bad = args.iter().any(|a| a == "mutate=bad_wiring");
            let base = run_lattice(side, bad, &mut checks)?;
            let lattice = Lattice::new(side);
            let k = Constants::new();
            for &m in &[3usize, 4, 8] {
                run_window(&lattice, &base, m, &k, &mut checks)?;
            }
            finish(&checks, "lattice")
        }
        Some("ladder") => {
            run_ladder(&mut checks)?;
            finish(&checks, "ladder")
        }
        Some("state") => {
            let (rows, cols, cycles) = (number(1)?, number(2)?, number(3)?);
            if rows * cols > 16 || rows * cols == 0 || cycles == 0 {
                return Err(usage.into());
            }
            run_state(rows, cols, cycles)?;
            finish(&checks, "state")
        }
        Some("small") => {
            let (rows, cols, cycles) = (number(1)?, number(2)?, number(3)?);
            if rows * cols > 9 || rows * cols == 0 || cycles == 0 {
                return Err(usage.into());
            }
            let conjugated = args.iter().any(|a| a == "mutate=conjugated_fourier");
            run_small(rows, cols, cycles, conjugated)?;
            finish(&checks, "small")
        }
        _ => Err(usage.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: C, b: C, ulps: i64) -> bool {
        (a.0 - b.0).abs() <= ulps && (a.1 - b.1).abs() <= ulps
    }

    #[test]
    fn ninth_roots_close_up() {
        let k = Constants::new();
        let mut w = (1i64 << SCALE_BITS, 0i64);
        for (power, z) in k.zeta.iter().enumerate() {
            assert!(close(w, *z, 64), "power {power}");
            w = cmul(w, k.zeta[1]);
        }
        assert!(close(w, (1i64 << SCALE_BITS, 0), 64));
    }

    #[test]
    fn gates_have_their_orders() {
        let k = Constants::new();
        let start: Vec<C> = (0..81)
            .map(|i| {
                (
                    (i as i64 + 1) << (SCALE_BITS - 10),
                    (40 - i as i64) << (SCALE_BITS - 10),
                )
            })
            .collect();
        let mut state = start.clone();
        for _ in 0..4 {
            fourier(&mut state, 3, k.root, false);
        }
        for _ in 0..9 {
            t_power(&mut state, 9, k.zeta[1]);
        }
        for _ in 0..3 {
            cz3(&mut state, 1, 27, k.zeta[3], k.zeta[6]);
        }
        for (a, b) in state.iter().zip(start.iter()) {
            assert!(close(*a, *b, 512));
        }
    }

    #[test]
    fn fourier_twice_is_the_parity_map() {
        let k = Constants::new();
        let mut state: Vec<C> = vec![(0, 0); 3];
        state[1] = (1i64 << SCALE_BITS, 0);
        fourier(&mut state, 1, k.root, false);
        fourier(&mut state, 1, k.root, false);
        assert!(close(state[2], (1i64 << SCALE_BITS, 0), 16));
        assert!(close(state[0], (0, 0), 16));
        assert!(close(state[1], (0, 0), 16));
    }

    #[test]
    fn small_lattice_reads_hold() {
        let mut checks = Checks { failed: 0 };
        let base = run_lattice(24, false, &mut checks).unwrap();
        assert_eq!(base.len(), 373_248);
        assert_eq!(checks.failed, 0);
    }

    #[test]
    fn the_dropped_edge_is_seen() {
        let mut checks = Checks { failed: 0 };
        run_lattice(24, true, &mut checks).unwrap();
        assert!(checks.failed > 0);
    }

    #[test]
    fn the_window_formula_equals_full_recomputation() {
        let lattice = Lattice::new(24);
        let k = Constants::new();
        let mut checks = Checks { failed: 0 };
        let base = seeded_occupancies(24);
        run_window(&lattice, &base, 3, &k, &mut checks).unwrap();
        assert_eq!(checks.failed, 0);
    }
}
