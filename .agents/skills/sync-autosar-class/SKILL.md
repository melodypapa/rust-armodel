---
name: sync-autosar-class
description: Sync an AUTOSAR model class's Rust implementation in rust-armodel to py-armodel parity — port its reader/writer methods, close model gaps through py2rust, and graduate its fixtures to zero-warning byte-identical round-trip. Use when the user says sync a class, port a class or class family, implement reader or writer coverage, graduate fixtures, update the port checklist, or continue or resume the class sync, or works on any P2–P4 parser/writer porting under src/parser/ or src/writer/. Phase 0 confirms the class closure before the 9-step TDD loop. Do not use for non-AUTOSAR code, for model-layer spec work that belongs to py-armodel's own sync-autosar-class skill, or for refactors with no porting surface.
author: melodypapa
repository: https://github.com/melodypapa/rust_armodel
license: MIT
metadata:
  version: "1.0.0"
  adapted_from: py-armodel sync-autosar-class 1.9.3
  keywords:
    - AUTOSAR
    - model-class
    - py-parity
    - TDD
    - port-checklist
    - reader
    - writer
    - rust-armodel
---

# Syncing an AUTOSAR Model Class in Rust (TDD)

## Core Principle

The **pinned py-armodel reference** is the source of truth: `target/py-armodel`
(checked out at the commit recorded in `tools/py2rust/PY_ARMODEL_VERSION`). Its
model classes already carry `# Spec verified:` stamps — the spec-fidelity work
happens **upstream**, with py-armodel's own `sync-autosar-class` skill. This
skill syncs the **Rust side** of one class (usually a whole class family):
port py's `readXxx`/`writeXxx` methods 1:1 into the hand-written parser/writer,
close model gaps through `tools/py2rust`, and prove the port with the fixture
harness. Sync runs in **two phases**:

- **Phase 0 — Discovery & class closure (Rule 0012):** build the closure of
  related work items (base helpers + member-type methods the class's reader
  calls, fixtures exercising its tags, model gaps), confirm the collected set
  with the user, locate each py method and the Rust model shape, and write the
  persistent sync todo list (`docs/plan/sync-todo/<InputClassName>.md`) — the
  queue survives session death.
- **Phase 1 — 9-step TDD per class family (Rules 0001–0011, 0013):** consume
  the queue one family at a time, **one family per fresh session** (Rule 0009).
  Two Red→Green pairs per family: reader (2→3) and writer (4→5). Write the
  failing test before the implementation. The 9 steps are mirrored into the
  session todo list — one todo per step, checked off the moment its step
  finishes — **and into the todo file itself: every queued family row carries a
  9-step sub-checklist written at file creation (Phase 0) and flipped per step,
  so step progress survives session death (Rule 0012.6)**. Each family ends
  with a port-parity confirmation gate (Step 9b) before its fixtures are
  graduated, the row is stamped finished, and the work is committed to the
  feature branch. All rows `[x]` in the todo list = the sync is finished.

