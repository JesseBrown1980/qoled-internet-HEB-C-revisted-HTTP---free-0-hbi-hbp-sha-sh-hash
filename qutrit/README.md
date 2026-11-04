# Qutrits in powers of 3 on HTTP-0

The operator's direction, November 4, 2026: *"I want you to make qubits in the powers of 3. I want you to make 23 million-ish qubits using HTTP 0 and run that."*

**Status: built and run on one laptop CPU, in integers, with Rust 1.81 and clippy `-D warnings`. GitHub's ubuntu-24.04 runner reproduces the receipt byte for byte, in 112 s at `6114590`. Every predicted row was met. The second seat attacked the run at `6114590` and refuted no number: [its reports](second-seat/REPORT.md). Its holds are applied here, and those edits are FABLE5's, unread by that seat.**

[The prediction](PREDICTION.hbp) was pushed before the code, at `42aacb7`, and its additions before the 96³ run, at `a9f3dbe`. Its row `QUTRIT_ALREADY_READ` lists what had run by then: unit tests at 24³, the 1×5, 2×3 and 3×3 circuits with the conjugated control, and the exact window statistic. [The receipt](RECEIPT.hbp), 86 rows, is byte-reproducible, with SHA-256 `c16c54f7e8cc855ba54e89bb75833cc6b759a9f8ad58ca46f62161a50262ab5b`. [The engine](../src/qutrit.rs) runs as `hebc-portal --qutrit all`.

## Four objects, each with its own count

| Object | Qutrits | State space | What is held in memory |
|---|---|---|---|
| **Lattice:** the [96³ engine](../engine96/README.md)'s channels | **23,887,872** | 3²³⁸⁸⁷⁸⁷² | One branch at a time, 23,887,872 bytes, and the engine's second buffer. Any amplitude comes from a formula |
| **Ladder:** rings of 3, 9, 27, 81, 243 … qutrits | up to **43,046,721** = 3¹⁶, HTTP-0's namespace | 3⁴³⁰⁴⁶⁷²¹ | One branch at a time |
| **State:** a 4×4 grid, numbered row by row, each amplitude at a 16-trit address whose last trit is qutrit 0 and whose first is qutrit 15 | **16** | 3¹⁶ = 43,046,721 amplitudes | All of them, 688,747,536 bytes |
| **Window:** the first m channels of a room in the lattice | m up to **16**, inside the 23,887,872 | 3ᵐ amplitudes, the other qutrits on one branch | All 3ᵐ |

## The lattice circuit

Every qutrit gets a Fourier gate, then T = diag(1, ζ, ζ⁸) with ζ⁹ = 1. CZ3 gates run on 46,891,008 edges: 23,887,872 around each room's ring of 27 channels, and 23,003,136 along each moving channel's streaming direction. Then come 3 ticks of the engine, with 28,311,552 four-qutrit collision gates per tick, each a permutation of 81 states that is its own inverse, with 15 swaps and 51 fixed codes, and the streaming.

Fourier gates on |0⟩ give flat magnitudes, and the rest is phases and a permutation. So the amplitude of any output string b is

**3^(−n/2) · ζ^phase(U⁻³(b))**

and one read runs the engine backward from b, 84,934,656 collision-table lookups on that one string, then sums 70,778,880 phase terms for its T and CZ3 gates on the string it reaches. The Fourier gates act through the factor 3^(−n/2).

## What was read

