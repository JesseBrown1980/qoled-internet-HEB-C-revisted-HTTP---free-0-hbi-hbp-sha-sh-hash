# Actual D3Q27 compute-engine upgrade: 24³ → 48³ → 96³

**Status (November 4, 2026): implementation and prediction-first hosted benchmark.** The original HTTP-0 24³ program, its 243-step receipts, and the (3^{16}) sparse certificate namespace are retained without changing their scientific results.

The requested 23-million-scale workload is a real **96×96×96 spatial lattice × 27 ternary occupation channels = 23,887,872 mutable channels**, already exercised in the separate Taylor–Green collapsing-vortex study. Here it is supported directly as a native HEB-C physical simulation, with the same underlying D3Q27 collision gates and periodic streaming rule.

## Frozen predictions

[Prediction-before-implementation contract](PREDICTION.hbp), commit `19a6ee8720ed46b99014e523a989d433b7e05e0d`. Its acceptance conditions include original gate SHA-256, byte-identical reproduction of the existing 24³ engine, all-five-invariant conservation, reversibility, three nontrivial update steps, and a distinct direct-coordinate reference for the first 96³ step.

The source of the pinned 288 collision LUT is [Taylor–Green's 2026-10-30 cutoff experiment](https://github.com/JesseBrown1980/Taylor_green_regularity-resolved-and-solved/tree/main/results/2026-10-30-cutoff), SHA-256 `a7fca71ac8a85fd3acbe4188e9c246e98e4beeeaa459f3e77d40c0342aa4bb12`. In the original Taylor–Green record, conservation control **C1 PASS** and vortex-amplification control **A1 FAIL** are both retained.

## Real state capacity and working memory

| Spatial cube | Rooms | Mutable D3Q27 trit channels | Two 1-byte/channel state buffers |
|---|---:|---:|---:|
| 24³ | 13,824 | 373,248 | 746,496 bytes |
| 48³ | 110,592 | 2,985,984 | 5,971,968 bytes |
| **96³** | **884,736** | **23,887,872** | **47,775,744 bytes** |

A separate 8-byte packed *room representation* would use 7,077,888 bytes at 96³, but the mutable update currently uses 27 byte-sized trits per room. Two streaming buffers therefore use 47.8 MB, before the reference comparator, state snapshots and runtime overhead. The old streaming destination table (one 64-bit index per channel) would alone require about **191.1 MB** at 96³; the new engine computes each streaming destination from precomputed 1D neighbour indices and uses only O(L) extra index memory.

The system's existing HTTP-0 (3^{16}=43,046,721) virtual address scheme and its 732 verified source slots remain a separate capability. They do **not** provide the physical mutable state.

## Native computation and measurement

The new [Rust 1.81 engine](../src/scale96.rs) reuses the repository's **exact original 288 involutive collision gates**, deterministic xorshift64 seed, 32 collisions per cell per step and periodic D3Q27 streaming. Its physical state transition has no floating-point operations, external runtime libraries, JSON or Node dependency.

~~~bash
cargo +1.81.0 build --release --locked --offline
target/release/hebc-portal --bench-engine 24 3
target/release/hebc-portal --bench-engine 48 3
target/release/hebc-portal --bench-engine 96 3
~~~

Each run emits one `ENGINE96_TICK` line per real update and an `ENGINE96_RESULT` summary with:
- elapsed native **nanoseconds**, state SHA-256, actual channel count and two-buffer allocation;
- byte-for-byte equality to the **pre-existing 24³ engine** on every step;
- exact conservation of total occupancy mass, three momentum components and twice channel kinetic energy;
- exact reversal of all three steps back to the initial state;
- direct-coordinate check of the first 96³ step, implemented separately from the precomputed axis-neighbour method.

The original seeded 24³ state must also match the original source's ternary unpacking before running. Invalid sides and step counts fail closed. The [benchmark CI](../.github/workflows/engine96.yml) repeats the 96³ experiment independently and compares SHA-256 values after removing only the variable clock measurements. It retains SHA-256-sealed HBP logs as downloadable artifacts.

**Performance interpretation:** nanosecond wall timings are observations on the hosted runner, not an a-priori speedup claim. Compare with the original 24³ baseline and the recorded Taylor–Green 96³ run, noting that the latter included forcing and 192 ticks whereas this pilot uses unforced nonuniform initial data over three ticks.

## Scientific scope and next gate

This first upgrade runs **three real physical ticks** on 23,887,872 channels. It is an actual engine change and not a virtual address enlargement. It does not yet replay the entire 192-tick 96³ collapsing-vortex experiment, its forcing law, coherent-amplitude diagnostic or A1/C1 controls. Those must be implemented and checked against pinned Taylor–Green source and measurement files in the next separate step.

The Yang–Mills finite Hamiltonian and its 45,508,080 rows remain on their own solver. The 3×4 evidence reader and formal `gap_verdict=OPEN` are unchanged; HTTP-0 capacity alone cannot accelerate a separate eigensolve.

**Merge gate:** original 24³ test suite, exact 96³ conservation/reversal/reference, rustfmt, Rust 1.81 Clippy and negative controls all pass at the same reviewed PR head.
