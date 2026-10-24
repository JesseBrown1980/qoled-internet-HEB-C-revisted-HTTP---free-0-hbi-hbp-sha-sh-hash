#!/usr/bin/env bash
# Reproduce HEB-C and the separately scoped finite S3 research, Rust 1.81.
set -euo pipefail
cd "$(dirname "$0")/.."
root="$PWD"
out="repro-output"
mkdir -p "$out"/fixture "$out"/synthetic "$out"/physical "$out"/synthetic-again "$out"/physical-again
cargo +1.81.0 fmt --all -- --check
cargo +1.81.0 clippy --all-targets --locked --offline -- -D warnings
cargo +1.81.0 test --all-targets --release --locked --offline
cargo +1.81.0 build --release --locked --offline
bin="$root/target/release/hebc-portal"
"$bin" --write-host8-window "$out/fixture/sector-000-window0.rooms"
echo "2dfe7d757ecb012475806b86fcdefc602680d042db836ffbcb2bfe924f2a18ee  $out/fixture/sector-000-window0.rooms" | sha256sum -c -
"$bin" --run-243 --headless --hbp "$out/synthetic/run.hbp" --hbi "$out/synthetic/run.hbi" --anchor "$out/synthetic/run.anchor" > "$out/synthetic/stdout.hbp"
"$bin" --verify-anchor "$out/synthetic/run.anchor" "$out/synthetic/run.hbp" "$out/synthetic/run.hbi"
grep -q '42fb223a755aef7c4bbefbef87af830d1e19ed2fc64de018d9f412493f41b475' "$out/synthetic/stdout.hbp"
"$bin" --run-243 --headless --physical-input "$out/fixture/sector-000-window0.rooms" --expect-input-sha 2dfe7d757ecb012475806b86fcdefc602680d042db836ffbcb2bfe924f2a18ee --hbp "$out/physical/run.hbp" --hbi "$out/physical/run.hbi" --anchor "$out/physical/run.anchor" > "$out/physical/stdout.hbp"
"$bin" --verify-anchor "$out/physical/run.anchor" "$out/physical/run.hbp" "$out/physical/run.hbi"
grep -q 'd33eaf8d8d156377957a42f8536e667b46ce6f22b3cb0b0256147ea74d246426' "$out/physical/stdout.hbp"
"$bin" --run-243 --headless --hbp "$out/synthetic-again/run.hbp" --hbi "$out/synthetic-again/run.hbi" --anchor "$out/synthetic-again/run.anchor" >/dev/null
"$bin" --run-243 --headless --physical-input "$out/fixture/sector-000-window0.rooms" --hbp "$out/physical-again/run.hbp" --hbi "$out/physical-again/run.hbi" --anchor "$out/physical-again/run.anchor" >/dev/null
for mode in synthetic physical; do
  for filename in run.hbp run.hbi run.anchor; do
    cmp "$out/$mode/$filename" "$out/$mode-again/$filename"
  done
done
echo "PASS: native HEB-C 244-state exact physical replay, deterministic receipts"
cd research/yang_mills_stage2
cargo +1.81.0 fmt --all -- --check
cargo +1.81.0 test --all-targets --locked --offline
cargo +1.81.0 clippy --all-targets --locked --offline -- -D warnings
cargo +1.81.0 build --release --locked --offline
target/release/hebc sector single | grep 'physical_hilbert_dim=3'
target/release/hebc sector double | grep 'physical_hilbert_dim=11'
target/release/hebc sector torus2 | grep 'physical_hilbert_dim=1393'
target/release/hebc spectrum 1 1 | grep 'positive=1'
target/release/hebc spectrum 0 0 | grep 'positive=0'
target/release/hebc run 3 512 0 "$root/$out/ym-stress-512"
target/release/hebc verify "$root/$out/ym-stress-512"
echo "PASS: pinned finite Yang-Mills Stage 2 spectral and one-million-encoded-Q-trit research"
