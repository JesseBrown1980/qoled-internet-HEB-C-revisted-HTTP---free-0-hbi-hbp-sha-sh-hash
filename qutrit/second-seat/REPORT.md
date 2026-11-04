# The second seat on the qutrit run, at `6114590`

Three lanes ran and none failed. Their reports follow verbatim, and each lane's findings, holds and row files are in the folders beside this one. Their scripts are held on this machine at `D:/safety-backups/opus55-qutrit-attack-20261104-tools.tar.gz`, 403 files, SHA-256 `34838ed17fbf39f2735836b4fe5441853bb45f2baf1fa17e5c778482a8b78806`, and their upload to the Hugging Face dataset is OPEN.

## The lattice and ladder code, and the second route

VERDICT: HOLD at 6114590. No number in the receipt was refuted. The holds cover what the checks reach: the lattice's second route reads branch A's input only, and the flip sites miss 4 channels.

**Holds** (17, all apply once each). On a separate tree with them applied, fmt, clippy `-D warnings` and `test --lib qutrit` pass, and the whole run is `cmp` equal to `RECEIPT.hbp`.
- **H1, high:** `second_route.py`: `lattice` reads all 3 branches, and a new `engine` route runs the ticks.
- **H2, high:** dispatch for the engine route.
- **H3, high:** usage line for the engine route.
- **H4, high:** `qutrit.yml` runs the engine route.
- **H5, high:** `SECOND-ROUTE.hbp` gets the new rows.
- **H6, high:** README lattice bullet gets its scope, and rows that hold for an identity map are named.
- **H7, medium:** check `the_ticks_moved_the_state`; prints no row while it holds.
- **H8, medium:** add `lattice_flip_sites`, spreading the drawn sites over all 27 channels.
- **H9, medium:** `lattice_flips` calls it.
- **H10, medium:** README row: 81 sites across all 27 channels.
- **H11, medium:** README control moves from 75 to 73, plus what the sum rule reads.
- **H12, medium:** `CONTROLS.hbp` `held=73`.
- **H13, low:** ladder row: distinct pairs 72, 74, 80 at powers 5, 6, 7.
- **H14, low:** light-cone row for 162 flips, with the 3,375 bound.
- **H15, low:** gate row adds the distinct-edge read.
- **H16, low:** window row: 4 to 7 labels compared per m.
- **H17, low:** collision gates are involutions, 15 swaps and 51 fixed codes.

**New measurements** (all MEASURED unless tagged):
1. **Formula holds.** DERIVED, and a gate-by-gate exact state vector on a 9-qutrit toy matches on 19,683 of 19,683 amplitudes. Wrong-order controls match only 6,561 to 10,935. `T_PHASE=[0,1,8]` is what the code uses.
2. **Graph is simple.** A sorted edge list gives 46,891,008 distinct edges and 0 self-loops, with 23,887,872 inside rooms. Ring and streaming edges cannot coincide. `neighbours()` differs from my list on 0 of 23,887,872 sites, and `phase_sum` on all ones reads 46,891,008.
3. **Map is a bijection.** 288 gates each have 4 distinct channels, and all are permutations, involutions and conserving. `replace_state` sets the whole state.
   - The reversal row reads less than it looks. With the ticks replaced by the identity, the run and the current route both exit 0. No-collide, no-stream, one-gate and stream-sign mutants do the same, with exactly 4 rows off the receipt.
   - **New engine route:** pure Python, no shared code, derives the 288 gates and matches the pin. It reproduces output SHA-256 for A, B and C.
   - On branch A, 31,455,566 of 84,934,656 gate applications change trits, and 15,908,063 of 23,887,872 channels differ from input to output. Identity and stream-sign mutants exit 1 on it.
4. **Flip sites miss channels.** The 81 sites hit 23 of 27 channels at every side; channels 12, 15, 19 and 20 have none.
   - Rewiring all 884,736 sites of channel 19 in `neighbours()` survives byte-identical. So does a one-site rewire on a trit-0 site.
   - A one-site `phase_sum` rewire moves branch B's row with no check failing.
   - The sum rule still reads 1 when every moving site's backward neighbour is replaced by forward, or every site's ring-back neighbour by ring-forward; only the flips catch those. The route alone catches `t_012`, `t_mod9_alias`, `cz_omega2` and `v_xz_lattice_only`.
   - With H8 and H9, channel 19's mutant exits 1 at 78 of 81, and receipt bytes stay the same.
