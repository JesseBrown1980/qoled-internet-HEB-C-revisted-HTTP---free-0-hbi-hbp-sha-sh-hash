# Reproduce HEB-C and finite S3 Yang–Mills Stage 2 from **one checkout**

This repo contains **both native Rust 1.81 crates** and every source/recipe needed for the accepted finite experiments. No 300 GB disk, GPU, screen, browser, JavaScript, Node, V8, JSON serializer, network server, or external library is needed for the **actual calculations**. The optional pixel/HTTP adapter is entirely separate from the headless physics loop.

## Requirements

- Rustup with **Rust 1.81.0** and the `rustfmt` and `clippy` components; offline Cargo dependencies thereafter (both crates depend on the Rust standard library only).
- Windows PowerShell or Bash (Linux/WSL/macOS), plus ordinary `sha256sum` on Bash platforms.
- For CI, GitHub Actions runners and checkout/upload actions access the network, but **native executables do not**.

```bash
git clone https://github.com/JesseBrown1980/qoled-internet-HEB-C-revisted-HTTP---free-0-hbi-hbp-sha-sh-hash.git
cd qoled-internet-HEB-C-revisted-HTTP---free-0-hbi-hbp-sha-sh-hash
bash scripts/reproduce_native.sh
```

Windows PowerShell (from the repository directory):

```powershell
pwsh -File scripts/reproduce_native.ps1
```

Alternative Windows built-in PowerShell 5.1: `powershell -ExecutionPolicy Bypass -File scripts/reproduce_native.ps1`.

## How the physical test reproduces without a physical disk

`src/host8_fixture.rs` uses a **published canonical integer room-identity codec**, instead of fabricating a random replacement for the Host8 input:

- 300 GB logical plane: 37,500,000,000 canonical rooms, but reconstruct **only** Sector 000 / Window 0, rooms 0..13,823 (13,824 × 8 bytes).
- Canonical seed `0x518bbb472a9db786`, signed address offset and bounded 64-bit reversible mixer `M1=0xbf58476d1ce4e5b9`, `M2=0x94d049bb133111eb`.
- Reconstructed source **110,592 bytes**, SHA-256 `2dfe7d757ecb012475806b86fcdefc602680d042db836ffbcb2bfe924f2a18ee`.
- Unpack exactly 27 base-three channels per room → initial-state SHA-256 `846f2b159a1efcf3b3b7c1d1a89ffb1ba839093b180d905e93c1ea377331077f`.
- `tests/native_physical_replay.rs` recomputes **all 244** SHA-256 states (tick 0..243), compares each against the committed `evidence/physical_sector000_w0000_243_ticks.csv` cross-executor reference, checks exact 5 integer invariants at each tick, and runs **243 reverse steps** to recover the starting state.
- Expected final SHA-256: `d33eaf8d8d156377957a42f8536e667b46ce6f22b3cb0b0256147ea74d246426`.
- Independently generated synthetic-source 243-step final SHA-256: `42fb223a755aef7c4bbefbef87af830d1e19ed2fc64de018d9f412493f41b475`.

These tests reproduce an *identical software input and trajectory*, not an independent observation of the original physical disk or physics beyond the mathematical model.

## Native HBI/HBP/SHA/SH/HASH strict audit

- `src/audit.rs` validates the trust-anchor root digests **and** every individual HBP canonical payload SHA, predecessor HASH, successor HASH, and HBI byte offset, row length, row digest, source SHA/SH agreement, exact tick ordering, virtual scope and probe count.
- `--verify-anchor` invokes the **strict** audit instead of checking root hashes alone.
- `--headless` is the default; no `index.html` rewriting unless `--dashboard` is requested explicitly.
- All source/receipt digests use SHA-256, and native wire records use `json=0` text fields, not JSON.
- The **anchor checksum is an unkeyed SHA-256 commitment, not a digital signature**. External provenance requires independent publishing/signing of its SHA digest; a fully malicious rewriter of every record and the unpinned root could create a consistent new chain.
- Python scripts and historical JSON evidence retained in `baseline/` and `evidence/` are preserved for historical comparison only; **no JSON or Python is executed by the native reproducer**.

Headless manual execution after `cargo +1.81.0 build --release --locked --offline`:

```bash
target/release/hebc-portal --write-host8-window repro-output/fixture/sector-000-window0.rooms
target/release/hebc-portal --run-243 --headless --physical-input repro-output/fixture/sector-000-window0.rooms --expect-input-sha 2dfe7d757ecb012475806b86fcdefc602680d042db836ffbcb2bfe924f2a18ee --hbp repro-output/physical/run.hbp --hbi repro-output/physical/run.hbi --anchor repro-output/physical/run.anchor
target/release/hebc-portal --verify-anchor repro-output/physical/run.anchor repro-output/physical/run.hbp repro-output/physical/run.hbi
```

## Embedded finite Yang–Mills Stage 2 — independently scoped research crate

`research/yang_mills_stage2/` is a **source snapshot** of `JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C`, pinned to upstream commit `276f776c80b7c22c49811ec5d6972c588fcec75e`; source formatting is normalized to Rust 1.81. It has its own Cargo.toml, Cargo.lock, rust-toolchain.toml and Rust library/binary source. See `research/yang_mills_stage2/README.md`.

The following do *not* share a physics kernel with the Host8 D3Q27 calculation:

1. A **finite S3 single-plaquette** gauge-invariant Hilbert space with dimension 3, and a symmetric integer interacting 3×3 Hamiltonian with electric coupling `J_E` and magnetic coupling `J_B`.
2. Exact integer Sturm eigenvalue rational bounds (DENOM=65536), plus independent characteristic-polynomial tests. At `J_E=J_B=1` the excitation gap is in `[727730,727732]/65536` lattice energy units.
3. Gauge-invariant Hilbert-space *dimension counts* 11 (double open plaquette) and 1,393 (periodic 2×2 torus). Full interacting Hamiltonian spectra for those larger spaces have **not** been computed here.
4. Classical gauge-equivariant 512×512 stress case with 524,288 link labels / 1,048,576 **encoded** balanced Q-trits, exact replay and HBP/HBI receipts. **Not** 1,048,576 coherent quantum qutrits or particles.

The finite S3 gap does not solve the Clay Millennium four-dimensional continuum Yang–Mills mass-gap conjecture. Virtual 23-billion-ID addressing does not imply 23 billion materialized kernels, real HTTP requests or free physics computation.

## CI and proof boundary

`.github/workflows/ci.yml` runs the **self-contained native D3Q27 reproducer** and the independently built **vendored S3 Stage 2 research crate**, including `cargo fmt`, `cargo clippy -- -D warnings`, release build, unit tests, exact 244-state replay, full receipt-chain checks, attack controls and large encoded Q-trit lattice stress. Evidence is retained as downloadable GitHub Actions artifacts.

**Acceptance criterion:** a green run on main for both crates on the pinned Rust 1.81 toolchain with `--locked --offline`. Do not count the older browser/JS and historical JSON experiments as canonical native tests.