| Row | Predicted | Read |
|---|---|---|
| Gates on the lattice | 23,887,872 Fourier, 23,887,872 T, 46,891,008 CZ3 | Equal, counted by the loop that sums one branch's phase. A sorted edge list has 46,891,008 distinct edges, degree 4 on 23,003,136 sites and 2 on 884,736, and the sum on an all-ones string reads 46,891,008 |
| Collision tables | 288 permutations | 288 |
| Reversal on 3 branches | Exact | Byte-exact, 3 of 3 |
| Mass, momentum, energy | Conserved | Conserved at every tick on all 3 |
| Phase read from the output alone, in units of ζ | Equal to the phase on the input | 8, 8, 0, each equal |
| One trit changed, then 3 ticks | At most 9,261 channels differ, within 3 rooms | 69 differ, at most 3 rooms away |
| The same on 162 flips: 3 rooms, 27 channels, both changes (second seat) | Not predicted | 3 to 687 differ, median 278, 158 at 3 rooms away and 4 at 2. The tick order bounds it at 5³ × 27 = 3,375, and 9,261 = 7³ × 27 sits above that. [Rows](second-seat/lattice/probe_cone.hbp) |
| One-trit relation against a full recomputation, 81 sites across all 27 channels | 81 | 81, and the sum rule reads 93,806,350 both ways |
| Ring ladder, powers 1 to 16 | The relation holds on all, and the top equals `ym43::CAPACITY` | 16 of 16. Powers 1 to 4 read every site. Above that each power reads 81 draws, with 72, 74 and 80 distinct site-and-change pairs at powers 5, 6 and 7, and 81 from power 8 up |
| 16-qutrit state, norm | Within 10⁻⁹ | Within 4 × 10⁻¹⁵ at every reading. The receipt's column is a floor, and the largest value is 3.963 × 10⁻¹⁵, after the first Fourier layer |
| 3¹⁶Σp² after the first Fourier layer | 1 | 1.000000 |
| 3¹⁶Σp² at cycle 40 | Between 1.9 and 2.1 | 1.999463 |
| Fractions of amplitudes with 3¹⁶p above 1, 2, 3, at cycle 40 | 367,879, 135,335, 49,787 ppm, within 1,000 | 367,910, 135,316, 49,764 |
| Window statistic at m = 3, 4, 8, 12, 16 | 1518518, 1493827, 1499923, 1499999, 1499999 ppm, written first from an exact transfer vector | Equal on all five. The exact form at every m is 3/2 − ½(−1/3)ᵐ, which depends on the chain and T alone, so the branch and the fields are tested by the amplitude rows |
| Window norm | Within 10⁻⁹ | Within 9 × 10⁻¹⁵ at every m |
| Window amplitudes against the exact transfer vector, on the fields in the receipt | Within 10⁻¹² | Within 1.02 × 10⁻¹⁵, on the 29 amplitudes printed in the receipt, 4 to 7 per m. The second seat's own wiring gives the same fields and all five state seals |
| Window phase at m = 3 against full recomputation | 27 of 27 | 27 of 27 |
| One amplitude read on the lattice | Under 30 s | 1.44, 1.47, 1.42 s on the 3 branches |
| The whole run | Under 30 minutes | 183 s |

The 16-qutrit state space equals 25.36 qubits. Its statistic, 3¹⁶Σp², the receipt's `xeb_ppm`, is computed from the state itself. It reaches the Porter–Thomas value of 2 for a deep random circuit. At cycle 40 the statistic sits 537 ppm from 2 and the three fractions 31, 19 and 23 ppm from their values, each inside one standard deviation for 3¹⁶ independent Porter–Thomas amplitudes (681, 73, 52 and 33 ppm):

| Cycle | 1 | 2 | 3 | 4 | 6 | 8 | 12 | 16 | 20 | 24 | 32 | 40 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 3¹⁶Σp² | 1.000000 | 1.444444 | 65.982442 | 3.395840 | 2.003499 | 2.018045 | 2.000417 | 2.000647 | 1.999926 | 1.999317 | 1.999680 | 1.999463 |

This statistic reads scrambling, and its spread at this size is 2/√(3¹⁶) = 3 × 10⁻⁴. The second seat changed the gates in a separate engine and read cycle 40. These read 2.000 within 0.0007, with the fractions in the predicted band: F† for F; a real orthogonal 3×3 for F; every T power set to 1; diag(1, ζ, ζ²) for T; a CZ3 table wrong at (2,2); one edge dropped; the 240 CZ3 on a wrong graph; one F missing. These move it: pattern 3 replaced by pattern 1 reads 4.05 on 16 qutrits, and on 3×4 Clifford powers only read 1.000, T in the first cycle only 1.691, and F never on one qutrit 6.00. The gates' identity rests on the exact small circuits, the amplitude rows and the state seal.