5. **Second route is code-independent, input-side and sign-blind.** DERIVED: a channel's edge set is the same under v to -v. Sign mutants are byte-identical, and negating `make_velocities` passes the gate pin and the route. Only `px`, `py`, `pz`, the output SHA and the cone row move. A non-sign permutation is caught. Stream sign chooses U, and the formula holds for either.
6. **Light cone.** Radius 3 is right. 9,261 holds with slack: the tick order bounds it at 5³ × 27 = 3,375. Over 162 flips, 3 to 687 channels differ, median 278; 158 reach radius 3, 4 reach radius 2, none go outside. 69 reproduces at the receipt's site; the same site with +2 gives 120.
7. **Ladder.** Exhaustive at n = 3 and 9: 354,456 cases, 0 wrong. Nothing breaks at small n. Duplicates appear at powers 5 to 7 only.
8. **Platform.** Clean extracts on Windows msvc and WSL Linux gnu, both Rust 1.81.0, each `cmp` equal to the receipt, SHA-256 `c16c54f7…`. The hosted `ubuntu-24.04` qutrit job passes at head 6114590; that job holds the `cmp`.

**Could not check:**
- CANNOT_SEE: a state vector on the real 96³ wiring; the formula rests on the derivation and the toy.
- CANNOT_SEE: 32-bit and big-endian targets, not run.
- NOT RUN: the hosted workflow on the held tree, since nothing here pushes.
- NOT RUN: H7 against the identity mutant. Its effect is derived; the mutants live in a different tree.
- NAMED: PowerShell `>` may rewrite line endings in a local compare.
- Not asked: fabric, recall and owning seats.

`canon/books/LAW-34-BROWN-SCHRODINGER.md` showed as modified in the checkout's first status (line endings), and I left it alone.

**Files** in `C:/tmp/opus55/qutrit-lattice-20261104/`:
- `FINDINGS.txt`
- `HOLDS.txt`
- `mutants.hbp`
- `sites.txt`
- `probe_gates.hbp`
- `probe_edges.hbp`
- `probe_wiring.hbp`
- `probe_cone.hbp`
- `probe_ringbrute.hbp`
- `formula_toy.hbp`
- `engine_route.py`
- `second_route_proposed.py`
- `proposed_lattice.hbp`
- `proposed_engine.hbp`
- `fix.hbp`
- `clean_all_windows.hbp`
- `clean_all_linux.hbp`
- `applied/`

## The 16-qutrit state and the windows, by separate code

VERDICT: HOLD at 6114590. The hold is on text only. Every numeric row I could reach stands. H1 is one literal understatement, and the rest add scope.

HOLDS (full OLD/NEW blocks in `HOLDS.txt`, each OLD anchor occurs exactly once)
- H1 medium, README: "Within 3 × 10⁻¹⁵" is a floor; the true largest is 3.963 × 10⁻¹⁵.
- H2 medium, README: add the separate 16-qutrit route, amplitude accuracy and the run-and-return result.
- H3 low, README: state the address order, where the last trit is qutrit 0.
- H4 medium, README: state what the Porter–Thomas row can and cannot show, with the mutation results.
- H5 low, src/qutrit.rs: comment that `xeb_ppm` wraps above 2^24.
- H6 low, README: `xeb_ppm` is the ideal state's own second moment.
- H7 medium, README: window rows are conditional on the branch and omit rest_phase; 3ᵐp ≤ 3.
- H8 medium, README: the window statistic has the closed form 3/2 − ½(−1/3)ᵐ and tests only the chain and T.
- H9 low, README: second_route.py takes `fields` from the receipt.

NEW MEASUREMENTS (MEASURED unless tagged)
1. **Separate engine (2^50 and 2^62, no shared lines).**
   - ppm difference is 0 at all 12 cycles against the receipt. Cycle 3 reads 65.982442605, and cycle 40 reads 1.999463766.
   - Dp fractions are equal from cycle 2 to 40. At cycle 1 the state is flat, so that reading only shows rounding direction.
   - Amplitude rows, receipt minus my 2^62 run, in units of 2^-60: label 0 is (−6, −7), label 43046720 is (3, 6), labels 81–83 are 150 or less, labels 1 and 2 are about 1,345.
2. **Gates, all correct.**
   - F is within 1 ulp of the exact matrix on 2,000,000 inputs.
   - T^k and CZ3 have 0 ulp error on a 3^6 register, all strides and pairs.
   - All 24 edges appear, each 10 times over 40 cycles.
   - Stride and address order are consistent.
3. **Rounding.** The bias is real, small, and invisible to the norm row.
   - The `>> 1` floors give ±0.25 ulp per F gate; measured mean is +0.2504.
   - Later F gates gather it at labels 1 and 2, at 1,352 units. DERIVED prediction: 1,339.
   - Round-half-even drops those labels to at most 12 units, with state distance 9.3 × 10⁻¹⁴ either way and no statistic digit changed.
   - Run and inverse returns re − 1 = −24 units and im = −58 units, with leak l2 1.31 × 10⁻¹³, the size of a random walk over 2,872 gates.
   - The verbatim copy reproduces seal 84eccaaa.
