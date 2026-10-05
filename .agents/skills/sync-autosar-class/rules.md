# AUTOSAR Class Port Rules (Rust)

Self-contained rule reference for syncing any AUTOSAR model class in
rust-armodel to py-armodel parity. `ClassName` denotes the input class (the
sync usually covers its whole family):

- reference source: `target/py-armodel/src/armodel/parser/arxml_parser.py` and
  `…/writer/arxml_writer.py` (pinned checkout; the pin is
  `tools/py2rust/PY_ARMODEL_VERSION`), plus the py model's `# Spec:` checklist
  under `…/src/armodel/models/M2/AUTOSARTemplates/…`
- generated progress ledger: `docs/port_checklist.md` (py method → Rust name →
  ported → py location; Rule 0002)
- generated Rust model: `src/m2/**` (spec-package-derived paths,
  `docs/code_guide.md` §2; never hand-edited — Rule 0003)
- hand-written port surface: `src/parser/arxml_parser.rs`,
  `src/writer/arxml_writer.rs` (dispatch seams: `read_element_payload` /
  `write_ar_package_element`)
- correctness bar: the fixture harness — zero parser warnings +
  byte-identical written output + model equality over the 32 verbatim
  fixtures in `tests/integration/test_files/`

**IDs:** rules carry contiguous 4-digit IDs (`Rule 0001` …). Each notes the
py-armodel rule it adapts for traceability; py rules govern upstream model
work and are cited only as background. The 9-step workflow in `SKILL.md`
references these IDs.

**Source of truth:** the pinned py-armodel implementation. Its model classes
are upstream-stamped `# Spec verified:` — spec fidelity is inherited, not
re-established here. When py's behavior looks ambiguous, consult the py
model's `# Spec:` block first, then the spec corpus
(`target/py-armodel/autosar/R23-11/`; PDF page numbers via
`python3 .agents/skills/sync-autosar-class/pdf_page.py <ClassName>`). The
release is detected from the parsed file (`document.get_ar_release()`) — py's
`setARRelease` has no Rust counterpart.

---

## Rule 0001 — Py-Parity Port *(adapts py Rules 0001 / 0013)*

The Rust port must reflect the pinned py implementation for the class family:
same elements, same orders, same attribute handling, same warnings. One Rust
method per py method; no invented handling.

### 1.1 Element kind and dispatch arms

- A py `readXxx`/`writeXxx` for a class whose tag appears as an `AR-PACKAGE`
  element gets (a) the `read_xxx`/`write_xxx` methods, (b) an `ElementRef`
  arm in `read_element_payload` / `write_ar_package_element`, (c) tests. A
  nested tag (e.g. `COMPU` under `COMPU-INTERNAL-TO-PHYS`, `KEYWORD` under
  `KEYWORDS`) gets **no** arm — the parent's methods call the helper
  directly. Worked example: only `COMPU-METHOD`, `DATA-CONSTR`,
  `KEYWORD-SET`, and the datatypes got arms in the datatypes batch; `COMPU`,
  `COMPU-SCALES`, `LIMIT` did not.
- A spec table/element whose py handling is attribute-only on the parent
  (no own element) ports inside the parent's methods — no separate class
  work.

### 1.2 Helper chain leveling *(adapts py Rule 0013.1)*

A `read_xxx` helper models the class's own attribute level and calls the
payload helper of its **direct model base** — never re-reads an ancestor:

- `read_compu_method` calls `read_identifiable_payload` once (which owns
  LONG-NAME/DESC/INTRODUCTION/ADMIN-DATA) plus its own elements
  (DISPLAY-FORMAT, UNIT-REF, the two COMPUs).
- A deeper subclass reads only one level: its own payload helper (if any)
  plus the direct base's — never the grandbase's payload too.

