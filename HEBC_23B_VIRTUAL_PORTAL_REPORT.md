# HEB-C — 23-billion-virtual-portal address benchmark

**Date:** October 24, 2026. **Run:** headless Chromium in a container. **Reference:** Host8 C++ D3Q27 kernel commit [`53f9432`](https://github.com/JesseBrown1980/finite-reversible-ternary-physics/commit/53f9432d112c7b94cdf7066d07cda36bc56879f8).

## What was tested

A standalone, self-stepping browser webpage applied the C++ reference's 288 reversible collision gates to a 24³ × 27-trit lattice, streamed with periodic boundaries, checked exact integer invariants, and stopped at 243 forward transitions. Separately it performed 16 forward + 16 inverse steps to confirm reversibility. The initial 13,824 source-room words were **generated**, not read from the physical 300 GB drive.

The webpage also exposed **23,000,000,000 virtual HTTP-0 address identifiers** (from 0 through 22,999,999,999). These are **potential logical slots** addressed through arithmetic without allocating individual portal objects. Every physics transition performed 8,192 actual deterministic, collision-free sampled address probes. This does **not** instantiate or run 23 billion network connections, HTTP servers, QLED pixel processors, or neural models. No real HTTP-0 transport was exercised. The rainbow pixels display an observer projection only.

## Measured results

| Metric | Result |
|---|---:|
| Available virtual addresses | 23,000,000,000 |
| Real portal objects allocated | 0 |
| Executed address probes | 1,990,656 |
| Distinct address probes | 1,990,656 (mathematically unique) |
| Probes per physics step | 8,192 |
| Address probe rate | 28,236,255/s |
| Routing measured time | 70.500 ms |
| End-to-end webpage run | 3110.100 ms |
| Physics kernel/checks time | 2624.300 ms |
| Completed steps | 243 / 243 |
| Room-ticks | 3,359,232 |
| Trit-ticks | 90,699,264 |
| Collision-gate applications | 107,495,424 |
| Integer invariant deviations | 0 |
| Reversal mismatches | 0 |
| State SHA-256 | `42fb223a755aef7c4bbefbef87af830d1e19ed2fc64de018d9f412493f41b475` |
| Actual HTTP requests | 0 |
| Real WebRTC connections | 0 |

The final state digest matches the previously measured 243-step browser run: `42fb223a755aef7c4bbefbef87af830d1e19ed2fc64de018d9f412493f41b475`. This verifies same synthetic inputs and output state in the browser port; it is **not** a complete independent replay from physical Host8 sectors.

## Address algorithm

```text
ordinal = tick * 8192 + probe_index
virtual_id = (ordinal * 1000003) % 23000000000
room = virtual_id % 13824
channel = floor(virtual_id / 13824) % 27
trit = state[room * 27 + channel]
```

Here `gcd(1000003, 23000000000) = 1` and total probes = 1,990,656 (less than 23B). The mapping is injective over the sampled ordinals; it is implemented with JavaScript safe integers, not 23B allocated slots. The 32-bit route fingerprint is a noncryptographic accumulator, not SHA-256 or a signed source authentication.

## Files and SHA-256

| File | Bytes | SHA-256 |
|---|---:|---|
| `index.html` | 26,799 | `38e6d57c3e20f8a3df4d3893b3eb2e773a6902b96bb1bff6656c387beb0903f9` |
| `evidence/heb_c_23b_measurement.json` | 2,810 | `30bfba482770ccf7da78f82cde2ffc68ad4ee66e8bda867ff042f25c11219809` |
| `evidence/heb_c_23b_trace.hbp` | 57,736 | `bf3e3befe62a60b12e7de41957db1724ba826f552e364eace051d6406790374d` |
| `evidence/heb_c_23b_index.hbi` | 15,880 | `0805191b405003a87611c0bf7681a6502c696e9ee6943bf8f4acb672dd9fb4a2` |
| `evidence/heb_c_23b_screen.png` | 814,015 | `9fda60d2075e3252c7d7ed3fbb9c776dcf361f460e532b4180d048e5ec8b4201` |

## Validation and scope

- `python tests/verify_evidence.py` validates the published measurements, 243 HBI byte offsets and HBP slices, conservation fields, source SHA-256 value and whole-file digests.
- Measurement duration is observer time via `performance.now()`. Step progression is scheduled via `MessageChannel`, **not** by a fixed interval timer. This does not make execution free of clock time or hardware resource costs.
- The HBP/HBI trace here is a **prototype** (per-row FNV32 plus independently hashed whole files). It is *not* the canonical five-part Host8 HBI/HBP/SHA/SH/HASH trust chain and carries no independent physical-source attestation.
- No GGUF decoding, HRM inference, QR camera processing, QLED optical storage or physical HTTP-0 peer-to-peer channel was benchmarked.
- The physical Host8 run and this browser test use different workloads: 48+16+16 per-source-window compared to 243 consecutive transitions of one synthetic lattice. The reported browser room-ticks/s cannot be divided by the Host8 windows/s to assert a scientifically valid speedup.

## Next experiment

Implement a real HTTP-0/WebRTC peer-to-peer link and send authenticated HBP/HBI state slices between two independently instrumented browser instances; benchmark actual messages, bytes transferred, end-to-end latency, errors and exactly-matched decoded state. Separately compare the same **physical room input** and identical D3Q27 trajectory between the browser and the Host8 implementation.