4. **read_state.** No reported digit changes. `(sum_p2 >> 16) * d` wraps above 2^24: a basis state reads 9492289000000 against a true 43046721000000. The largest reported value is 65.98.
5. **Windows.**
   - My own trits and a full scan of 46,891,008 edges reproduce the fields and all five window state seals.
   - Full recomputation holds at m = 4, 8, 12, 16 (40 of 40 each); the receipt ran only m = 3.
   - Exact counts over 3^16 terms confirm the 1,166 (mine 1,167, at label 0). That error does not change with the halves.
   - 3/2 is DERIVED by hand. Fields cancel because Σ_y forces the replica sum ≡ 0 mod 3. The 12 non-pairing states vanish in triples. The 15 pairing states give (d,u) → (3d+12u, 3d+3u), with eigenvalues 9 and −3.
   - Proved exactly by M²v = 6Mv + 27v in Z[ζ]; brute force at m = 1–5 agrees. The transfer vector in second_route.py is sound.
   - The value is 2D/(D+1) at D = 3, so Dp ≤ 3 at every m.
6. **Mutations, 16 qutrits, cycle 40.** These still read 2.000 within 0.00066, with fractions in the predicted band:

   | Mutation | Reading |
   |---|---|
   | F† in place of F | 1.999984 |
   | real orthogonal gate in place of F | 2.000232 |
   | every T power 1 | 2.000406 |
   | diag(1, ζ, ζ²) in place of T | 2.000152 |
   | CZ table wrong at (2,2) | 2.000144 |
   | one edge dropped | 2.000653 |
   | wrong graph | 1.999624 |
   | one F missing | 1.999725 |

   - Moved on 16 qutrits: pattern 3 replaced by pattern 1 reads 4.05.
   - Moved on 3×4 only: Clifford powers 1.000, T in cycle 1 only 1.691, F never on one qutrit 6.00.
   - Spread is 3 × 10⁻⁴ (DERIVED; 3.6 × 10⁻⁴ sample), so the band [1.9, 2.1] is about 330 spreads.
   - The statistic shows scrambling, not which gates ran.

COULD NOT CHECK
- NOT RUN: exact Z[ζ] amplitudes on the 16-qutrit circuit; my reference is the 2^62 run.
- CANNOT_SEE: the fabric and recall. I did not query them; these checks are arithmetic on a frozen commit.
- NOT RUN: the lattice and ladder beyond branch A, hebc-portal itself (I ran verbatim copies of its functions), and the owning CI and clippy gates.
- The checkout shows LAW-34 modified (line endings), already present at my first read and not from me. HEAD is unchanged.

FILES (all under C:/tmp/opus55/qutrit-state-20261104/)
- `FINDINGS.txt`
- `HOLDS.txt`
- `src/own.rs`
- `src/bin/own16.rs`
- `src/bin/own_window.rs`
- `src/bin/engine_copy.rs`
- `src/bin/l2diff.rs`
- `src/engine_gates.rs`
- `compare_receipt.py`
- `window_exact.py`
- `window_labels.py`
- `holds_check.py`
- `out/` (raw rows)

## Every claim in the text against the rows

VERDICT: HOLD at 6114590. Every number in the text matches the rows (TOTAL_BAD=0), but the text needs changes. All 27 OLD blocks occur exactly once at the commit or in the PR body. No hold has number H7.

**HOLDS**

High
- H1, README top: "Any amplitude… 1.4 s" comes before the kind of circuit. New text says phase state plus permutation, with one input string for each output string.
- H2, README scope: states the cost, one pass over n trits, and that the count is the length of the string held. `NOT_RUN` becomes Open.
- H3, qutrit scope: same point, adding uniform sampling and ladder times of 527, 1,536 and 4,674 ms.
- H4, wrong wiring: "one ring edge" is false. The code drops the 0–1 edge in every room, 884,736 edges.
- H26, PR scope: same as H2.

