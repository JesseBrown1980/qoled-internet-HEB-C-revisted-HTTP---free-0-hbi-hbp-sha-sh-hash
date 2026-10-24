# HEB-C — Canonical Rust 1.81 Engine, Virtual HTTP-0 Portal & Q-Trit Physics

Canonical Rust 1.81 implementation of the HEB-C discrete ternary physics kernel, native HBI/HBP/SHA/SH/HASH wire fabric, and zero-JavaScript HTTP-0 portal.

## Quality Gate & Guarantees
- **Toolchain**: Rust 1.81.0 (`edition = "2021"`, `rust-version = "1.81"`).
- **Strict Quality Gate**:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets -- -D warnings` (Zero warnings)
  - `cargo test --all-targets` (All tests pass)
  - `cargo build --release`
- **Strict Constraints**:
  - **NO JS / NO NODE / NO V8**: Pure native Rust std executable and loopback server.
  - **NO JSON (`json=0`)**: Native `HBI-HBP-SHA-SH-HASH` format with external trust anchor (`.anchor`).
  - **NO FLOAT**: 100% discrete integer arithmetic ($\{-1, 0, +1\}$).
  - **Bit-Exact Reversibility**: 16-step forward + 16-step reverse Loschmidt test $\to$ 0 mismatches.
  - **Exact Invariants**: $\Delta M = 0$, $\Delta \vec{P} = \vec{0}$, $\Delta E_2 = 0$.

## Architecture & Substrates
- **Lattice Gas**: $L=24$, $V=13,824$ rooms, $K=27$ velocity directions, $N=373,248$ discrete Q-trits.
- **Collision Gates**: 288 involutive collision gates ($U = U^{-1}$, 18 closed $O_h$ orbits), 32 gates/room/tick ($107,495,424$ gate operations over 243 ticks).
- **HTTP-0 Portal**: Native Rust loopback server (`127.0.0.1:5088`) streaming uncompressed 24-bit BMP rainbow pixel frames ($288 \times 288$) directly to plain semantic HTML/CSS dashboard with **STRICTLY ZERO JAVASCRIPT**.
- **Virtual Namespace**: 23 billion virtual Q-trit coordinates sampled via $1,990,656$ coprimality stride probes.

## Running the Engine
```bash
# Execute canonical 243-step simulation and generate receipts
cargo run --release -- --run-243

# Verify trust anchor and receipt chain
cargo run --release -- --verify-anchor evidence/heb_c_rust_243.anchor evidence/heb_c_rust_243.hbp evidence/heb_c_rust_243.hbi

# Launch zero-JavaScript HTTP-0 Portal
cargo run --release -- --serve --port 5088

# Run against 300 GB physical dataset (Sector 000 Window 0)
cargo run --release -- --physical-input D:\ASOLARIA-HOST8-ROOMS-300G\sector-000.rooms
```

## Reports & Documentation
- [Canonical Rust HEB-C Report](CANONICAL_RUST_HEBC_REPORT.md)
- [Cross-Validation Report (Host8 C++ vs HEB-C Rust)](CROSS_VALIDATION_HOST8_HEBC_REPORT.md)
- [23B Virtual Portal Report](HEBC_23B_VIRTUAL_PORTAL_REPORT.md)
- Reference C++ Host8 Kernel: [finite-reversible-ternary-physics](https://github.com/JesseBrown1980/finite-reversible-ternary-physics)
