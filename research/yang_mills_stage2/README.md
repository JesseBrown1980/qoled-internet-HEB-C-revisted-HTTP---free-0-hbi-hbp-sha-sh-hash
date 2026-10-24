# Pinned native finite S3 Yang–Mills Stage 2 reference

**Original repository:** [Finite-universe Yang–Mills using HEB-C](https://github.com/JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C)
**Source snapshot:** commit \`276f776c80b7c22c49811ec5d6972c588fcec75e\`, \`native/\` directory. The source is included here as a **complete independent Cargo crate** with Rustfmt normalization for Rust 1.81.0; the separately owned upstream repository remains authoritative for future development.

This independent crate does **not** implement the D3Q27 reversible Host8 physics. It studies a **finite S3 non-Abelian gauge group**, exact Gauss-law invariant Hilbert-space dimensions, and a three-dimensional **single-plaquette interacting quantum Hamiltonian**. Its 512x512 stress test executes a distinct **classical gauge-equivariant toy** lattice rule. These workstreams are deliberately not conflated.

## Exact repro commands

From this directory:

\`\`\`sh
cargo +1.81.0 fmt --all -- --check
cargo +1.81.0 test --all-targets --locked --offline
cargo +1.81.0 clippy --all-targets --locked --offline -- -D warnings
cargo +1.81.0 build --release --locked --offline
target/release/hebc sector single
target/release/hebc sector double
target/release/hebc sector torus2
target/release/hebc spectrum 1 1
target/release/hebc spectrum 0 0
target/release/hebc run 3 512 0 ci-stress-512
target/release/hebc verify ci-stress-512
\`\`\`

On Windows invoke \`target\release\hebc.exe\`.

## Expected mathematical results

- One open plaquette: 4 links, 8 encoded Q-trits, raw 1,296 group configurations, **3** physical gauge-invariant basis states (fully calculated interacting spectrum).
- Two open plaquettes: 7 links, 14 encoded Q-trits, **11** physical basis states (**dimension only**, no projected interacting spectrum yet).
- Periodic 2×2 torus: 8 links, 16 encoded Q-trits, **1,393** physical basis states (**dimension only**).
- In the single-plaquette normalized character basis, for \`J_E=J_B=1\` the finite interacting Hamiltonian excitation gap is within **[727730,727732]/65536** (rational interval, lattice energy units). The \`J_E=J_B=0\` test correctly does **not** certify a positive gap.
- A 512×512 classical stress model has 262,144 sites, 524,288 gauge-link labels, and **1,048,576 canonical two-Q-trit link encoding positions**. This is not a million-particle quantum many-body calculation, and it is not comparable to the 23-billion virtual-ID namespace.

The Rust code uses only native integer arithmetic, no JSON/JavaScript/Node/V8, no screen, and no direct Host8 disk I/O. Reproducible HBP/HBI receipts and SHA hashes are generated locally.

**Scientific boundary:** Finite S3 excitation gaps and deterministic classical gauge-covariant tests do not establish existence of a four-dimensional continuum quantum Yang–Mills theory with positive mass gap for compact simple Lie groups.
