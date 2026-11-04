# Yang–Mills finite 3×4 certificates on the HTTP-0 3^16 address portal

**Source-only, fail-closed interoperability.** The implementation reads three original, SHA-256-pinned Yang–Mills HBP receipts from a fixed GitHub commit. It does not perform a new spectral calculation or convert the S₃ Hilbert space into the Clay or Taylor–Green models.

The existing 43,046,721-address (3^16) portal was implemented for three time steps of a finite Taylor–Green → Clay **81-momentum-observable** conversion. That earlier study occupies **729** sparse slots. We now reserve the fifth ternary digit as a distinct model domain and occupy **three** additional Yang–Mills certificate locators, for a combined **732** populated addresses. All **43,045,989** others remain explicitly unpopulated. The original 23-billion virtual portal remains unchanged.

## Registered address layout

The address is 16 unsigned ternary digits, where the lowest digit is index 0.

- Original Clay/Taylor–Green experimental leaves: fifth digit (index 4) is zero; 729 occupied positions in the previous time/arm/register layout.
- Yang–Mills evidence: fifth digit is **one**, upper eleven digits zero, lower four digits encode an evidence-class index 0,1,2.
- The three Yang–Mills addresses are **81, 82 and 83**.
- Source is [Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C](https://github.com/JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C) at exact commit **5517268b511d5f28e307b21fc53eede3200ed7b9**.

| Virtual address | Recorded source class | Exact statement retained |
|---|---|---|
| 81 | **PERRON** | t_cert = 94.070673 J_E; upper bound 94.070936 J_E; source **gap_verdict OPEN** |
| 82 | **FULLREBUILD** | 45,508,080 Hamiltonian rows reported recomputed; zero recorded mismatches |
| 83 | **ALLPAIRS** | 1,632,625,631 weighted pairs reported checked in four passes; zero recorded violations |

Proof metadata and the three source files are in the [Yang–Mills bridge document](https://github.com/JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C/blob/main/docs/HTTP0-3POW16-YANG-MILLS-BRIDGE.md). The source report remains an *attestation of past independent calculations*, not a fresh calculation at portal startup.

## Native, read-only HTTP-0 service

The Rust module [src/ym43.rs](../src/ym43.rs) checks the exact file sizes and SHA-256 digests of all three source receipts and validates important **original integer fields**, with the formal 3×4 gap verdict explicitly remaining **OPEN**. It never makes network requests or substitutes a synthetic receipt.

To serve values, set **HEB_C_YM43_ROOT** to a trusted checkout of the pinned Yang–Mills commit. Without that environment variable and exact evidence, the dynamic endpoints return **503**, not fabricated data.

| Route | Behavior |
|---|---|
| /bridge/43m/catalog | 43,046,721-address shared catalog, source classes and 732 occupied slots |
| /ym/43m/contract | Public Yang–Mills address contract |
| /ym/43m/availability | 200 for verified local source files, otherwise 503 |
| /ym/43m/address/81 | SHA-256-pinned Perron source record and OPEN flag |
| /ym/43m/address/82 | SHA-256-pinned full-row reconstruction record |
| /ym/43m/address/83 | SHA-256-pinned weighted all-pairs record |
| /bridge/43m/lookup/81 | Shared dispatcher for the same Yang–Mills address |
| /bridge/43m/address/81 | Original **Clay-only** route, intentionally 404 |
| Unpopulated / malformed / out-of-bounds | 404 / 400 / 400 |

Running locally with the HTTP-0 repo built for Rust 1.81:

~~~bash
cargo +1.81.0 run --release --locked --offline -- --ym43-verify /path/to/pinned/ym-repo
cargo +1.81.0 run --release --locked --offline -- --ym43-lookup /path/to/pinned/ym-repo 81
~~~

The [independent Yang–Mills Python verifier](https://github.com/JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C/blob/main/native/scripts/verify_http0_ym43.py) validates both SHA-256 **and** original Git blob SHA-1 with a separate implementation, checks the exact fields, and includes corruption and missing-file controls. Hosted [Rust integration CI](../.github/workflows/ym43.yml) independently checks out the frozen source commit and tests both CLI and HTTP paths.

**Status interpretation:** Source hash verified and field matching PASS means the portal read the exact recorded proof evidence. It does **not** imply a new independent Yang–Mills eigenvalue calculation, a new physics equivalence, operator approval of the 3×4 formal gap, or a continuum Yang–Mills solution.
