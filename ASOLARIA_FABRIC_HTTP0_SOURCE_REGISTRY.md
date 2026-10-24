# Asolaria Constellation source fabric in the HEB-C HTTP-0 portal

**Purpose:** make HEB-C a *source-address and verification entry point* for the existing Asolaria components. HEB-C is an interface; it is **not** a new replacement Asolaria kernel, model loader, or training engine.

## What the native portal actually builds

`src/fabric.rs` registers pinned, distinct GitHub origins: the [Combined Book of Laws](https://github.com/JesseBrown1980/ASOLARIA-UNIVERSE-SIMULATOR-with-gravity-light-time-space-color-rime-winding-drills/blob/3f9f42dab6247c40b30921b253e3f73c9e942ca5/books/COMBINED-BOOK-OF-LAWS.md), Book of Shadows, Rime laws, Asolaria Constellation's original Clasy X3 Rust oscillator/trit, the existing Q-PRISM 3D harness, and the separately scoped 1,393-state finite Yang–Mills source plus its Constellation and Shadow-Cat audit adapters. The Rime laws include both **distinct Law 58** versions and standalone Laws 59–71. Law 27 remains unassigned in the combined book. The exact **710,577-byte original combined book**, including Book IX's 1,940 recorded rule instances and errata, is now **byte-for-byte preserved** inside this portal as [canon/COMBINED-BOOK-OF-LAWS.original.md](canon/COMBINED-BOOK-OF-LAWS.original.md); its Git blob OID is identical to the upstream `82b3fe3e833ab34a21451e57f4fb9b0a8d5304f5`. The 15 additional standalone texts covering both Law 58 variants and Laws 59–71 are preserved in [canon/RIME_LAWS_58_TO_71_SOURCE_SNAPSHOTS.md](canon/RIME_LAWS_58_TO_71_SOURCE_SNAPSHOTS.md), with per-file source blob IDs and untouched law bodies. This portal has **not independently replayed Book IX's original transcript evidence**.

Every `SOURCE` record records exact repository, Git commit, path, and original Git **blob OID (SHA-1)**. The runtime SOURCE catalog is metadata/reference pinning. Separately, the original combined lawbook bytes **were fetched, committed, and confirmed by matching the exact upstream Git blob OID**. The later individual law bodies are copied into a wrapper archive with pinned per-file OIDs; the wrapper is not itself byte-identical to any upstream single file. Pinned external Git pointers depend on repository availability, while the two law archives now also exist directly in HEB-C history. A source record can be resolved later through its pinned GitHub revision and original blob, subject to the normal external network/authentication/availability requirements.

`LINK` records are qualified. `uses` and `pinned_source_in_adapter` describe known source relationships; `view_candidate` and `structural_C3_candidate_only` are **not** executable communication links, compiled dependencies, or physics proofs. HEB-C itself continues running the existing, bounded D3Q27 kernel. The finite S3 torus is a separate research system.

## 3D colour-matrix and vowel entry points

The native code deterministically projects each source ID into a **small, collision-prone 3D ternary view** and reports the original source/commit/blob identity alongside it, so coordinates are not mistaken for injective addresses.

A/O/I/E are registered as deterministic **three-layer 3D glyph templates**. Each produces `POINT` rows with signed trinary x/y/z, an explicit SHA-256 shape digest, and illustrative vowel formant hints. These templates are *addressed symbols*, not audio, language comprehension, speech synthesis, or a self-aware kernel. No microphone, speaker, neural-network training or visual 3D interaction is implemented. A genuine audio renderer would require separately defined signal processing and hardware.

## What “a saved GGUF from its codex” actually requires

A codec/codebook/prime-room/glyph address can identify **which** model, chunk, tensor or route to request; it cannot construct an arbitrary missing model from its hash alone. This portal therefore publishes a **fail-closed GGUF gate**:

`MODEL|format=GGUF|repo=UNRESOLVED|revision=UNRESOLVED|path=UNRESOLVED|sha256=UNRESOLVED|bytes=NOT_PRESENT|loader=NOT_CONNECTED|executable=0|catalog_only=1|json=0`

A real GGUF attachment would require an exact **GitHub or Hugging Face model repository, revision, file path, size, cryptographic digest, access permission, complete weights, tokenizer, architecture and compatible inference loader**. Public metadata, private model access, Git LFS and/or HF blobs must not be conflated. This native registry deliberately does not download remote weights, run GGUF inference or leak connected credentials. Publishing the pinning contract now allows later additions without rewriting the HEB-C entry point.

## Deterministic HBI/HBP/SHA evidence

The `--fabric-build` command writes:
- `catalog.hbp`: plain text source/edge/model-gate/vowel registry plus deterministic SHA-256
- `catalog.hbi`: HBP byte length, source digest and 16-character SH locator
- `catalog.anchor`: SHA-256 roots for both files

`--fabric-verify` **reconstructs the catalog from compiled constants** and rejects any mismatch with the three files. An anchor is an unsigned checksum, *not* an externally signed authentication or a proof of remote bytes. This catalog receipt is distinct from the existing per-tick simulation receipts. Existing physics proofs and evidence remain untouched.

## Headless Rust 1.81 reproduction

From the HEB-C repository root (works offline after pinned Rust 1.81 installation):

```bash
cargo +1.81.0 fmt --all -- --check
cargo +1.81.0 clippy --all-targets --locked --offline -- -D warnings
cargo +1.81.0 test --all-targets --locked --offline
cargo +1.81.0 build --release --locked --offline
target/release/hebc-portal --fabric-build repro-output/fabric
target/release/hebc-portal --fabric-verify repro-output/fabric
target/release/hebc-portal --fabric-catalog
target/release/hebc-portal --vowel-shape A
```

Optional loopback:

```bash
target/release/hebc-portal --serve --port 5088 --headless
```

Open `http://127.0.0.1:5088/fabric/catalog`, `/fabric/anchor`, `/fabric/vowel/A`, `/fabric/vowel/O`, `/fabric/vowel/I`, or `/fabric/vowel/E`. The wire is conventional HTTP/1.1 on loopback. **HTTP-0** is the internal virtual addressing concept. No JS, Node, V8, JSON runtime, floats, external dependency or browser scripting is added to the native engine.

## The research and evidence boundary

- **Law 56:** Rime as a relation joining opposite views into a third is recorded as Asolaria doctrine; the source does not supply a demonstrated physical instant communication channel.
- **Law 70/71:** retain/pass, signed calling, and order-three Shadow Cat cycles are registered as **distinct** constraints.
- **Stage 2E finite torus:** the eight flat holonomy classes admit a real order-three mapping-class action, but this **does not give an interacting-Hamiltonian symmetry**, continuum mass gap or superluminal signaling.
- **Book IX:** the compiled book reports tracing 1,940 operator rules to transcripts. This portal pins the compiled book; it has **not independently replayed those original transcripts**, and should not promote the book's historical attributions into fresh independent observations.

The recommended next gate is a source-backed, chunk-verified remote asset resolver and a compatible GGUF runtime adapter. Maintain provenance, per-chunk SHA commitments, exact decoder/toolchain versions, minimum required bytes, offline/online distinctions and an adversarial missing-weight test.