Double base reads duplicate arena inserts and diverge from py's leveling.
The writer stays symmetric (`write_identifiable_attributes` /
`write_identifiable_parts` own the chain elements; the subtype emitter adds
only its own extras, in py's order). After any reader/writer edit, grep each
new helper's calls and verify the one-level rule by eye.

### 1.3 No fabrication, no silent extras

- Every new Rust method traces to a py method; every element a Rust method
  handles traces to the py body. Do not handle an element py does not; do
  not skip an element py handles without recording the deferral (Rule 0008).
- Port only the **model-backed subset** when py handles more than the
  generated Rust model carries — and record the remainder as a deferral
  (e.g. py's `setSwDataDefProps` writes more conditional children than the
  Rust model has; port the model-backed subset, list the rest).
- A needed-but-missing field is a model gap → Rule 0003, never a stub.
- Never route a value through a helper typed for a different model class
  (adapts py Rule 0013.2): the matched-pair rule holds — model accessor
  `set_x`/`get_x`, reader `read_x`/writer `write_x` carry the **same** name
  suffix on both sides; a cross pair (`set_x1` ↔ `get_x2`) is wrong and is
  fixed by renaming, not documented.

### 1.4 Two independent orders — never merge them *(adapts py Rule 0001.11)*

1. **Reader order** is find-based and wrapper-relative: extract it from the
   py **reader** body (which children it `find`s, in which order, which are
   optional).
2. **Writer order** is py's emit order (the `sequenceOffset` order py's
   writer encodes): extract it from the py **writer** body.

They legitimately differ. Worked example (datatypes batch): the DataConstr
rule reader reads CONSTR-LEVEL, **INTERNAL-CONSTRS, PHYS-CONSTRS**; the
writer emits CONSTR-LEVEL, **PHYS-CONSTRS, INTERNAL-CONSTRS**. Port each
side's own order exactly — "fixing" one to match the other breaks
byte-identity. Chain-element order is fixed and verified: py
`writeIdentifiable` (Rust: `write_identifiable_attributes` +
`write_identifiable_parts`) emits SHORT-NAME, LONG-NAME, DESC, CATEGORY,
INTRODUCTION, ADMIN-DATA (batch-1 ground truth).

### 1.5 Naming — grep, never guess *(adapts py Rules 0001.5 / 0003)*

- `readXxx` → `read_xxx`, `writeXxx`/`setXxx` → `write_xxx`/`set_xxx`,
  `getXxx` → `get_xxx` (the port checklist pre-computes the mapping — Rule
  0002).
- Arena names, `ElementRef` variant names, accessor names, and enum type
  names come from the **generated model** (`src/m2/**`) — grep before use:
  ```bash
  grep -rn "pub struct CompuMethod\b" src/m2/           # the struct
  grep -n "compu_methods" src/m2/autosar_templates/autosar_top_level_structure.rs   # the arena
  grep -n "CompuMethod" src/m2/autosar_templates/generic_structure/general_template_classes/ar_object.rs  # the ElementRef variant
  ```
- A name mismatch is fixed by using the generated name, never by renaming
  generated code (Rule 0003). Verify `ElementRef` variant names before
  compiling (batch ground truth: heterogeneous fields are
  `Option<ElementRef>` — check the variant exists).

### 1.6 Warnings mirror py *(see Rule 0011)*

Warning texts mirror py's wording so log output stays comparable against the
reference implementation. Unknown tags warn ("unsupported element"), never
abort.

### 1.7 Coverage both directions *(adapts py Rule 0001.7)*

Every ported attribute is covered by **both** the reader and the writer; an
attribute ported one-sided silently drops or fabricates on round-trip. The
checklist (Rule 0002) is existence-based and blind to this — the byte-identical
gate catches it only if a fixture carries the element; for elements no
fixture carries, verify by eye in Step 9b.

---

## Rule 0002 — The Generated Port Checklist *(adapts py Rules 0002 / 0017.2)*

`docs/port_checklist.md` is **generated** by
`python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src
--emit-port-checklist` — **never hand-edit it**. One row per py
reader/writer method:

```
| py method | Rust name | ported | py location |
|---|---|---|---|
| `readCompuMethod` | `read_compu_method` | [x] | `parser/arxml_parser.py:8484` |
```

- The status column is **existence-based** (scans `src/` for the `fn`): it
  measures *what is ported*, never *whether it is correct*. The fixture
  harness decides correctness. A `[x]` row on a family whose fixture still
  warns is **ported-but-unproven**, not finished.
- Regenerate after landing methods — a landed method with a stale `[ ]` row
  is an incomplete commit (Rule 0009.3). Verify with `--check` (exit 0).
- Use the checklist as the Phase 0 census: it is the authoritative map of py
  methods → Rust names → py file:line.

---

## Rule 0003 — Model Gaps Go Through py2rust *(adapts py Rule 0001.10 workflow)*

`src/m2/**` is **generated** (every file carries the banner). When a port
needs a field, accessor, arena, or type the generated model lacks:

1. **Write the gate test first**: extend
   `tools/py2rust/tests/test_model_gaps.py` with an assertion for the
   missing field's emitted shape; see it fail.
2. Fix the **tool**, not the output: `tools/py2rust/overrides.py`,
   `typemap.py`, or `templates.py` (worked example: `TYPE-TREF` collapsed to
   `Option<String>` but fixtures carry `BASE`/`DEST` → remove `TRefType`
   from `typemap.PRIMITIVES` so it becomes an arena type).
3. Regenerate: `python3 tools/py2rust/main.py --py-armodel target/py-armodel
   --out src`, then `--check` must exit 0.
4. `cargo build && cargo fmt && cargo clippy --all-targets -- -D warnings &&
   cargo test`; fix fallout **in hand-written code only**.
5. Commit the tool change separately from the port:
   `feat(py2rust): <gap fixed> (… Task 0)`.

Model gaps queue as **Task 0** of the family (Phase 0 step 4); the affected
reader/writer rows stay unported until the regenerated model lands. Do not
hand-edit `src/m2/**`, do not stub a field in hand-written code.

---

## Rule 0004 — Rust Source Style *(adapts py Rules 0003–0009; `docs/code_guide.md` is the authority)*

- **No `unwrap`/`expect`/`panic!` in library code.** Readers/writers return
  `Result<(), ParseError>` / `Result<(), WriteError>`; the CLI is the only
  place allowed to `exit(1)`. Panics are allowed in tests.
- **All cross-type mutation goes through `&mut Document`** (arenas live
  there). Scalar fields go through the generated `get_`/`set_` accessors;
  setters return `&mut Self` for chaining — but the **reader/writer source
  does not exploit it**: no statement chains two+ mutator calls on one
  receiver (adapts py Rule 0013); each call is its own statement. Spot-check
  with:
  ```bash
  grep -nE '\)\s*\.\s*(set|add)_[a-z_]+' src/parser/arxml_parser.rs src/writer/arxml_writer.rs
  ```
  (must return nothing for chained mutators; value-reading sub-expressions
  like `ref.get_dest()` are fine).
- **Payload plumbing**: an `Identifiable`-based reader starts from
  `read_identifiable_payload(element, document)?` (returns an
  `IdentifiablePayload`) and applies it via the generated setters; a writer
  starts from `write_identifiable_attributes`/`write_identifiable_parts`.
- **clippy rejects >7-argument functions** — bundle parameters into a
  struct (batch-1 lesson).
- Helper names mirror py's (`find`, `find_all`,
  `get_child_element_string`, `get_child_element_optional_ref_type` —
  code_guide §8); the whitespace rule (verbatim under `xml:space="preserve"`,
  trimmed otherwise) stays owned by the one reader method that already
  implements it.
- `cargo fmt` before clippy; run commands under `set -o pipefail` so gated
  pipes fail correctly.
- No comments unless they carry information; the one sanctioned comment is
  the `/// py `readXxx`.` doc line on each ported method (provenance — see
  Rule 0007).

---

## Rule 0005 — Tests *(adapts py Rule 0006)*

