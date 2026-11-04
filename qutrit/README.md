# Qutrits in powers of 3 on HTTP-0

The operator's direction, November 4, 2026: *"I want you to make qubits in the powers of 3. I want you to make 23 million-ish qubits using HTTP 0 and run that."*

**Status: built and run on one laptop CPU, in integers, with Rust 1.81 and clippy `-D warnings`. Every predicted row was met. The second seat's attack is OPEN.**

[The prediction](PREDICTION.hbp) was pushed before the code, at `42aacb7`, and its additions before the 96³ run, at `a9f3dbe`. [The receipt](RECEIPT.hbp), 86 rows, is byte-reproducible, with SHA-256 `c16c54f7e8cc855ba54e89bb75833cc6b759a9f8ad58ca46f62161a50262ab5b`. [The engine](../src/qutrit.rs) runs as `hebc-portal --qutrit all`.

## Four objects, each with its own count

| Object | Qutrits | State space | What is held in memory |
|---|---|---|---|
| **Lattice:** the [96³ engine](../engine96/README.md)'s channels | **23,887,872** | 3²³⁸⁸⁷⁸⁷² | One branch at a time, 23,887,872 bytes, and the engine's second buffer. Any amplitude comes from a formula |
| **Ladder:** rings of 3, 9, 27, 81, 243 … qutrits | up to **43,046,721** = 3¹⁶, HTTP-0's namespace | 3⁴³⁰⁴⁶⁷²¹ | One branch at a time |
| **State:** a 4×4 grid, each amplitude at a 16-trit address | **16** | 3¹⁶ = 43,046,721 amplitudes | All of them, 688,747,536 bytes |
| **Window:** the first m channels of a room in the lattice | m up to **16**, inside the 23,887,872 | 3ᵐ amplitudes, the other qutrits on one branch | All 3ᵐ |

## The lattice circuit

Every qutrit gets a Fourier gate, then T = diag(1, ζ, ζ⁸) with ζ⁹ = 1. CZ3 gates run on 46,891,008 edges: 23,887,872 around each room's ring of 27 channels, and 23,003,136 along each moving channel's streaming direction. Then come 3 ticks of the engine, with 28,311,552 four-qutrit collision gates per tick, each a permutation of 81 states, and the streaming.

Fourier gates on |0⟩ give flat magnitudes, and the rest is phases and a permutation. So the amplitude of any output string b is

**3^(−n/2) · ζ^phase(U⁻³(b))**

and one read is the engine run backward from b, then one sum over the gates. The gate counts in the table below are what that sum covers.

## What was read

| Row | Predicted | Read |
|---|---|---|
| Gates on the lattice | 23,887,872 Fourier, 23,887,872 T, 46,891,008 CZ3 | Equal, counted on the register |
| Collision tables | 288 permutations | 288 |
| Reversal on 3 branches | Exact | Byte-exact, 3 of 3 |
| Mass, momentum, energy | Conserved | Conserved at every tick on all 3 |
| Phase read from the output alone, in units of ζ | Equal to the phase on the input | 8, 8, 0, each equal |
| One trit changed, then 3 ticks | At most 9,261 channels differ, within 3 rooms | 69 differ, at most 3 rooms away |
| One-trit relation against a full recomputation, 81 sites | 81 | 81, and the sum rule reads 93,806,350 both ways |
| Ring ladder, powers 1 to 16 | The relation holds on all, and the top equals `ym43::CAPACITY` | 16 of 16 |
| 16-qutrit state, norm | Within 10⁻⁹ | Within 3 × 10⁻¹⁵ at every reading |
| 3¹⁶Σp² after the first Fourier layer | 1 | 1.000000 |
| 3¹⁶Σp² at cycle 40 | Between 1.9 and 2.1 | 1.999463 |
| Fractions of amplitudes with 3¹⁶p above 1, 2, 3, at cycle 40 | 367,879, 135,335, 49,787 ppm, within 1,000 | 367,910, 135,316, 49,764 |
| Window statistic at m = 3, 4, 8, 12, 16 | 1518518, 1493827, 1499923, 1499999, 1499999 ppm, written first from an exact transfer vector | Equal on all five |
| Window amplitudes against the exact transfer vector | Within 10⁻¹² | Within 1.0 × 10⁻¹⁵ |
| Window phase at m = 3 against full recomputation | 27 of 27 | 27 of 27 |
| One amplitude read on the lattice | Under 30 s | 1.4 s |
| The whole run | Under 30 minutes | 183 s |

The 16-qutrit statistic, 3¹⁶Σp², reaches the Porter–Thomas value of 2 for a deep random circuit:

| Cycle | 1 | 2 | 3 | 4 | 6 | 8 | 12 | 16 | 20 | 24 | 32 | 40 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 3¹⁶Σp² | 1.000000 | 1.444444 | 65.982442 | 3.395840 | 2.003499 | 2.018045 | 2.000417 | 2.000647 | 1.999926 | 1.999317 | 1.999680 | 1.999463 |

## Second routes and controls

[`second_route.py`](second_route.py) shares no code with the engine, and its rows are in [`SECOND-ROUTE.hbp`](SECOND-ROUTE.hbp). [`CONTROLS.hbp`](CONTROLS.hbp) has the controls.

- **Small circuits:** on 1×5, 2×3 and 3×3 grids, exact amplitudes in Z[ζ] give Σ|a|² = 3^(Fourier gates) with no rounding, and the engine's 39,366 parts on 3×3 agree to 22 units of 2⁻⁶⁰.
- **Lattice:** Python regenerates the 23,887,872 trits and sums the phase with its own wiring in 19 s. The SHA-256, both sums and the phase all agree.
- **Ladder:** the same at all 16 powers.
- **A wrong gate:** a conjugated Fourier gate differs from the exact amplitudes on 1,458 of 1,458 parts.
- **Wrong wiring:** dropping one ring edge from the local relation at 96³ exits 1, with 75 of 81 sites holding and the sum rule at 92,035,134 against 93,806,350.

## Times and memory

On an i5-8300H, one thread, with other work on the machine: 3 forward ticks in 1.5 s, one lattice amplitude in 1.4 s, the 16-qutrit state in 136 s, and the sum over all 3¹⁶ branches of the window in 1.6 s. [All stages](TIMING-LOCAL-20261104.hbp).

## Scope

**Flat probabilities on the lattice and ladder.** Every output string has probability exactly 3⁻ⁿ, so those runs read chosen amplitudes and their phases.

**Interference in the state and the windows.** The 16-qutrit state runs 40 cycles of interfering layers. A window is a chain, so a three-entry transfer vector gives the same amplitudes in m steps.

**HTTP-0's part:** the register is `hebc::scale96`'s channels and its forward and backward steps, the ladder's top rung is `hebc::ym43::CAPACITY`, and 16-trit addresses label the state's amplitudes. The run goes through the portal's command line.

## Open

- **A final Fourier layer across the whole lattice register.** Each amplitude is then a sum of 3²³⁸⁸⁷⁸⁷² terms. `NOT_RUN`.
- **Willow's circuit:** 103 qubits at depth 40, in a different gate set. Its state space, 2¹⁰³, is first matched at 65 qutrits, since 3⁶⁴ < 2¹⁰³ < 3⁶⁵, and the state engine holds 16 here. `NOT_RUN`, and this run makes no statement about quantum advantage.
- **The second seat's attack on this folder.**