Medium
- H5, H6: gate-count wording. The sum covers T and CZ3 only, Fourier enters as 3^(−n/2), and each read does 84,934,656 collision lookups.
- H8: the Python lattice route starts at branch A's input, and the output SHA has no route.
- H10: "1.0 × 10⁻¹⁵" is really 1.0113e-15, on 29 printed amplitudes. Also adds the missing window-norm row.
- H11: windows run before any engine tick, and the note on the pinned "alone" row goes beside it.
- H12: Willow wording, with `OPEN` and the rule fix.
- H13: `OPEN`, plus four unlisted open items (16-qutrit route, ticks and other branches, chosen b, HTTP-0 route).
- H14: hosted reproduction added.
- H22: workflow paths omit sha256, ym43, bridge43 and Cargo files.
- H23: pin the receipt SHA, because an engine and receipt changed together pass today.
- H27, H28: PR route wording and table row.

Low
- H9: "16 of 16" counts powers, and the top rung tests 81 sites.
- H15: disclose `QUTRIT_ALREADY_READ`.
- H16: "shares no code".
- H17: "no rounding".
- H18: Porter–Thomas spread and 25.36 qubits.
- H19: the heading promises memory figures the section lacks.
- H20: three read times.
- H21: step name.
- H24: compare SECOND-ROUTE rows.
- H25: the PR link lands on a login page.

**NEW MEASUREMENTS**
- **"Any" holds:** a patched copy read 3 drawn strings in 1452, 1520 and 1476 ms. Forward returned b byte-exact on 3 of 3, and a non-trit is rejected. The committed command line has no argument for b.
- **One-room edge drop:** rooms 100000 and 300000 exit 0 with every check passing. Rooms with a0·a1≠0, and room 0, exit 1. Share passing, DERIVED: about 5/9 of rooms, the 0 being on branch A.
- **All window amplitudes at m=3,4,8:** 6,669 pass, at most 26 units of 2⁻⁶⁰.
- **Window inputs:** fields and rest phase recomputed on my own wiring, 5 of 5 equal.
- **Ring flip sites:** 69, 73 and 80 distinct at powers 5 to 7.
- **Prediction not moved:** 42aacb7 to a9f3dbe has 9 insertions and 0 deletions. a9f3dbe to 6114590 leaves src, second_route and PREDICTION unchanged.
- **Prediction order:** a9f3dbe was committed 3 s before the run on one clock. Its push at 21:41:34Z precedes the estimated run start by about a second, and that margin is at clock resolution (local clock about 7 s ahead, one sample). The push is about 3 minutes before the run closed.
- **Hosted run 41919836095 on 6114590:** all steps pass, byte-equal in 112 s. The text never mentions this.
- **Porter–Thomas, DERIVED:** cycle-40 readings are within one standard deviation (537/681, 31/73, 19/52, 23/33 ppm).
- **Willow:** spec sheet opened, 103 qubits, depth 40, XEB 0.1%, iSWAP-like. "65 qutrits" is right by exact integers, and "different gate set" is right.

**PREDICTION ROWS WITHOUT A TABLE READING**
- Window norm: met, with H10 adding the row.
- 5-qutrit control: in the bullets, met.
- `qubit_equivalent` and the probability strings: DERIVED, with no reading of their own.
- `bad_wiring`: the row's name mismatches what ran.

**WORKFLOW**
Steps match their names except the prediction step, which checks a hash rather than push order (H21). It catches a changed receipt, a changed engine under the listed paths, and a passing bad-wiring run. Gaps are in H22, H23 and H24.

**COULD NOT CHECK**
- Any 96³ run before 18:41:40 outside the output folder: CANNOT_SEE. The commands are unlogged, and a 38 s gap would fit a 27 s lattice run.
- Full transfer-vector comparison at m=12 and m=16: CANNOT_SEE in budget.
- Willow's single-qubit gate set: the sheet leaves it unnamed.
- Peak RSS: unmeasured.
- A Python route for the engine's ticks, and a separate route for the 16-qutrit state, are outside this lane and open.
- I did not rerun `--qutrit all`. The hosted cmp is the owning check.
- H24's `cmp` on the hosted runner is untested. Locally the rows match with line endings normalized.

**FILES**
- C:/tmp/opus55/qutrit-text-20261104/FINDINGS.txt
- C:/tmp/opus55/qutrit-text-20261104/HOLDS.txt
- C:/tmp/opus55/qutrit-text-20261104/check_numbers.out
- C:/tmp/opus55/qutrit-text-20261104/check_holds.out
- C:/tmp/opus55/qutrit-text-20261104/fields.out
- C:/tmp/opus55/qutrit-text-20261104/anyb.out
- C:/tmp/opus55/qutrit-text-20261104/oneroom-100000.out (other rooms alongside)
- C:/tmp/opus55/qutrit-text-20261104/allamps-route.out
- C:/tmp/opus55/qutrit-text-20261104/scripts/ (patched source copy in repo/)

Sources: [Willow spec sheet](https://quantumai.google/static/site-assets/downloads/willow-spec-sheet.pdf)