The parse/write shape every port feeds (release is detected from the file, not
set by the caller — py's `setARRelease` has no Rust counterpart):

```rust
let mut document = Document::new();
ARXMLParser::new(default_options()).load(path, &mut document)?;
ARXMLWriter::new().save(path, &document)?;
```

Detailed rules live in **`rules.md`** (*Rule 0001*–*Rule 0013*); this skill is
self-contained. Each step below points into `rules.md` for the detail — do not
re-derive it here.

## When to use / NOT to use

**Use** when porting, extending, or verifying parser/writer coverage for an
AUTOSAR class (or family) against the py-armodel reference — the P2–P4
mechanical port (`src/parser/arxml_parser.rs`, `src/writer/arxml_writer.rs`,
plus model-gap fixes routed through `tools/py2rust`).

**Not for:**
- **Model-layer spec work** — if a class's py source is missing, stubbed, or
  out of sync with its PDF spec table, that fix belongs to **py-armodel's**
  `sync-autosar-class` skill, followed by an explicit pin bump
  (`tools/py2rust/PY_ARMODEL_VERSION`) and regeneration. Bumping the pin is a
  deliberate decision, never a side effect of a class sync (Rule 0013).
- Non-AUTOSAR code; refactors with no porting surface; P5 dispatch-engine
  redesign work (its own plans).

## Phase 0 — Discovery & Class Closure (Rule 0012)

Before the per-family 9-step loop, build the closure of work items the input
class depends on, have the end user confirm the collected set, locate each py
method and the generated Rust model shape, and emit an ordered sync queue.
The 9-step workflow assumes this closure exists — running it without Phase 0
risks inventing element handling or arena names mid-port when a referenced
member type turns out to be unported (Rule 0001.3).

**Procedure (full detail in Rule 0012):**

1. **Closure** = {input class's py reader/writer methods} ∪ {the py helper
   chain its reader calls — base payloads, wrapper readers} ∪ {member types'
   reader/writer methods: aggregated children, shared enums used as values} ∪
   {fixtures exercising the family's element tags} ∪ {model gaps: fields or
   accessors `py2rust` did not emit}.
2. **Confirm the collected set (gate).** Present every collected item to the
   end user with its role (`input` / `base helper` / `member` / `fixture` /
   `model gap`) and, for members, the referencing method, then ask: *is this
   set correct and complete?* Do **not** port anything or build the queue
   until the user confirms. This membership gate is distinct from the
   model-gap resolution gate in step 4.
3. **Locate each item.** For py methods: `grep -n "def readXxx\|def writeXxx"
   target/py-armodel/src/armodel/parser/arxml_parser.py
   target/py-armodel/src/armodel/writer/arxml_writer.py` and the generated
   checklist rows (`docs/port_checklist.md` — py method → Rust name → ported
   → py location). For the Rust model: the generated struct, its arena name
   (`document.<arena>`), `ElementRef` variant, and accessors in `src/m2/**`.
   For fixtures: grep the fixture files for the family's element tags.
4. **Resolve model gaps (interactive, batched).** For every field/accessor the
   port needs but the generated model lacks, the fix goes through `tools/
   py2rust` (`overrides.py`/`typemap.py` + regeneration — Rule 0003), queued
   as Task 0 of the family. Do not proceed past the gate without listing them;
   do not plan a hand-edit to `src/m2/**` — generated files are never
   hand-edited.
5. **Build the sync queue — dependency-first** (Rule 0012.5): a method another
   queued method calls (base payload reader, member-type reader/writer) is
   queued **before** its callers; deepest helper first, input class last. Skip
   methods already marked `[x]` in `docs/port_checklist.md` unless drift or
   extension is intended. **"The model struct exists" is not "ported"** — a
   member type with a generated struct but no reader/writer rows is queued
   like a missing one (Rule 0012.4).
6. **Write the sync todo list file** (Rule 0012.6): persist the confirmed
   queue to `docs/plan/sync-todo/<InputClassName>.md` — one row per queued
   item, **each row carrying its 9-step sub-checklist (all `[ ]`, names per
   Rule 0009.1, written now at file creation — not deferred to family
   start)**, plus the model-gap decisions. **The queue lives in this file, not
   in the conversation.** Keep each row's checkbox line short — role · py
   location · fixture set only; put any Step 1 finding on a separate note
   bullet underneath, never appended to the checkbox line (Rule 0012.7). The
   Phase 0 session ends here.

**Output:** `docs/plan/sync-todo/<InputClassName>.md` — the persistent queue
(Rule 0012.6). Phase 1 consumes it one row at a time, **one family per fresh
session** (Rule 0009).

## Phase 1 — Session loop & the 9-step workflow (Rule 0009)

- **Entry (every session):** the user invokes the skill (e.g. `/sync-autosar-class
  <ClassName>` or "continue the sync"). If `docs/plan/sync-todo/<ClassName>.md`
  exists, **resume — do NOT re-run Phase 0** (the closure was already confirmed;
  re-running it re-asks the interactive gates for nothing). Read the todo file,
  take the **first row still `[ ]`**, and run the 9-step workflow for that one
  family. If the file does not exist, run Phase 0 first.
- **One family per session.** Never sync two families in one session, even when
  the context still feels fresh — the Step 9b order-diff work degrades silently
  under a loaded context. After a family finishes (below), stop and tell the
  user to start a new session. Sole exception — **autonomous mode (Rule 0009.5)**:
  when the user asks the sync to run automatically, the orchestrator chains
  families via isolated per-family subagents and skips only the between-family
  pause; the 9b confirmation stays mandatory for every family, in every mode.
- **Mirror the 9 steps into the session todo list (Rule 0009.2).** After taking
  the `[ ]` row and before Step 1, create **9 session todos — one per workflow
  step** (`Step 1 — Extract py reference & Rust model shape` … `Step 9 —
  Verify (9a) + confirm (9b)`). Mark each `in_progress` when the step begins
  and `completed` **the moment that step finishes** — one completed step =
  exactly one newly checked todo item, **and in the same action flip the
  matching step checkbox in the todo file's per-family 9-step sub-checklist**
  (Rule 0012.6 — written at file creation; the file is the durable record, the
  session todos the live display). Never merge steps into fewer todos, never
  batch-check. N/A steps (e.g. 4/5 for a read-only py quirk with no writer
  surface) complete with the N/A reason.
- **Finish (per family, after 9b):** once the user confirms Step 9b — (1)
  commit the family's changes to the **current feature branch** (parser +
  writer source, unit tests, the three graduation-list edits, py2rust changes
  if any, and the todo file itself; message following the repo convention
  `feat(p2): <Family>; graduate <fixtures>`), (2) flip the todo row to `[x]`
  and record the commit hash in the same commit, (3) run
  `python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src
  --emit-port-checklist` so `docs/port_checklist.md` reflects the landed
  methods (then `--check` must pass) and commit the regenerated checklist with
  the row flip — never hand-edit it (Rule 0002), (4) report and stop. The
  finished row **keeps** its 9-step sub-checklist and write-ups as the durable
  audit trail — never collapse or strip it.
