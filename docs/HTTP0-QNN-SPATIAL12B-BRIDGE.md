# HTTP-0 read-only bridge for QNN Spatial12B

This is a **read-only address adapter** linking the canonical native HEB-C/HTTP-0 portal to the [QNN Spatial12B implementation](https://github.com/JesseBrown1980/GGUF-files-operating-as-trained-quantum-neural-network-models/pull/21). The source QNN revision that supplied the first tested address contract is `6981cf54b1f289dabedac286108f4a5d4fa7f2f6`.

The established physical 24³/48³/96³ D3Q27 integer engine, its reversible collision gates, HBI/HBP/SHA receipts, and the previously available 23-billion virtual namespace remain unchanged.

## Contract

- Virtual QNN address range: `0..11999999999` (12 billion **potential** addresses).
- Room number `id/27`; channel `id%27`, x-y width 1024 and x-y-z coordinate decomposition from QNN Rust.
- 22 ternary digits per address, printed most-significant trit first.
- Reject missing top addresses, nonnumeric input, and the invalid tail of the final room. Read-only requests return **no node state**, because no QNN model is loaded into HTTP-0.
- No browser JavaScript, Node, floating point, JSON or external model fetch is required.
- This route is not a WebRTC connection, not an active neural training service, and not a simulated twelve-billion-qutrit quantum state.

## Commands

```sh
cargo +1.81.0 test --locked --lib qnn_spatial12b_bridge
cargo +1.81.0 run --locked --release -- --qnn-spatial12b-contract
cargo +1.81.0 run --locked --release -- --qnn-spatial12b-lookup 11999999999
```

When running the existing native loopback portal at 127.0.0.1:5088, these HTTP paths become available:

- `/qnn/spatial12b/contract` — address capacity and source metadata only.
- `/qnn/spatial12b/address/11999999999` — exact virtual lookup with x=796, y=875, z=423, channel=11, ternary path `1010222022010011100102`.
- `/qnn/spatial12b/address/12000000000` — HTTP 400, out of bounds.

**Cross-repo independent fixture:** Both the QNN and HEB-C Rust implementations calculate the same final address and 22-trit path. This is independent agreement for the mapping, **not** an independent simulation of neural behavior or particle dynamics.

The QNN code owns active sparse node state, its 27-coefficient generated neural weight matrices, inference, and CPU benchmarks. This HTTP-0 repo owns only a validated address route.

Further integration must pin a real model or inference source and provide authenticated state/weight receipts before an endpoint may return model outputs instead of `state=UNAVAILABLE`.
