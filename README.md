# HEB-C — virtual HTTP-0 address space and ternary browser physics

**[Launch the standalone webpage](index.html)**. It automatically runs 243 D3Q27 ternary physics transitions with 288 reversible collision gates, checks five integer invariants, runs a separate 16+16 reverse check, samples 8,192 addresses per transition from a **23-billion-ID virtual namespace**, renders a seven-band rainbow projection, measures performance and stops automatically.

The 23 billion are **addressable labels, not 23 billion simultaneously executing HTTP instances**. The measured workload executes **1,990,656 real virtual-address probes**, **zero real network requests**, and **zero WebRTC connections**. It uses synthetic room inputs and does not perform GGUF, HRM or QR decoding.

- [Full results and limitations](HEBC_23B_VIRTUAL_PORTAL_REPORT.md)
- [Cross-validation report (Host8 C++ vs Browser)](CROSS_VALIDATION_HOST8_HEBC_REPORT.md)
- [243-step measurement](evidence/heb_c_23b_measurement.json)
- [HBP receipts](evidence/heb_c_23b_trace.hbp) / [HBI index](evidence/heb_c_23b_index.hbi)
- [Original no-portal 243-step baseline](baseline/heb_c_243_measurement.json)

For static evidence verification run `python tests/verify_evidence.py`. For a new browser run, open `index.html`, let it reach STOPPED 243 / 243, and export fresh receipts using the webpage buttons. Timings depend on the machine.

Reference kernel: [Asolaria Host8 D3Q27](https://github.com/JesseBrown1980/finite-reversible-ternary-physics/blob/main/asolaria_equilibrium_suite.cpp) (Stage G Commit [`7d6a64b`](https://github.com/JesseBrown1980/finite-reversible-ternary-physics/commit/7d6a64bf50529d297ff017f94e9fce0ca3aa9a86)).