- **Termination:** after marking a row `[x]`, if **every** queue row is `[x]`,
  the sync is **finished** — report the summary (families, commits, graduated
  fixtures, deferred items). No further session needed. Any `[ ]` left → next
  session picks it up.

## The port checklist is the progress ledger

Progress is tracked in **`docs/port_checklist.md`** — **generated** by
`tools/py2rust --emit-port-checklist`, never hand-edited (Rule 0002). One row
per py method: `| py method | Rust name | ported | py location |`. The status
column is existence-based (scans `src/` for the `fn`), so it measures *what is
ported*, never *whether it is correct* — the fixture harness decides
correctness. Regenerate it after landing methods; a row whose Rust `fn` exists
but whose fixture still warns is a **ported-but-unproven** method, not a
finished one.

Graduating a fixture = editing **three lists** (the harness then asserts the
stronger bar):

| List | File | Meaning |
|---|---|---|
| `WARNING_FREE_SOURCES` | `tests/integration/all_fixtures.rs` | parse produces **zero warnings** (parser consumes every element) |
| `P2_P4_PENDING` | `tests/integration/all_fixtures.rs` | fixture **leaves** this list on graduation (deeper checks no longer skipped) |
| `FORMAT_SOURCES` | `tests/integration/format_byte_roundtrip.rs` | written output is **byte-identical** to the original fixture |