## Second routes and controls

[`second_route.py`](second_route.py) is a separate Python program with its own arithmetic and wiring, written from the same circuit definition and seeds, and its rows are in [`SECOND-ROUTE.hbp`](SECOND-ROUTE.hbp). [`CONTROLS.hbp`](CONTROLS.hbp) has the controls.

- **Small circuits:** on 1×5, 2×3 and 3×3 grids, exact amplitudes in Z[ζ] give Σ|a|² = 3^(Fourier gates) exactly, and the engine's 39,366 parts on 3×3 agree to 22 units of 2⁻⁶⁰.
- **Lattice, phases:** Python regenerates the 23,887,872 trits of all 3 branches and sums each phase with its own wiring. The SHA-256, both sums and the phase all agree. A CZ3 edge is symmetric, so this route reads each channel's line of edges, and the sign of a velocity is read by the engine route.
- **Lattice, engine:** Python derives the 288 gates, equal to the pinned SHA-256, and runs the 3 ticks on all 3 branches, with slices for the streaming. The 3 output SHA-256 agree. On branch A, 31,455,566 of the 84,934,656 gate applications change trits, and 15,908,063 of the 23,887,872 channels differ between input and output.
- **What the rows read:** reversal, conservation, the phase read from the output and the light-cone check hold for any reversible map that conserves, the identity included. The output SHA-256 rows read the map itself, through the byte compare and the engine route.
- **Ladder:** the same at all 16 powers.
- **A wrong gate:** a conjugated Fourier gate differs from the exact amplitudes on 1,458 of 1,458 parts.
- **Wrong wiring:** dropping the ring edge between channels 0 and 1 in every room, 884,736 edges, from the local relation at 96³ exits 1, with 73 of 81 sites holding and the sum rule at 92,035,134 against 93,806,350. The prediction's row calls this mutation `one_room_edge_dropped`. On branch A a single dropped edge shows when both its trits are nonzero or a listed site touches it.
- **What the sum rule reads:** Σ xᵢNᵢ = 2·cz on one branch. A site whose trit is 0 adds nothing to it, and a rewiring that keeps each edge's weight in the total, such as forward twice on a channel, leaves it equal. The flip sites read those.
- **The 16-qutrit state, second seat:** a separate Rust 1.81 engine at scale 2⁵⁰ and 2⁶², sharing no lines with this one, reads every 3¹⁶Σp² and fraction at cycles 2 to 40 to the same ppm. 65.982442 at cycle 3 reads 65.982442605 at both scales.
- **Accuracy of the 16-qutrit amplitudes:** against the 2⁶² run, the state's distance is 9.3 × 10⁻¹⁴, and a typical amplitude differs by about 16 units of 2⁻⁶⁰. Labels 1 and 2 differ by about 1,345 units, a relative 1.2 × 10⁻¹¹: the `>> 1` halves in the Fourier gate floor, a constant ±¼ unit per gate, and the 15 Fourier gates after it gather that constant at those labels. With round-half-even in those halves they differ by at most 12 units, the distance stays 9.3 × 10⁻¹⁴, and every statistic keeps its digits.
- **Run and return:** the exact inverse after the 40 cycles returns amplitude 1 − 2 × 10⁻¹⁷ − 5 × 10⁻¹⁷i on |0…0⟩, with 1.31 × 10⁻¹³ left over the other labels, the size expected from independent rounding over 2,872 gates.

## Times

On an i5-8300H, one thread, with other work on the machine: 3 forward ticks in 1.5 s, one lattice amplitude in 1.4 to 1.5 s, the 16-qutrit state in 136 s, and the sum over all 3¹⁶ branches of the window in 1.6 s. [All stages](TIMING-LOCAL-20261104.hbp).

## The second seat, at `6114590`