TDD: **write the test first (Red), then implement (Green)** — for the reader
(Step 2→3) and the writer (Step 4→5).

- **Reader unit test** lives in the parser file's `#[cfg(test)]` module:
  a **hand-written XML fragment** → `ARXMLParser` → assert **field values**
  through `document.get_<arena>()` accessors — never just `Ok(())` or
  element counts (a lossy parse passes those).
- **Writer unit test** lives in the writer file's `#[cfg(test)]` module:
  build the model (or parse a fragment) → serialize → compare against a
  **hand-written expected XML string**, including the empty-element rule
  (Rule 0006.2) and one non-empty case.
- Include an **empty-wrapper-list** case: wrapper absent on write, `[]` on
  re-parse.
- **Graduate the fixtures in Step 2** (the Red act at integration level):
  add to `WARNING_FREE_SOURCES` (`tests/integration/all_fixtures.rs`), remove
  from `P2_P4_PENDING` (same file), add to `FORMAT_SOURCES`
  (`tests/integration/format_byte_roundtrip.rs`). The harness then asserts
  the stronger bar and fails until Step 6 lands.
- The harness is **cumulative**: a batch must never regress an earlier
  fixture; the model-equality gate (all 32) stays green throughout.

---

## Rule 0006 — Fixtures & Byte Fidelity *(adapts py Rule 0019's fixture-immutability constraint)*

### 6.1 Fixtures are immutable

`tests/integration/test_files/*.arxml` are verbatim copies of py-armodel's
`tests/integration_tests/test_files/`. **Never edit, regenerate, or
"normalize" a fixture** — not to make a byte-diff pass, not to drop an
element, not for whitespace. The Rust side conforms to the fixtures.

### 6.2 The bar is raw-byte-identical

The writer-fidelity gate compares **raw bytes** against the original fixture —
deliberately **stricter than py**, whose harness normalizes some escapes
(py's writer unescapes `&quot;` in text; our quick-xml `BytesText` re-escapes
`"` → `&quot;`, which matches the original fixtures). Implement to the
fixture, not to py's normalized output.

**Empty-element rule** (py `patch_xml` semantics): an empty element
self-closes **iff** it carries attributes (`<L-2 L="EN"/>`), else it expands
(`<SD></SD>`). Non-empty text elements are unaffected. Owned by the writer's
`write_text_element` neighborhood.

### 6.3 py divergences are recorded, not papered over

Where py's output would differ from the fixture (escaping, ordering quirks),
the fixture wins and the divergence is recorded in the family row's note (and
the PR body) — never silently reconciled by editing either side.

---

## Rule 0007 — Verification Gates *(adapts py Rule 0006 verification block)*

Run from the repo root, in this order, **stop on any failure**:

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check   # regeneration guard
cargo build
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test    # includes: model equality (32), zero-warning gate, byte-identity gate
```

- Debugging writer diffs:
  `cargo run --bin arxml-format -- tests/integration/test_files/<f>.arxml /tmp/out.arxml
  && diff tests/integration/test_files/<f>.arxml /tmp/out.arxml` — the byte
  diff is the remaining to-do list.
- Each ported method carries a `/// py `readXxx`.` doc line citing its
  reference — this is the provenance signal for review, mirroring the role
  py's `# Spec:` checklist plays upstream.
- The zero-warning gate (`WARNING_FREE_SOURCES`) and the byte gate
  (`FORMAT_SOURCES`) apply to a batch's own fixtures at merge time; earlier
  fixtures must never regress.

---

## Rule 0008 — Deferral & Deviation Tracking *(adapts py Rule 0014)*