The bar is **raw-byte-identical** — stricter than py, whose own writer
normalizes some escapes (verified: py unescapes `&quot;` in text; our
quick-xml `BytesText` re-escapes `"` → `&quot;`, which matches the original
fixtures). Fixtures are verbatim copies of py's
`tests/integration_tests/test_files/*.arxml` — **never regenerate or edit
them** (Rule 0006).

## Input

**Required:** `ClassName` (the input class; the queue usually covers its whole
family). From it, locate:

| Artifact | Path |
|---|---|
| py reader/writer reference | `grep -n "def read<ClassName>\|def write<ClassName>" target/py-armodel/src/armodel/parser/arxml_parser.py target/py-armodel/src/armodel/writer/arxml_writer.py` — the pinned reference bodies: element lists **in order**, attribute handling, warning texts |
| py model + parity checklist | `target/py-armodel/src/armodel/models/M2/AUTOSARTemplates/<pkg>/…` — the class's `# Spec:` block carries the upstream reader/writer ownership columns; cite it when py's behavior looks ambiguous |
| generated port checklist | `docs/port_checklist.md` — py method → Rust name → `[x]` status → py file:line (generated; Rule 0002) |
| generated Rust model | `src/m2/autosar_templates/…` (spec-package-derived path, `docs/code_guide.md` §2) — struct fields, arena name (`document.<arena>`), `ElementRef` variant, `get_`/`set_` accessors. **Never hand-edit** (Rule 0003) |
| py2rust tool | `tools/py2rust/` — model-gap fixes go here (`overrides.py`, `typemap.py`) + regeneration (`Rule 0003`) |
| dispatch seams | `src/parser/arxml_parser.rs` `read_element_payload` / `src/writer/arxml_writer.rs` `write_ar_package_element` — package-element tags get an `ElementRef` arm here |
| harness fixtures | `tests/integration/test_files/*.arxml` (32, verbatim from py); lists in `all_fixtures.rs` + `format_byte_roundtrip.rs` |
| spec corpus (rare) | `target/py-armodel/autosar/R23-11/{markdown,pdf,xsd}` — consult only when py's behavior is ambiguous; `python3 .agents/skills/sync-autosar-class/pdf_page.py <ClassName>` finds the PDF page |
| py2rust pin | `tools/py2rust/PY_ARMODEL_VERSION` — the pinned py commit; **read-only** for class syncs (Rule 0013) |

### The 9-step workflow (TDD, per class family)

Runs once per family in the queue built by Phase 0, inside the session loop
above. Two Red→Green pairs: **2→3** (reader) and **4→5** (writer). Do not
write the implementation before its failing test. Each step is tracked as its
own session todo — created as a set of 9 before Step 1, checked off one at a
time as each step finishes (*Rule 0009*).

| Step | What | Rules | Phase |
|---|---|---|---|
| 1 | Extract the py reference (bodies, orders, warnings) & the Rust model shape | 0001, 0003 | — |
| **2** | **Write the failing reader test + graduate the fixtures in the three lists** | 0005, 0006 | **Red** |
| **3** | **Port the reader** (+ `ElementRef` dispatch arm) | 0001, 0004 | **Green** |
| 4 | Extract writer order (**independent of the reader!**); write the failing writer test | 0001.4, 0005 | **Red** |
| **5** | **Port the writer** (+ dispatch arm) | 0001, 0004 | **Green** |
| 6 | Drive the fixtures to green in the harness (zero warnings + byte-identical) | 0006, 0007 | — |
| 7 | Regenerate the port checklist (never hand-edit) | 0002 | — |
| 8 | Model gaps ⇒ py2rust loop; record deferrals | 0003, 0008 | — |
| 9 | Verify (9a) + confirm (9b) ⇒ commit + flip todo row | 0007, 0009, 0010 | — |

**Essence per step** (full detail in `rules.md`):

