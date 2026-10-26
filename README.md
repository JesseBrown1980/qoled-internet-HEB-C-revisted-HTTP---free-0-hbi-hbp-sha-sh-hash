## Asolaria Constellation source fabric — pinned inside HTTP-0

**[Permanent native source registry, laws, 3D matrix and model boundary](ASOLARIA_FABRIC_HTTP0_SOURCE_REGISTRY.md)** · **[Rust registry implementation](src/fabric.rs)**

**[Original IX/LX dictionary and chain language](canon/IX_LX_GLYPH_DICTIONARY_ORIGINAL.md)** · **[Byte-identical original Books I–IX](canon/COMBINED-BOOK-OF-LAWS.original.md)** · **[Original standalone Law 58–71 bodies](canon/RIME_LAWS_58_TO_71_SOURCE_SNAPSHOTS.md)** · **[Verified source contract](ASOLARIA_FABRIC_HTTP0_SOURCE_REGISTRY.md)**

The HTTP-0 portal now indexes the existing Asolaria law corpus, original Constellation Rust oscillator/trit source, Q-PRISM 3D harness, and Yang–Mills research via **full Git commit + Git blob pins**. It includes reproducible HBI/HBP/SHA catalog receipts, a deterministic 3D trinary source view, and A/O/I/E vowel *glyph templates*. These are addresses and shapes. The existing D3Q27 calculation is unchanged.

Build catalog offline: `cargo +1.81.0 run --release --locked --offline -- --fabric-build repro-output/fabric`; independently replay: `--fabric-verify repro-output/fabric`; view `--fabric-catalog` or `--vowel-shape A`. Optional loopback `/fabric/catalog`, `/fabric/anchor`, `/fabric/vowel/A` (O/I/E similarly).

# HEB-C — Canonical Rust 1.81 Native Kernel and Self-Contained Reproduction

**[One-checkout complete reproduction](REPRODUCE_NATIVE_RUST_181.md)** · **[Finite S3 Yang–Mills Stage 2 reference included](research/yang_mills_stage2/README.md)** · **[GitHub native CI](.github/workflows/ci.yml)**

Both fully independent Rust-native crates are included in this repository. The Host8 300 GB source volume is **not required** for the bounded window regression: the exact 110,592-byte canonical physical test block is regenerated locally, matched to its pinned SHA-256, and replayed for all 244 states. The Stage 2 Yang–Mills crate includes the finite Gauss-law state counting, exact integer Hamiltonian spectral certification and 512×512 encoded-Q-trit stress tests.

One-command Bash/WSL reproduction: `bash scripts/reproduce_native.sh`. Windows PowerShell: `pwsh -File scripts/reproduce_native.ps1`. These scripts use pinned Rust 1.81 with `--offline --locked`, Clippy `-D warnings`, integer-only kernels, no Node/V8/JavaScript, and no native JSON serialization.

**Headless is the default.** The screen/BMP adapter is optional. **23 billion** is a *virtual address capacity*, not a count of simultaneously instantiated kernels or particles. The HBI/HBP SHA hash chain authenticates its bytes relative to an anchor; the unkeyed anchor checksum is **not** a third-party cryptographic signature.



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
cargo +1.81.0 run --release --locked --offline -- --run-243 --headless

# Verify trust anchor and receipt chain
cargo +1.81.0 run --release --locked --offline -- --verify-anchor evidence/heb_c_rust_243.anchor evidence/heb_c_rust_243.hbp evidence/heb_c_rust_243.hbi

# Launch zero-JavaScript HTTP-0 Portal
cargo +1.81.0 run --release --locked --offline -- --serve --port 5088 --dashboard

# Run against 300 GB physical dataset (Sector 000 Window 0)
cargo run --release -- --physical-input D:\ASOLARIA-HOST8-ROOMS-300G\sector-000.rooms
```

## Reports & Documentation
- [Canonical Rust HEB-C Report](CANONICAL_RUST_HEBC_REPORT.md)
- [Cross-Validation Report (Host8 C++ vs HEB-C Rust)](CROSS_VALIDATION_HOST8_HEBC_REPORT.md)
- [23B Virtual Portal Report](HEBC_23B_VIRTUAL_PORTAL_REPORT.md)
- Reference C++ Host8 Kernel: [finite-reversible-ternary-physics](https://github.com/JesseBrown1980/finite-reversible-ternary-physics)

## Self-contained Host8 physical fixture and strict verifier

- [Integer Host8 window generator](src/host8_fixture.rs) · [Native per-step state replay](tests/native_physical_replay.rs) · [Full HBP/HBI SHA chain audit](src/audit.rs)
- [Full native reproduction guide](REPRODUCE_NATIVE_RUST_181.md) · [Automated cross-platform CI](.github/workflows/ci.yml)
- [Vendored, separately scoped finite S3 Stage 2 crate](research/yang_mills_stage2/) (pinned to upstream commit `276f776c80b7c22c49811ec5d6972c588fcec75e`; Rustfmt normalized)

Prior JSON and browser benchmark files are historical evidence. The active engine, tests and receipts are Rust-native and `json=0`. The finite S3 graph and single-plaquette gaps do not establish continuum 4D Yang–Mills existence or mass gap.