- **Deferred** = py behavior the fixtures never trigger or the model-backed
  subset omits (e.g. `MASK`, `VALIDITY`, `MONOTONY`,
  `ANNOTATIONS`-in-props, py's `--unescape-entities` CLI flag behaviors).
  Deferred items are recorded in the family todo row's note bullet and
  listed on the PR body. They are **not** failures — they are scoped-out
  work with a paper trail.
- **Blocked** = a method that cannot port until a model gap lands (Rule
  0003); its checklist row stays `[ ]` and the family row records the
  blocking gap.
- A divergence from py (Rule 0006.3) is recorded in the family row.
- No silent skips: every py behavior in the closure is **ported, deferred,
  or blocked** — and the todo row says which.

---

## Rule 0009 — Per-Family Session Loop, Commit & Completion *(adapts py Rule 0017)*

### 9.1 Entry & one-family-per-session

- On every invocation, check `docs/plan/sync-todo/<ClassName>.md` first.
  **File exists ⇒ resume:** take the **first row still `[ ]`**, run the
  9-step workflow for that one family. Do **not** re-run Phase 0 or re-ask
  the gates. **File missing ⇒ run Phase 0** (which ends by writing it).
- **One family per session.** After finishing a family, stop and tell the
  user to start a new session — even when the context "still feels fresh".
  A user request to continue in-session does not override this rule; the
  persistent todo file makes a fresh session cost near zero. Explain that
  and stop.
- **Drift/extension exception:** to re-port a family whose row is already
  `[x]`, the user must say so explicitly; reset that row to `[ ]` with a
  `drift` note, then sync it in a fresh session.

### 9.2 Step-level todos

Create exactly **9 session todos** before Step 1 (names per the SKILL.md
step table); mark `in_progress` when a step begins, `completed` the moment
it finishes, **and in the same action flip the matching checkbox in the todo
file's sub-checklist**. Never batch-check, never merge Red→Green pairs. A
legitimately N/A step completes with the reason appended.

### 9.3 Finish — commit, then mark `[x]`

A family is finished only after Step 9b passes. Then, in this order:

1. **Commit** to the **current feature branch**: parser + writer source,
   unit tests, the three graduation-list edits, py2rust changes (already
   committed as Task 0), and the todo file itself. Message per repo
   convention: `feat(p2): <Family>; graduate <fixtures> (… Task N)`. If the
   working branch is `main`, ask the user which feature branch to use
   before committing.
2. **Flip the todo row `[x]`** with the commit hash — in the same commit (or
   an immediate follow-up amending the todo file only).
3. **Regenerate the checklist** (`--emit-port-checklist`, then `--check`
   must pass) and commit the regenerated `docs/port_checklist.md` with the
   row flip — never hand-edit it.
4. **Report and stop.** Class finished (hash), N of M rows done, start a new
   session for the next family — or, if all rows are `[x]`, that the sync
   is complete.

**No `[x]` before the commit exists. No deferred bulk commit.** Push the
branch and open a PR per the repo's delivery convention; list deferred
items (Rule 0008) on the PR body.

### 9.4 Termination

After marking a row `[x]`, re-read the todo file: every queue row `[x]` ⇒
the sync is **finished** — report the summary. Any `[ ]` remaining ⇒ the
next session picks it up.

### 9.5 Autonomous mode

When the user asks the sync to run automatically, the orchestrator chains
families via isolated per-family subagents and skips only the
between-family pause. **9b stays mandatory for every family, in every
mode** — automation never self-confirms. Any subagent failure or
unanswered gate stops the chain.

### 9.6 Forbidden workarounds

- Keeping the queue only in the conversation (Rule 0012.6).
- Rebuilding the queue by grepping `[x]` rows in the port checklist instead
  of reading the todo file — the checklist carries no queue order, roles, or
  gap decisions.
- Porting a second family "because the context still feels fresh" (9.1).
- Marking `[x]` without a commit hash, or deferring commits to the end (9.3).
- Re-running Phase 0 when the todo file already exists (9.1).
- Hand-editing `docs/port_checklist.md` "just this one row" (Rule 0002).
- Skipping the checklist regeneration with "the next batch will regen it
  anyway" (9.3).

---

## Rule 0010 — 9b Port-Parity Confirmation Gate *(adapts py Rule 0006.1)*

Step 9 has two phases. **9a** is the automated verification (Rule 0007) —
stop on any failure. **9b** is a human confirmation gate that runs **after**
9a passes and **before** the family is committed and its fixtures stay
graduated. Automated checks are blind to most parity rules; 9b presents the
complete pre-commit checklist and gets the end user's explicit confirmation:

- **Method parity both directions** — every closure py method is ported
  (checklist rows `[x]` after regen), and every new Rust `fn` traces to a py
  method (no invented handling) (*0001.3*).
- **Reader/writer orders diffed against the py bodies** — each side's
  element list checked against its own py method, not assumed symmetric
  (*0001.4*).
- **Helper chain leveling** — each new reader calls exactly its direct
  base's payload helper (no double base reads); the writer is symmetric
  (*0001.2*).
- **No chained mutator calls** in reader/writer source (*0004*).
- **Warning texts mirror py** (*0001.6*, *0011*).
- **Names verified against the generated model** — arenas, `ElementRef`
  variants, accessors grepped, not guessed (*0001.5*).
- **Tests assert field values** and include the empty-wrapper-list case
  (*0005*).
- **Model gaps resolved through the tool** (`--check` green) or recorded as
  blocked; deferred items listed (*0003*, *0008*).
- **No `unwrap`/`expect`/`panic!` in library code**; fmt + clippy clean
  (*0004*).
- **Fixtures graduated in all three lists; earlier fixtures unregressed**
  (*0006*).

Do **not** commit the family or advance to the next queue item until the
user confirms every item. If any item failed, fix it and re-present 9b. The
commit (with graduation) is the **output** of 9b — a family whose work is
already byte-identical is still not finished until the gate ran.

---

## Rule 0011 — Warnings & Errors Parity *(adapts py's two-mode parser policy; code_guide §7)*

- **Two-mode policy**: `ParserOptions { warning: true }` (the default via
  `default_options()`) collects warnings and continues; `warning: false`
  fails fast on the first error. Unknown/unsupported elements produce a
  warning, never a hard failure — this is what lets unported fixtures keep
  passing via the warning-and-skip path.
- **Warning texts mirror py's wording** so log output stays comparable
  against the reference implementation (e.g. an unsupported interval type
  warns with py's message).
- A method that must reject invalid input returns `Err(ParseError::…)` per
  the `thiserror` model — no panics (Rule 0004).

---

## Rule 0012 — Class Closure & Discovery (Phase 0) *(adapts py Rule 0016)*

### 12.1 Build the closure

For input class C, the closure contains:

- C itself — its py `readC`/`writeC` plus C-specific helpers (wrapper
  readers, payload helpers).
- **Base helpers** — the py helper chain C's reader calls
  (`read_identifiable_payload` and below). Usually already ported; queue
  only unported ones.
- **Member types** — reader/writer methods for every aggregated child,
  wrapper list, shared enum used as a value, and referenced element type.
  Recursion depth: member types of member types are **not** pulled in
  automatically; each gets its own Phase 0 pass when its turn arrives.
- **Fixtures** — every fixture in `tests/integration/test_files/` carrying
  the family's element tags (`grep -l "<TAG-NAME>" tests/integration/test_files/*.arxml`).
- **Model gaps** — fields/accessors/arenas the generated model lacks
  (Rule 0003).

### 12.2 Confirm the closure with the end user (gate)

Before porting anything, present the **entire collected set** with each
item's role (`input` / `base helper` / `member` / `fixture` / `model gap`)
and, for members, the referencing method. Ask one question: *is this set
correct and complete — anything to add or drop?* Add ⇒ re-collect and
re-present; drop ⇒ remove and re-present. Do **not** build the queue until
the user confirms.

### 12.3 Locate each item

py methods via `docs/port_checklist.md` rows + reading the pinned py bodies;
Rust model shape via grep of `src/m2/**` (Rule 0001.5); fixtures via grep;
gaps via the Phase 0 shape check. Cite py `file:line` (the checklist carries
it) — the py source is the citation surface, replacing upstream's
`Table N.M, p.NN` citations.

### 12.4 "Exists" is not "ported"

A generated struct without reader/writer rows round-trips nothing. Queue a
member type whose methods are unported exactly like a missing one. The
checklist's `[x]` means the `fn` exists — pairing it with the family's
fixture set tells you whether it is *proven*.

### 12.5 Build the ordered sync queue — dependency-first

A method another queued method calls is queued **before** its callers:
deepest base helper first, then member types (deepest first, ties in py
call order), input class last. Model gaps land as **Task 0** — before all
reader/writer rows. A queue where a caller precedes its callee is malformed.

### 12.6 Phase 0 output — the persistent sync todo list

Write `docs/plan/sync-todo/<InputClassName>.md`:

```markdown
# Sync todo: <InputClassName>

Input class: <InputClassName> · Generated: <YYYY-MM-DD> · Queue order = row order
(resume = first class row still `[ ]`; all rows `[x]` = sync finished)

## Queue (dependency-first)
- [ ] Task 0 — model gaps: `X` needs `<field>` (py2rust overrides + regen)
- [ ] readCompu family (member · parser/arxml_parser.py:8419-8484 · fixtures: AUTOSAR_Datatypes, CompuMethod_Blueprint)
  - [ ] Step 1 — Extract py reference & Rust model shape
  - [ ] Step 2 — Write failing reader test + graduate fixtures (Red)
  - [ ] Step 3 — Port the reader (Green)
  - [ ] Step 4 — Extract writer order; write failing writer test (Red)
  - [ ] Step 5 — Port the writer (Green)
  - [ ] Step 6 — Harness to green (zero warnings + byte-identical)
  - [ ] Step 7 — Regenerate the port checklist
  - [ ] Step 8 — Model gaps & deferrals
  - [ ] Step 9 — Verify (9a) + confirm (9b)
- [ ] CompuMethod (input · parser/arxml_parser.py:8484 + writer · graduates AUTOSAR_Datatypes, CompuMethod_Blueprint)
- [ ] read/writeDataConstr (input · … · graduates 2 fixtures)

## Not queued (decisions)
- <method> — already ported `[x]` in docs/port_checklist.md
```

- One row per queued item, **each with its 9-step sub-checklist written at
  file creation**; a step checkbox flips the moment the step finishes (with
  the session todo — Rule 0009.2). A finished row keeps its sub-checklist
  and notes as the audit trail.
- The Phase 0 session **ends** after writing this file.

### 12.7 Class-row checkbox lines stay short

The parenthetical carries only role · py location · fixture set · one-clause
tags (`model gap`, `N/A writer`, `drift`). Findings and deferral write-ups
go on a separate note bullet underneath. A row line longer than ~200
characters is a violation signal — split it before editing that row again.
Never record status in a side note instead of flipping the row.

---

## Rule 0013 — Upstream Relationship & the Pin *(new for rust-armodel)*

- **py-armodel is the spec carrier.** Its model classes carry the
  `# Spec verified:` stamps; spec-fidelity work (field sets, docstrings,
  enums, deviations) happens **upstream** with py-armodel's own
  `sync-autosar-class` skill. This skill never re-derives the model from
  the PDF — it ports the pinned py implementation.
- **The pin is read-only during a class sync.** `tools/py2rust/
  PY_ARMODEL_VERSION` and the `target/py-armodel` checkout move only as a
  deliberate project decision (new fixtures upstream, an upstream fix this
  port needs) — never to unblock a family mid-sync. Re-cloning or checking
  out a different commit mid-sync invalidates every diff and gate run.
- **Bumping the pin** = re-checkout at the new commit, regenerate,
  `--check`, full harness green, checklist regenerated — a project-level
  change with its own commit, documented in the PR.
- If a class's py implementation itself deviates from its spec table, port
  **py as pinned** (parity is the bar), and note the upstream deviation in
  the family row so it can be fixed upstream — do not "fix" it on the Rust
  side.