- **1** — For every py method in the closure, read the **body**: the element
  list **in py's order**, attribute handling (`S`/`T`, `DEST`, …), warning
  texts, and which helpers it calls. Then record the Rust model shape:
  struct + arena name, `ElementRef` variant name, accessor names — **grep
  them, never guess** (Rule 0001.5). List the fixtures that exercise the
  family's tags. Collect model gaps now; they queue as Task 0 (Rule 0003).
- **2** — Unit test in the parser file's `#[cfg(test)]` module: a
  **hand-written XML fragment** → parse → **assert field values** (not just
  `Ok(())` or element counts). In the same step, edit the three graduation
  lists — the harness now asserts the stronger bar and **fails** (that is the
  integration-level Red).
- **3** — One `read_xxx` per py `readXxx` in `arxml_parser.rs`, signature
  `(element: &Node, …, document: &mut Document)`; the helper call chain
  mirrors the model composition chain (never re-read a base level — Rule
  0001.2); insert into arenas via `document`, mutate via the generated
  setters; wire the `ElementRef` dispatch arm if the tag is a package
  element. Warnings mirror py's wording (Rule 0001.6).
- **4** — Read the py **writer** body separately: writer element order is
  py's own and **may differ from the reader** (worked example: DataConstr
  rules — reader INTERNAL-CONSTRS then PHYS-CONSTRS, writer PHYS first).
  Writer unit test: build/parse a model → serialize → compare a hand-written
  expected XML string, including the empty-element rule (self-closing iff
  attributes — Rule 0006.2).
- **5** — One `write_xxx`/`set_xxx` per py method in `arxml_writer.rs`,
  element order exactly as extracted in Step 4; never re-derive values
  (code_guide §8); emit wrapper elements **only when non-empty**.
- **6** — Run the harness for the family's fixtures; debug with
  `cargo run --bin arxml-format -- tests/integration/test_files/<f>.arxml
  /tmp/out.arxml && diff` — the byte diff is the to-do list. Iterate until
  zero warnings + byte-identical; earlier fixtures must never regress.
- **7** — Regenerate: `python3 tools/py2rust/main.py --py-armodel
  target/py-armodel --out src --emit-port-checklist`; confirm the family's
  rows flipped `[x]`.
- **8** — Model gaps discovered mid-port go through the py2rust loop
  (gate test in `tools/py2rust/tests/test_model_gaps.py` written first →
  `overrides.py`/`typemap.py` → regenerate → `--check` green). Py branches
  the fixtures never trigger (e.g. `MASK`, `VALIDITY`) are **recorded as
  deferred items** for the PR, never silently skipped (Rule 0008).
- **9** — **(9a automated)** `cargo build && cargo fmt --all -- --check &&
  cargo clippy --all-targets -- -D warnings && cargo test` — full suite,
  all three harness gates, `--emit-port-checklist` + `--check` clean. **Stop
  on any failure.** **(9b confirm — gate)** then present the **complete
  pre-commit** port-parity checklist covering every check automation is
  blind to:
  - method parity both directions — every closure py method ported, every
    new Rust `fn` traces to a py method (no invented handling) (*0001.3*)
  - reader/writer element orders diffed against the py bodies — not assumed
    symmetric (*0001.4*)
  - helper chain levels mirror the model composition chain — no double base
    reads (*0001.2*); no chained mutator calls (*0004*)
  - warning texts mirror py (*0001.6*); names verified (arena/`ElementRef`/
    accessor) against the generated model, not guessed (*0001.5*)
  - tests assert field values, include the empty-wrapper-list case (*0005*)
  - model gaps resolved through the tool (`--check` green) or recorded;
    deferred items listed for the PR (*0003*, *0008*)
  - no `unwrap`/`expect`/`panic!` in library code; fmt + clippy clean
    (*0004*)
  - fixtures graduated in all three lists; earlier fixtures unregressed
    (*0006*) — and get explicit user confirmation; **when all pass, commit
    and flip the todo row** (*0009*). Fix & re-present on any failure
    (*Rule 0010* has the full checklist).

