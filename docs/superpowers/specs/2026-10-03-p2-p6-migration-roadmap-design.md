# rust-armodel — P2–P6 Migration Roadmap Design

Date: 2026-10-03
Status: Approved for planning
Scope: Umbrella roadmap for phases P2–P6; each phase gets its own implementation plan

## 1. Context

The migration from py-armodel to rust-armodel follows the phased plan locked in the P0
design (`2026-10-01-rust-armodel-p0-walking-skeleton-design.md`). Two phases are complete:

- **P0 walking skeleton** (merged, PR #2): the full pipeline works end to end on one real
  file; the arena model, quick-xml I/O, Document-centric mutation, and the round-trip
  harness are validated.
- **P1 py2rust converter** (merged, PR #6): the complete m2 model tree is generated from
  py-armodel (1,647 classes / 323 files); all 32 pinned fixtures parse → write → re-parse →
  structurally equal; the `arxml-format` CLI is byte-identical on one fixture
  (`AdminDataWhitespace.arxml`, PR #9).

What remains is the bulk of the work: the parser and writer are still P0-scope
(512 + 436 lines against py's 16,253 + 15,889). Unsupported elements are skipped with
warnings, so the model-level round-trip over 32 fixtures is shallow — written bytes match
on exactly one fixture.

This document is the umbrella design for the remaining phases. It commits to an execution
strategy and defines hard phase gates; per-phase implementation plans (written with the
writing-plans skill) carry the task-level detail.

## 2. Goal and non-goals

### Goal

Take rust-armodel from "model-equal on 32 fixtures, byte-identical on 1" to **full parity
with py-armodel's parser/writer on the pinned fixture corpus**, then redesign the
parser/writer internals, then productize.

Phases:

- **P2–P4** — mechanical parser/writer port, batched by AUTOSAR domain.
- **P5** — redesign into a table-driven dispatch engine.
- **P6** — productization: CLI parity for the two kept tools, release engineering.

### Non-goals (whole roadmap)

- The other py CLI tools: `connector2xlsx`, `connector-update`, `format-xml` (a *different*
  CLI from `arxml-format` — single-file in-place), `uuid-checker`, `swc-list`,
  `system-signal`, `file-list`, `memory-section`, `os-config-export`.
- The `data_models`, `report`, and `lib` subsystems — ported only if the two kept CLIs
  ever need them.
- XSD validation machinery (see P5 scope, §6).
- Tracking py-armodel upstream main. The corpus stays pinned to the revision recorded in
  `tools/py2rust/PY_ARMODEL_VERSION` (32 `.arxml` fixtures). The "133 files" figure in
  AGENTS.md refers to upstream main and is corrected to 32 as part of this roadmap's docs
  hygiene. Bumping the pin is an explicit future decision.

## 3. Locked decisions

| Decision | Choice |
|---|---|
| Execution strategy | **Approach A — mechanical hand-port**: one Rust method per py method, hand-written in domain batches; py2rust emits a *progress checklist only*, never parser/writer logic |
| P2–P4 acceptance bar | All 32 fixtures: **zero parser warnings** + **byte-identical written output**; model equality stays as a safety net |
| P5 | Table-driven dispatch engine, strangler-migrated per domain, public API unchanged |
| P6 CLI scope | `arxml-dump` and `arxml-format` only, at py flag/behavior parity |
| P6 release scope | crates.io publish, docs.rs clean render, README/CHANGELOG/versioning |
| Corpus | The 32 fixtures of the pinned py-armodel revision; fixtures are never regenerated |
| Coding rules | `docs/code_guide.md` remains the authority (composition, slotmap arenas, `get_`/`set_`, no unwrap in lib code, fmt + clippy clean) |

### Rejected alternatives (for the record)

- **B — logic codegen** (py2rust translates parser/writer method bodies): translating
  behavior is a qualitatively harder codegen problem than P1's data-model emission;
  generated Rust fights the 1:1-reviewability goal; debugging shifts into the generator.
- **C — engine-first** (build the schema-driven engine directly, skip the mechanical
  port): reverses the P0 decision to defer the engine redesign until foundations are
  proven; losing the 1:1 method mapping makes parity unauditable.

## 4. Phase gates

The P0 harness is the backbone: every phase must keep it green.

| Phase | Delivers | Done when |
|---|---|---|
| **P2–P4** | Full reader/writer coverage, hand-ported method-per-method in domain batches | All 32 fixtures: zero parser warnings, byte-identical written output (`FORMAT_SOURCES` = all 32), model-equality green |
| **P5** | Hand-written parser/writer refactored to table-driven dispatch | Public API unchanged; all 32 fixtures still byte-identical at every merge; the harness is the only contract |
| **P6** | `arxml-dump` + `arxml-format` at py option parity; release engineering | Both CLIs match py's flags and exit codes; crate published to crates.io; docs.rs renders cleanly; README quickstart matches the real API |

## 5. P2–P4 — the mechanical port

### 5.1 Domain batching

Batches are fixture clusters, ordered by model-area dependency. A batch is "port the
reader/writer methods these fixtures exercise". Proposed order:

1. **Admin/packaging** — `AdminDataWhitespace.arxml` (already byte-identical; the template
   for what a batch completion looks like)
2. **Datatype blueprints** — 19 fixtures (`AUTOSAR_Datatypes.arxml` plus 18
   `AUTOSAR_MOD_AISpecification_*`) covering application data types, base types, compu
   methods, data constraints, units, physical dimensions, keywords
3. **Port/SWC blueprints** — 5 fixtures covering port interfaces, port prototype
   blueprints, SW component types
4. **ECUC** — `Os_ECUC.arxml`, `Os_ECUC_4.4.0.arxml`
5. **System/communication** — `CanSystem.arxml`
6. **BSW** — `BswMMode.arxml`, `BswM_Bswmd.arxml`
7. **Full SWC** — `SoftwareComponents.arxml`, `SwRecordDemo.arxml`

The exact fixture-to-batch mapping is finalized in the implementation plan; the
spec-level rule is: cluster by exercised model area, and the harness is cumulative —
each batch must not regress any earlier fixture.

### 5.2 The port checklist (the one py2rust extension)

`tools/py2rust` gains `--emit-port-checklist`: it AST-parses py's `arxml_parser.py` and
`arxml_writer.py`, extracts every `readXxx`/`writeXxx` method with the element tags it
handles, maps each to its snake_case Rust name, and emits a generated
`docs/port_checklist.md`. The status column is filled by scanning `src/` for the
corresponding `fn` — existence-based, never hand-maintained, regenerated by the tool.

The checklist is deliberately dumb: it measures *what is ported*, not *whether it is
correct* — the harness decides correctness. Its job is to make ~32k lines of porting
measurable and gap-proof.

### 5.3 Harness evolution

- `tests/integration/all_fixtures.rs` gains a per-fixture **zero-warnings assertion**
  (parser `get_warnings()` must be empty) alongside model equality. This is the
  parser-coverage gate.
- `tests/integration/format_byte_roundtrip.rs`'s `FORMAT_SOURCES` grows one cluster per
  batch until it lists all 32 fixtures. This is the writer-fidelity gate.
- Both gates apply to a batch's own fixtures at batch merge time; earlier fixtures must
  never regress. Fixtures not yet in a landed batch keep passing via the
  warning-and-skip path for their unported elements.

### 5.4 Porting rules

code_guide §8 applies (one reader method per py method, `&mut Document` + ids, whitespace
rule owned by one reader method; one writer emitter per py method, spec field order,
never re-derive values). The spec adds:

- **Warning texts mirror py's wording** so log output stays comparable against the
  reference implementation.
- **Model gaps loop through the tool**: when a reader needs a field or accessor py2rust
  did not emit, the fix goes through `overrides.py` + regeneration — never a hand-edit to
  `src/m2/**`. The checklist marks the method blocked until the regenerated model lands.
- **Parser warnings policy stays two-mode** (code_guide §7): `warning: true` collects and
  continues, `warning: false` fails fast; unknown elements warn, never abort.

## 6. P5 — table-driven dispatch engine

> **Status (2026-10-07): landed** ahead of the P2–P4 completion (user decision, recorded in
> `docs/superpowers/plans/2026-10-07-p5-table-driven-dispatch.md`). Both dispatch seams are
> table-driven (py2rust `--emit-dispatch-tables`, generated shim fns); the 32-fixture
> byte-identical harness was green at every strangler merge. Family sessions auto-wire new
> handlers by regenerating the tables.

"Schema-driven single engine" means concretely, with YAGNI applied:

- Replace the hand-written tag→method dispatch chains with **generated dispatch tables**
  (element tag → reader/writer handler), built by `py2rust` from the metadata it already
  extracts (class names, tags, spec placement, the registry behind `ElementRef`).
- The ported handler **bodies are untouched**; only the dispatch scaffolding becomes
  uniform: a single read loop (tag → table lookup → `handler(&mut Document, &Node, ctx)`)
  and a single write loop (model type → ordered emitter table, following the spec
  `sequenceOffset` order py's field order already encodes).
- Warnings/error policy unchanged (code_guide §7).

**Out of scope for P5:** XSD validation machinery, reflection-style generic modeling, any
change to the m2 structs or the public `ARXMLParser`/`ARXMLWriter` API.

**Migration:** strangler, one domain at a time — a domain moves from hand-dispatch to
table-dispatch with the 32-fixture byte-identical harness green at every merge; the old
dispatch chain is deleted when the last domain moves. The payoff: a future AUTOSAR
release bump is "regenerate tables", not "edit N methods".

## 7. P6 — productization

### 7.1 CLI parity

- **`arxml-format`** — already close (PR #9). Remaining: full flag parity with py's
  `arxml_format_cli.py` (`--warning`/`--no-warning`, `--remove-admin-data`,
  `--unescape-entities`, verbose logging), same positional args, same exit codes.
  Deliberately dropped: py's `arxml_format.log` file side-effect — the Rust CLI logs to
  stderr only.
- **`arxml-dump`** — py's is a human-readable report over SWC/BswM content (ports, com
  specs, variable accesses, implementation types). Port its output format 1:1. It depends
  on the model areas of batches 3, 6, and 7, so it lands last in P6. Flag-level details
  are fixed in the P6 implementation plan.

### 7.2 Release engineering

- Versioning: `0.x.y` during P2–P5; `1.0.0` at P6 exit.
- Publish to crates.io as `armodel`; if the name is taken, the fallback (e.g.
  `rust-armodel`) is decided at P6 entry. Not a P2–P4 blocker either way.
- docs.rs must render the public API cleanly; README quickstart rewritten against the
  real API; CHANGELOG maintained; release tags per version.
- CI: the GitHub Actions quality gate (build, `fmt --check`, `clippy -D warnings`, test)
  remains the merge requirement throughout the roadmap. AGENTS.md's Travis section is
  stale and is corrected as a P6 docs task.

## 8. Risks

| Risk | Mitigation |
|---|---|
| Model gaps mid-batch (py reader needs a field py2rust didn't emit) | Fix in `overrides.py` + regenerate — the P1 loop; checklist marks the method blocked until regenerated |
| Writer divergence found late (model-equal but not byte-identical) | Byte test is cumulative per batch; `arxml-format` reports size + first-differing-byte with context for fast debugging |
| Release-specific tag quirks (R20-11 vs R23-11 and friends) | The `xsd_to_version` table is already ported; per-release handling mirrors py's structure; batch tests catch drift |
| Mixed-content/whitespace edge cases beyond `xml:space` | Port py's text-capture rules verbatim in the affected handlers; `SwRecordDemo.arxml` exercises them |
| P5 refactor regresses formatting | Strangler per-domain migration; harness green at every merge is the gate |
| crates.io name `armodel` taken | Fallback chosen at P6 entry; does not block P2–P4 |
| Checklist drifts from reality | Regenerated by py2rust; status derived by scanning `src/`, never hand-maintained |

## 9. Open items carried out of this design

- Whether `data/` (the `format_byte_roundtrip` output directory, currently gitignored)
  should be tracked — flagged in PR #9, decision not needed until P6 release hygiene.
- AGENTS.md staleness (Travis section, "133 files") — corrected during the roadmap;
  Travis retirement was already a known follow-up from P0.
- Bumping the py-armodel pin (new fixtures upstream) — explicitly out of scope; revisit
  after P6.
