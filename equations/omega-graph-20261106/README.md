# Omega HTTP-0 equation-only backend — November 6, 2026

**Purpose:** run mathematical operations as a headless backend, with no pixel front end or full quantum-state matrix. This is an addition to the existing HEB-C Rust 1.81 repository, not the browser-based light-hub portal, and it does not change the original qutrit or Yang–Mills engines.

## Exactly what it does

For N=3..23,887,872 three-level units on a periodic cycle, the graph-state family is specified by

```
|Psi_t> = 3^(-N/2) * SUM_{x in Z3^N} omega^[(t mod 3)*SUM_i x_i*x_{i+1}] |x>
omega^3=1, omega!=1
```

This represents preparation of all qutrits in the Fourier-plus state followed by t layers of commuting CZ3 gates on neighboring sites. It is an *exact symbolic state family*, not a general N-qutrit state vector. It uses no O(3^N) arrays.

At a contiguous cut A=0..c-1, the entanglement entropy is exactly r * log2(3), where r is the GF(3) rank of the cross-cut graph adjacency: r=0 at t divisible by 3, r=1 if one side is a single site, otherwise r=2. Schmidt rank = 3^r. A connected graph can therefore have millions of genuinely nonseparable qutrits but at most two qutrit entanglement units across this contiguous cut. **This does not demonstrate all-to-all Bell pairs, general quantum simulation, learning, physical hardware entanglement or quantum speedup.**

## Headless commands

```sh
cargo +1.81.0 test --locked --bin omega-equations
cargo +1.81.0 run --locked --bin omega-equations -- --query 6 1 3 2
cargo +1.81.0 run --locked --bin omega-equations -- --query 23887872 1 11943936 23887871
cargo +1.81.0 run --locked --bin omega-equations -- --serve 5092
curl http://127.0.0.1:5092/equations/v1/graph/6/1/3/2
```

GET endpoint fields: N, slice, cut, probe. Probe defines a basis label with only sites probe and (probe+1)%N equal to 1, the others 0; its exact cyclotomic phase is t mod 3. The service binds only to 127.0.0.1 and responds using text/plain HBP rows; no HTML, BMP, JavaScript, JSON or GUI is served. Bad sizes, probe coordinates and slices fail closed.

## Two different hashes

The state hash is SHA-256 of a **canonical symbolic specification** (N, cycle and t mod 3), not SHA-256 of a materialized full wavefunction. The chained history hash commits the previous history, current descriptor and slice index. State t=3 is physically the same graph state as t=0; its state hash repeats but its history hash differs. Neither hash creates entanglement. An outside anchor is required to authenticate the origin of a receipt.

## Evidence protocol

- Prediction first: `PREDICTION.hbp` at commit `592214f44ed095557fc853f7b980c47537dc420e`.
- Rust source: `src/bin/omega-equations.rs`, separate backend binary.
- Independent check: `verify_equations.py` computes GF(3) ranks by its own elimination routine, compares SHA-256 and checks a real TCP/HTTP loopback request.
- Hosted gates: Rust 1.81 `cargo test`, `cargo clippy -D warnings`, a binary run and the independent check. No tests from the existing engine are modified.

**Next scientific test:** add explicit local measurements, adaptive feedback and a learning objective, compare to classical metadata-sharing controls. Graph-state generation and cryptographic metadata are mathematically distinct from measured quantum correlations, and a model of the universe is not evidence that the physical universe runs on hashes.