**Workflow adaptations** (which steps still apply):

- **Attribute-only classes** (py handles the class purely as attrs on the
  parent's element — no own element): Steps 2–5 collapse into the parent's
  port; the family row records `attribute-only` and Steps 2–6 complete with
  the parent's test as the evidence.
- **Non-package-element classes** (nested tags — e.g. `COMPU` under
  `COMPU-INTERNAL-TO-PHYS`, `KEYWORD` under `KEYWORDS`): no `ElementRef`
  dispatch arm; the reader inserts directly into the arena and the writer is
  a helper the parent calls. Only tags that appear as `AR-PACKAGE` elements
  get arms (Rule 0001.1).
- **Model gap blocks the port** (reader needs a field py2rust did not emit):
  the checklist row stays `[ ]` until the regenerated model lands (Task 0);
  do not stub the field by hand (Rule 0003).
- **py-only behavior differences** (e.g. py normalizes an escape the fixture
  carries raw): our raw-byte bar wins — implement to the fixture, and record
  the divergence in the family row's note (Rule 0006.3).

## Common Mistakes / Red Flags — STOP

- **Hand-editing `src/m2/**`** to unblock a port — the tree is generated
  (banner says so); the fix goes through `tools/py2rust` + regeneration, and
  `--check` must stay green (*Rule 0003*).
- **Assuming the writer order from the reader** — py's reader is find-based
  (wrapper-relative) and the writer emits in `sequenceOffset` order; they
  legitimately differ. Extract each from its own py body (*Rule 0001.4*).
- **Guessing arena / `ElementRef` / accessor names** — grep the generated
  model first; a wrong guess is a compile error at best, a silent second
  arena at worst (*Rule 0001.5*).
- **Porting before the test** (reader 2→3, writer 4→5).
- **Treating a ported `fn` as done because the checklist row flipped** — the
  checklist is existence-based; only the harness (zero warnings +
  byte-identical) proves correctness (*Rule 0002*).
- **Asserting only `Ok(())` or element counts** in the unit test — assert
  field values, including one aggregated child a level down (*Rule 0005*).
- **Re-reading a base payload level in a subclass reader** — the helper chain
  mirrors the model composition chain one level at a time; double reads
  duplicate work and diverge from py (*Rule 0001.2*).
- **Emitting an empty wrapper element unconditionally** — wrappers are
  written only when non-empty; empty elements self-close **iff** they carry
  attributes (*Rule 0006.2*).
- **Editing a fixture `.arxml`** to make a byte-diff go away — fixtures are
  verbatim; the Rust side conforms to them, never the reverse (*Rule 0006*).
- **Deferring a model gap into a hand-written stub field** — queue it as
  Task 0 through py2rust; a stubbed field is a fabrication the harness can
  pass while the model is wrong (*Rule 0003*).
- **Silently skipping untriggered py branches** — record them as deferred
  items on the PR (*Rule 0008*).
- **Hand-editing `docs/port_checklist.md`** — regenerate it; hand edits
  drift from `src/` on the next run (*Rule 0002*).
- **Keeping the queue only in the conversation** — the sync map that lives
  in conversation dies with the session. The queue lives in
  `docs/plan/sync-todo/<InputClassName>.md` (*Rule 0012.6*).
- **Re-running Phase 0 on resume** — a todo file for the family already
  exists ⇒ the closure was confirmed; read the file and take the first
  `[ ]` row (*Rule 0009.1*).
- **Syncing a second family in the same session** — "context still feels
  fresh" is not evidence; the 9b order diffs degrade silently under a loaded
  context. One family per session, then stop (*Rule 0009.1*).
- **Marking a todo row `[x]` before the commit exists** — the row records
  the commit hash; no commit, no `[x]` (*Rule 0009.3*).
- **Merging the 9 steps into fewer session todos, or batch-checking them** —
  one step = one todo, checked the moment the step finishes (*Rule 0009.2*).
- **Committing without regenerating the checklist** — a landed method with a
  stale `[ ]` row is an incomplete commit (*Rule 0009.3*).
- **Bumping `PY_ARMODEL_VERSION` or re-cloning `target/py-armodel` mid-sync**
  — the pin moves only as a deliberate project decision, never to unblock a
  class (*Rule 0013*).

| Rationalization | Reality |
|---|---|
| "The generated struct exists, so the member type is done" | Existence is not ported — a struct without reader/writer rows round-trips nothing; queue it (*Rule 0012.4*). |
| "The writer order surely matches the reader" | They are independent in py; DataConstr rules are the counter-example. Extract each body (*Rule 0001.4*). |
| "Simple method — I'll port then test" | A test written after mirrors the code, not py's behavior. Step 2 first. |
| "It's just a one-field addition to the generated file" | `src/m2/**` is generated; one hand-edit breaks `--check` and the next regen clobbers it. Go through `overrides.py` (*Rule 0003*). |
| "The checklist row is `[x]`, the method works" | The row only says the `fn` exists. Only zero warnings + byte-identical prove it (*Rule 0002*). |
| "The fixture bytes look wrong; I'll fix the fixture" | Fixtures are verbatim upstream artifacts; our raw-byte bar is deliberately stricter than py (*Rule 0006*). |
| "That py branch never fires in the corpus — skip it silently" | Record it as a deferred item on the PR; silent skips are unverifiable (*Rule 0008*). |
| "The arena is probably called `document.<x>s` — I'll just write it" | Grep the generated model. Wrong names are compile noise at best, wrong-arena bugs at worst (*Rule 0001.5*). |
| "Tests pass and the family's fixtures are byte-identical — I can move on" | 9b still applies: order diffs, warning texts, no-fabrication, and the deferral list are automation-blind (*Rule 0010*). |
| "I'll keep the queue in the conversation — writing a file is overhead" | The conversation dies with the session; the queue, roles, and gap decisions are lost. The todo file is the queue (*Rule 0012.6*). |
| "Context still feels fresh — I'll port the next family in this session" | 9b diffs degrade silently under loaded context; one family per session (*Rule 0009.1*). |
| "I'll commit everything at the end of the whole sync" | A session death then loses every finished family's work; commit per family, right after 9b (*Rule 0009.3*). |
| "One todo for the family is enough — or I'll check them all at the end" | The step todos expose a skipped/half-finished step in real time; batch-checking shows progress the work doesn't have (*Rule 0009.2*). |

## References

- **Rules (self-contained):** `rules.md` in this skill folder — *Rule 0001*–*Rule 0013*.
- Coding standards: `docs/code_guide.md` (the Rust authority — composition,
  slotmap arenas, naming, errors, testing).
- Roadmap: `docs/superpowers/specs/2026-10-03-p2-p6-migration-roadmap-design.md`
  (§5 = the P2–P4 port this skill executes); batch plans under
  `docs/superpowers/plans/`.
- py-armodel reference (pinned): `target/py-armodel/` — clone/refresh per
  `docs/superpowers/plans/2026-10-02-p1-py2rust-converter.md`; pin recorded in
  `tools/py2rust/PY_ARMODEL_VERSION`.
- Spec corpus (via the pinned checkout): `target/py-armodel/autosar/R23-11/`
  (`markdown`, `pdf`, `xsd`; R4.3.1 fallback corpus also present).
- Page-number helper: `python3 .agents/skills/sync-autosar-class/pdf_page.py
  <ClassName>` (reads the pinned corpus; `--pdf PATH` for one PDF, `--table
  N.M` by table id).
- Upstream spec-sync skill (model layer): py-armodel's
  `.agents/skills/sync-autosar-class/` — run it **in the py-armodel repo**
  when a class itself is out of sync with its spec table.
