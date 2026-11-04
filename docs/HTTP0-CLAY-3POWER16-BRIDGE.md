# HTTP-0: 3^16 virtual addresses for three Taylor–Green → Clay leaves

**Capacity:** 3^16 = **43,046,721** (the first power of 3 above 23 million). This is an **address namespace**, not 43 million simulated states, particles, independent shell modes, or NTT coefficients. It is **separate** from the portal's existing **23-billion** virtual Q-trit namespace.

**Pinned experiment:** [the original Taylor–Green → Clay three-step computational bridge](https://github.com/JesseBrown1980/continuum-Clay-variation/tree/36b304006adfecff13df6ed1098c9ed30433bc3b/bridge/taylor-green), built from [the original Taylor–Green source data](https://github.com/JesseBrown1980/Taylor_green_regularity-resolved-and-solved/tree/ee71e0eea7fd61bfe5a45ebf2f590311742161f7/results/2026-10-30-noise-test). Taylor–Green computes 24³×27 microscopic trits; the **81 projected signed block-momentum observables** remain the actual information being indexed.

## Exact ternary layout

Each address is an integer in [0, 43,046,721), written as 16 base-3 trits, high to low:

    [step trit][arm trit][0000000000][four register trits]

- Step trit: 0,1,2 → measurements after steps 1,2,3
- Arm trit: 0=TGV, 1=SCR (shuffled), 2=NULL
- Ten interior trits are **zero by contract**
- Four low trits: index 0..80, since 3^4=81

The address formula is

    id(t, arm, k) = (t - 1) * 3^15 + arm * 3^14 + k

for t=1,2,3; arm=0,1,2; k=0..80.

All **729** (3×3×81) occupied addresses are unique. The other **43,045,992** addresses are unpopulated and must not return synthetic values. A virtual address can be decoded and re-encoded exactly using 16 balanced/unsigned ternary digits; storage is sparse (9 × 81 values), not a huge 43M-element array. The use of trits here is an **address representation**, not a physics result.

## Integrity and evidence

The built-in read-only loader accepts the three *original* SHA-256-pinned HBP leaf files and the HBI index, verifies the fixed SHA-256 digests and predecessor chain, requires TGV, SCR, NULL records per step, validates all 81 signed-to-residue conversions and the global momentum correspondence, and rejects missing/corrupted/inconsistent files. No data are fetched or regenerated when evidence is unavailable.

| Leaf | Pinned SHA-256 |
|---|---|
| 1 | ee5036f22e94990de1992e4649302d603aa5a2891222b9333a512959c95b40be |
| 2 | 67c1ceb79d45761b68835910e0a05c76564785b77b487bff3fe6d5a49901b11e |
| 3 | c638926523a7b45e44e1396320a4f3a640c17633716e65824351a8a543819c9f |

**Important outcome of those computations:** the momentum-observable conversion is exact, but the nonlinear time-evolution update **does not commute** between the Taylor–Green gas and Clay shell map. All 81 TGV registers differed at each of the three steps. Increasing address capacity to 3^16 does not resolve that mismatch.

## Native Rust and HTTP-0 usage

The CI generates the original three leaves from pinned GitHub commits and then runs the independent HTTP-0 verifier.

~~~bash
cargo +1.81.0 run --release --locked --offline -- --bridge43-verify /path/to/three-leaf-receipts
cargo +1.81.0 run --release --locked --offline -- --bridge43-lookup /path/to/three-leaf-receipts 0
~~~

The existing local HTTP-0 Rust server offers these read-only routes:

| HTTP-0 route | Meaning |
|---|---|
| /bridge/43m/contract | 3^16 capacity, sparse occupancy and model provenance; always available |
| /bridge/43m/availability | 200 when pinned evidence was loaded, otherwise 503 |
| /bridge/43m/address/0 | TGV step 1 register 0; returns value only when evidence verified |
| /bridge/43m/address/14348907 | TGV step 2 register 0 |
| /bridge/43m/address/38263832 | NULL step 3 register 80 |
| Any unpopulated address | 404 with UNPOPULATED, not an invented value |
| Out-of-range or malformed address | 400 |

To make verified values available in the portal, set the local environment variable **HEB_C_CLAY_LEAVES_DIR** to the directory containing LEAF-1.hbp, LEAF-2.hbp, LEAF-3.hbp and LEAVES.hbi *before* starting the HTTP-0 server. Without the files, the contract remains visible and value lookup returns 503; the existing physics engine and its 23-billion address support are unchanged.

This extension adds no networking to the simulation or receipt loader, uses integer arithmetic, and never treats 43 million *addresses* as 43 million *measured coordinates*. Continuum Navier–Stokes correspondence remains OPEN.
