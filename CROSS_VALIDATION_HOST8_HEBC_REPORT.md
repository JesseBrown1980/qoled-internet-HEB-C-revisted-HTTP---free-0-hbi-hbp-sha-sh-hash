# Cross-Validation: Host8 Stage G Physical Substrate & HEB-C Browser Portal

**Date**: October 24, 2026  
**Repositories**:
- Host8 Physics Substrate: [`finite-reversible-ternary-physics`](https://github.com/JesseBrown1980/finite-reversible-ternary-physics) (Commit `7d6a64b`)
- HEB-C HTTP-0 Portal: [`qoled-internet-HEB-C-revisted-HTTP---free-0-hbi-hbp-sha-sh-hash`](https://github.com/JesseBrown1980/qoled-internet-HEB-C-revisted-HTTP---free-0-hbi-hbp-sha-sh-hash)  
**Verification Status**: **100% VERIFIED PASS** across both physical and browser substrates.

---

## 1. Dual-Substrate Architecture

The Asolaria ecosystem implements two complementary execution environments for reversible discrete ternary physics:

| Architectural Dimension | Host8 Stage G Substrate (C++) | HEB-C Browser Portal (JS/Chromium) |
|---|---|---|
| **Primary Execution Path** | Native C++17 (`asolaria_equilibrium_suite.exe`) | Self-stepping browser webpage (`index.html`) |
| **Physics Kernel** | Involutive D3Q27 ternary collision & streaming | Involutive D3Q27 ternary collision & streaming |
| **Reversible Gates** | 288 conserving gates (18 closed $O_h$ orbits) | 288 conserving gates (18 closed $O_h$ orbits) |
| **Arithmetic** | Pure integer (0 float ops in state updates) | Pure integer (0 float ops in state updates) |
| **Physical Data Source** | `D:\ASOLARIA-HOST8-ROOMS-300G` (375 Sectors, 300 GB) | Synthetic deterministic room stubs (`seed=0x4d5954484f533831`) |
| **Scale Tested** | 1,005 Windows (13,824,000 rooms, 373,248,000 trits) | 243 Steps (3,359,232 rooms, 90,699,264 trits) |
| **Addressing Scope** | Physical byte-exact 8-byte room records | 23,000,000,000 virtual HTTP-0 address namespace |
| **Receipt System** | Schema v2 `HBI-HBP-SHA-SH-HASH` + `.anchor` | Indexed 243-step HBP trace + HBI index |
| **Network & Cost** | 100% Localhost / Offline ($0 free) | 100% Localhost / Offline ($0 free) |

---

## 2. Key Experimental Findings

### A. Physics Conservation & Invertibility
- **Integer Conservation**: In both C++ and JavaScript implementations, total mass ($M$), linear momentum ($P_x, P_y, P_z$), and kinetic energy norm ($E_2$) remain bit-for-bit invariant across all steps ($\Delta M = 0$, $\Delta P = 0$, $\Delta E_2 = 0$).
- **Loschmidt Reversibility**: A separate 16-forward + 16-reverse step check confirmed **zero bit mismatches** in both environments.
- **State Determinism**: The browser run produced exact identical final state digest:
  $$\text{SHA-256} = \texttt{42fb223a755aef7c4bbefbef87af830d1e19ed2fc64de018d9f412493f41b475}$$
  across consecutive runs with and without virtual portal probing.

### B. Virtual Address Probing (23-Billion Namespace)
- **Compact Addressing**: $23 \times 10^9$ virtual HTTP-0 address slots addressed arithmetically via:
  $$\text{virtual\_id} = (\text{ordinal} \times 1,000,003) \pmod{23,000,000,000}$$
- **Coprimality Guarantee**: $\gcd(1,000,003, 23,000,000,000) = 1$, ensuring collision-free injection over sampled range.
- **Probes Executed**: 1,990,656 distinct addresses sampled ($8,192 \text{ probes/step} \times 243 \text{ steps}$) in 70.5 ms ($28,236,255 \text{ probes/s}$).
- **Zero Real Overhead**: 0 live HTTP requests, 0 WebRTC connections allocated ($0 cost).

### C. Local $0-Cost Portals Integration
The browser portal and C++ runner coexist seamlessly with the local offline server infrastructure:
- `http://127.0.0.1:5088/`: `host8-serve-rust` (Fable 5 backend lens, PID 5464)
- `http://127.0.0.1:4949/`: `super-dashboard` (Live HBI stream & raw projection)
- `http://127.0.0.1:4792/`: `gnn-oracle`
- `http://127.0.0.1:4796/`: `recall-serve-rust`
- `http://127.0.0.1:4947/`: `behcs-bus`

---

## 3. Cryptographic Evidence Index

| File | Size | SHA-256 Checksum |
|---|---:|---|
| `baseline/heb_c_243_measurement.json` | 2,299 | `fcb9e7102e3b26c6d09c6eb162d1a3b118b628a8d56b0606f716611388b144fa` |
| `baseline/heb_c_243_trace.hbp` | 40,673 | `537e3638c95f68bbfa5133a1592555624c4b7e1a6d8125b9deab8ef34185dc0e` |
| `baseline/heb_c_243_index.hbi` | 16,103 | `ec24e369718451a2264729c95b974fab0d7927e77bbb0a5c9d2f62d0cd9252a3` |
| `evidence/heb_c_23b_measurement.json` | 2,810 | `30bfba482770ccf7da78f82cde2ffc68ad4ee66e8bda867ff042f25c11219809` |
| `evidence/heb_c_23b_trace.hbp` | 57,736 | `bf3e3befe62a60b12e7de41957db1724ba826f552e364eace051d6406790374d` |
| `evidence/heb_c_23b_index.hbi` | 15,880 | `0805191b405003a87611c0bf7681a6502c696e9ee6943bf8f4acb672dd9fb4a2` |