Three lanes ran and none failed: [their reports, verbatim](second-seat/REPORT.md), with each lane's findings, holds and row files beside them. Their scripts are held on this machine at `D:/safety-backups/opus55-qutrit-attack-20261104-tools.tar.gz`, 403 files, SHA-256 `34838ed17fbf39f2735836b4fe5441853bb45f2baf1fa17e5c778482a8b78806`, and their upload to the Hugging Face dataset is OPEN.

| What it found | Where it went |
|---|---|
| The 81 flip sites read 23 of the 27 channels, so a mutant wrong on all of channel 19 passed every check | The drawn sites now spread over all 27. That mutant exits 1, and the receipt keeps every byte |
| An engine replaced by the identity passed the run and the route, with 4 receipt rows off | A check that the ticks moved the state, and a Python route that derives the 288 gates and runs the 3 ticks to the 3 output SHA-256 |
| The Python lattice route read one branch's input | It reads all 3 branches |
| Text that read wider than the rows, such as "one ring edge" for 884,736 edges and "within 3 × 10⁻¹⁵" for a floor | Corrected in place, with the kind of circuit stated first |
| The statistic reads scrambling, and 8 of 9 wrong gate sets still read 2.000 | Stated beside the statistic, and the gates' identity rests on the exact small circuits, the amplitude rows and the seal |

## Scope

**The lattice and ladder are phase states, and on the lattice the engine then permutes the basis strings.** Every output string has probability exactly 3⁻ⁿ and comes from exactly one input string, so each amplitude is one term, and sampling the output is drawing n uniform trits. Those runs read chosen amplitudes and their phases. A read is one pass over the n trits on a classical machine, with time in proportion to n: the ring rungs at powers 14, 15 and 16 took 527, 1,536 and 4,674 ms. The register count on these two objects is the length of the string held in memory. The interfering state measured here is the 16-qutrit row.

**Interference in the state and the windows.** The 16-qutrit state runs 40 cycles of interfering layers, one circuit with its T powers drawn from a fixed seed. A window takes branch A's input string, before any engine tick, and adds a second Fourier layer on the first m channels of room 0. It is a chain, so a three-entry transfer vector gives the same amplitudes in m steps. A window's amplitude row is A(y) = 3⁻ᵐ Σ_w ζ^(local(w) + 3w·y), conditioned on the other qutrits' branch. In the register that amplitude is 3^(−(n−m)/2) · ζ^rest_phase · A(y), with rest_phase in the same receipt row. A(y) is a product of 3×3 unitaries, so 3ᵐp is at most 3 at every m, which is why the fraction above 3 reads 0. The pinned scope row names the state alone, and the window rows added at `a9f3dbe` extend it.

**HTTP-0's part:** the register is `hebc::scale96`'s channels and its forward and backward steps, the ladder's top rung is `hebc::ym43::CAPACITY`, and 16-trit addresses label the state's amplitudes. The run goes through the portal's command line.

## Open

- **A final Fourier layer across the whole lattice register.** Each amplitude is then a sum of 3²³⁸⁸⁷⁸⁷² terms. `OPEN`.
- **An exact route for the 16-qutrit state.** The exact Z[ζ] rows reach 9 qutrits at 6 cycles. At 16 qutrits the comparison is the second seat's separate fixed-point engine.
- **A command argument for a chosen output string.** The run reads the three branch outputs, and the formula covers every string, since the engine's map is a bijection on trit strings. The second seat's patched copy read 3 drawn strings in 1,452, 1,520 and 1,476 ms.
- **An HTTP-0 route for these reads.** The run is on the portal's command line.
- **Willow's circuit:** 103 two-level qubits at depth 40, XEB fidelity 0.1%, with iSWAP-like two-qubit gates, from the [spec sheet](https://quantumai.google/static/site-assets/downloads/willow-spec-sheet.pdf). Its state space, 2¹⁰³, is first matched at 65 qutrits, since 3⁶⁴ < 2¹⁰³ < 3⁶⁵, and the state engine holds 16 here. `OPEN`. Quantum advantage is outside this folder's scope.
