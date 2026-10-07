# P5 Table-Driven Dispatch Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development
> (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the hand-written `ElementRef`→method dispatch matches in the reader and
writer with py2rust-generated dispatch tables, strangler-migrated per domain, with the public
API and warning policy unchanged.

**Architecture:** py2rust already scans py's parser/writer and already scans `src/` for
ported `fn`s (that is how `--emit-port-checklist` fills its status column). A new
`--emit-dispatch-tables` mode reuses both scans to emit two generated table modules —
`src/reader/arxml_reader/dispatch_tables.rs` (element tag → reader handler) and
`src/writer/arxml_writer/dispatch_tables.rs` (`ElementRef` variant index → writer emitter) —
containing entries only for handler `fn`s that exist in `src/`. The two hand-written `match`
seams become table lookups with the same fallback behavior; after the last domain moves, the
hand matches are deleted. Handler bodies are untouched.

**Tech Stack:** Rust (no new dependencies — `std::sync::LazyLock` + sorted-slice binary
search), Python (py2rust generator + its pytest gate), the 32-fixture byte-identical harness
as the only migration contract.

**Prerequisite:** P2–P4 complete (`docs/superpowers/plans/2026-10-07-p2-p4-mechanical-port.md`
Task 7 exit gate green). Migrating a partially-ported dispatch makes the table a second
source of truth for coverage — the strangler starts only when the hand chain is feature-complete.

**Out of scope (roadmap §6):** XSD validation machinery, reflection-style generic modeling,
any change to the m2 structs or the public `ARXMLReader`/`ARXMLWriter` API, warning policy.

---

## File structure

- Create: `tools/py2rust/dispatch_tables.py` — the emitter (pure function: scanned handler
  inventory → two Rust file texts)
- Modify: `tools/py2rust/main.py` — `--emit-dispatch-tables` flag wiring
- Modify: `tools/py2rust/tests/test_dispatch_tables.py` — new gate test (written first)
- Create: `src/reader/arxml_reader/dispatch_tables.rs` — generated (banner-marked)
- Create: `src/writer/arxml_writer/dispatch_tables.rs` — generated (banner-marked)
- Modify: `src/reader/arxml_reader/mod.rs` — `read_element_payload` becomes the single read
  loop (table lookup + fallback)
- Modify: `src/writer/arxml_writer/mod.rs` — `write_ar_package_element` becomes the single
  write loop (table lookup + fallback); `save()` gains the one-line writer shim

## Design decisions (locked)

1. **Reader table is tag-keyed.** `(tag, handler)` where handler is
   `fn(&mut ARXMLReader, &Node, ElementRef, &mut Document) -> Result<(), ParseError>`. The
   allocation loop already resolved tag→`ElementRef` via the element registry; the table
   adds only handler lookup. Lookup: `READ_TABLE.binary_search_by_key(|e| e.0, tag)` over a
   `&'static [(…)]` sorted by tag.
2. **Writer table is `ElementRef`-discriminant-keyed.** The writer's inputs are ids, not
   tags. Table entries are `(variant_index, handler)` where `variant_index` is generated as
   a match arm over `ElementRef` (the generator knows every variant; the arm is data-shaped:
   `ElementRef::CompuMethod(_) => 7`). Handler signature:
   `fn(&ARXMLWriter, &mut dyn Write, ElementRef, &Document) -> Result<(), WriteError>`.
3. **Writer handlers become `dyn Write`.** Generic fn pointers cannot live in a static
   table. Handlers switch from `writer: &mut Writer<W>` to `writer: &mut Writer<&mut dyn
   Write>`; the single generic entry point (`save<W: Write>`) constructs
   `Writer::new(&mut dyn_write_shim)` once. This is the only handler-signature change of the
   phase and it is mechanical (`cargo` errors point at every site).
4. **Fallback preserves the current behavior.** Reader: tag not in table → `Ok(())` (the
   allocation loop's warning-and-skip path unchanged). Writer: variant not in table →
   `Ok(())` — the hand-written match's fallback is `_ => {}` (silent skip) and the table
   must not change that contract; the writer-side warning question stays out of scope.
5. **No new dependencies.** `std::sync::LazyLock` (stable since 1.80) hosts the sorted
   slices; no map/phf crates.

---

### Task 1: Generator gate test (Red)

**Files:**
- Create: `tools/py2rust/tests/test_dispatch_tables.py`

- [ ] **Step 1: Write the failing gate test**

```python
"""Gate tests for --emit-dispatch-tables (P5, roadmap 2026-10-03 §6)."""
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
MAIN = REPO / "tools" / "py2rust" / "main.py"


def _emit(tmp_path: Path) -> tuple[str, str]:
    """Run the emitter against a COPY of the repo src tree: the handler
    existence scan reads --out, so an empty temp dir would yield empty tables."""
    out = tmp_path / "src"
    shutil.copytree(REPO / "src", out)
    subprocess.run(
        [sys.executable, str(MAIN), "--py-armodel", str(REPO / "target" / "py-armodel"),
         "--out", str(out), "--emit-dispatch-tables"],
        check=True,
    )
    reader = (out / "reader" / "arxml_reader" / "dispatch_tables.rs").read_text()
    writer = (out / "writer" / "arxml_writer" / "dispatch_tables.rs").read_text()
    return reader, writer


def test_reader_table_is_sorted_and_covers_ported_handlers(tmp_path):
    reader, _ = _emit(tmp_path)
    tags = [line.split('"')[1] for line in reader.splitlines() if line.startswith('    (')]
    assert tags == sorted(tags), "reader table must be sorted by tag for binary search"
    assert "COMPU-METHOD" in tags          # ported handler (read_compu_method exists)
    assert "DATA-CONSTR" in tags


def test_unported_methods_produce_no_table_entry(tmp_path):
    reader, writer = _emit(tmp_path)
    assert "read_sender_com_spec" not in reader        # not ported at P5 start
    assert "write_sender_com_spec" not in writer
```

(The writer-table assertions land with Task 4, after the dyn-Write conversion makes the
writer handlers table-storable; before that conversion the generated writer file must not
be declared as a module — a generic fn cannot be a fn pointer.)

- [ ] **Step 2: Run it to verify it fails**

Run: `python3 -m pytest tools/py2rust/tests/test_dispatch_tables.py -v`
Expected: FAIL — `main.py: error: unrecognized arguments: --emit-dispatch-tables`.

### Task 2: Generator `--emit-dispatch-tables` (Green)

**Files:**
- Create: `tools/py2rust/dispatch_tables.py`
- Modify: `tools/py2rust/main.py`

- [ ] **Step 1: Implement the emitter**

`dispatch_tables.py` (core logic; `scan_ported_handlers` and the py AST walk are reused
from the existing `--emit-port-checklist` implementation in `main.py` — import them, do not
copy):

```python
"""Emit reader/writer dispatch tables from the ported-handler scan (P5)."""

READER_ENTRY = '    (\n        "{tag}",\n        ARXMLReader::{rust_fn},\n    ),\n'
WRITER_INDEX_ARM = "            ElementRef::{variant}(_) => {index},\n"
WRITER_ENTRY = '    (\n        {index},\n        ARXMLWriter::{rust_fn} as write_shim_{rust_fn},\n    ),\n'

READER_TEMPLATE = """//! GENERATED by tools/py2rust --emit-dispatch-tables. DO NOT EDIT.
//! Tag-keyed reader dispatch table (P5, roadmap 2026-10-03 §6). Entries exist
//! only for handler fns present in src/ — regenerate after landing families.
use super::*;

pub(crate) static READ_TABLE: LazyLock<Vec<(&'static str, ReadHandler)>> =
    LazyLock::new(|| vec![
{reader_entries}]);

pub(crate) type ReadHandler =
    fn(&mut ARXMLReader, &Node, ElementRef, &mut Document) -> Result<(), ParseError>;

pub(crate) fn lookup_read_handler(tag: &str) -> Option<ReadHandler> {{
    READ_TABLE
        .binary_search_by(|entry| entry.0.cmp(tag))
        .ok()
        .map(|index| READ_TABLE[index].1)
}}
"""

WRITER_TEMPLATE = """//! GENERATED by tools/py2rust --emit-dispatch-tables. DO NOT EDIT.
//! ElementRef-variant-keyed writer dispatch table (P5). Entries exist only
//! for handler fns present in src/ — regenerate after landing families.
use super::*;

pub(crate) type WriteHandler =
    fn(&ARXMLWriter, &mut Writer<&mut dyn std::io::Write>, ElementRef, &Document)
        -> Result<(), WriteError>;

pub(crate) fn variant_index(element: &ElementRef) -> Option<usize> {{
    let index: usize = match element {{
{index_arms}            _ => return None,
    }};
    Some(index)
}}

pub(crate) static WRITE_TABLE: LazyLock<Vec<(usize, WriteHandler)>> =
    LazyLock::new(|| vec![
{writer_entries}]);

pub(crate) fn lookup_write_handler(index: usize) -> Option<WriteHandler> {{
    WRITE_TABLE
        .binary_search_by(|entry| entry.0.cmp(&index))
        .ok()
        .map(|idx| WRITE_TABLE[idx].1)
}}
"""


def emit_dispatch_tables(ported_reader_methods, ported_writer_methods, element_variants,
                         out_dir):
    """ported_*: {py_method: (rust_fn, tag/variant)} from the checklist scan."""
    reader_entries = "".join(
        READER_ENTRY.format(tag=tag, rust_fn=rust_fn)
        for _py, (rust_fn, tag) in sorted(ported_reader_methods.items(), key=lambda kv: kv[1][1])
    )
    variants = sorted(
        (variant, rust_fn)
        for _py, (rust_fn, variant) in ported_writer_methods.items()
    )
    index_arms = "".join(
        WRITER_INDEX_ARM.format(variant=variant, index=index)
        for index, (variant, _rust_fn) in enumerate(variants)
    )
    writer_entries = "".join(
        WRITER_ENTRY.format(index=index, rust_fn=rust_fn)
        for index, (_variant, rust_fn) in enumerate(variants)
    )
    reader_path = out_dir / "reader" / "arxml_reader" / "dispatch_tables.rs"
    writer_path = out_dir / "writer" / "arxml_writer" / "dispatch_tables.rs"
    reader_path.parent.mkdir(parents=True, exist_ok=True)
    writer_path.parent.mkdir(parents=True, exist_ok=True)
    reader_path.write_text(READER_TEMPLATE.format(reader_entries=reader_entries))
    writer_path.write_text(
        WRITER_TEMPLATE.format(index_arms=index_arms, writer_entries=writer_entries)
    )
```

- [ ] **Step 2: Wire the flag in `main.py`**

In the argparse block (`main.py:266-270`), after the `--emit-port-checklist` line:

```python
    parser.add_argument("--emit-dispatch-tables", action="store_true",
                        help="emit reader/writer dispatch tables for ported handlers (P5)")
```

The handler inventory comes from the same scan the port checklist uses
(`main.py:321-334` scans `arxml_parser.py`/`arxml_writer.py` for `readXxx`/`writeXxx` and
`src/` for the snake_case `fn`s — factor that block's per-method inventory into a helper
`collect_ported_handlers() -> tuple[dict, dict]` returning
`{py_method: (rust_fn, tag)}` / `{py_method: (rust_fn, variant)}` if it does not already
expose one; the variant/tag columns already exist in the checklist data because the
generator emits the `ElementRef` registry from them). Then after the checklist block:

```python
    if args.emit_dispatch_tables:
        reader_methods, writer_methods = collect_ported_handlers()
        variants = parse_element_ref_variants(
            pathlib.Path(args.out) / "m2" / "autosar_templates" /
            "generic_structure" / "general_template_classes" / "ar_object.rs"
        )
        dispatch_tables.emit_dispatch_tables(
            reader_methods, writer_methods, variants, pathlib.Path(args.out)
        )
```

with `parse_element_ref_variants` reading the `ElementRef` enum's `Variant(Id)` lines (a
10-line regex scan — the enum lives at
`src/m2/autosar_templates/generic_structure/general_template_classes/ar_object.rs`).

- [ ] **Step 3: Declare the reader module** (the generated writer file also lands on disk
  but stays **undeclared** until Task 4's dyn-Write conversion — a generic fn cannot be a
  fn pointer):

```rust
// src/reader/arxml_reader/mod.rs — after the last `mod <domain>;` line
mod dispatch_tables;
```

- [ ] **Step 4: Add `use std::sync::LazyLock;`** to both `mod.rs` files (or rely on the
  generated file's `use super::*` if the parent already imports it — it does not yet, so
  add it to both parents).

- [ ] **Step 5: Regenerate and run the gate test**

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --emit-dispatch-tables
python3 -m pytest tools/py2rust/tests/test_dispatch_tables.py -v
```

Expected: gate tests PASS. The reader lookup fns are not called yet, so `cargo clippy`
flags dead code — add `#[allow(dead_code)]` on the two reader lookup items (removed in
Task 3 Step 5):

```rust
#[allow(dead_code)] // temporary — consumed by read_element_payload in P5 Task 3
```

- [ ] **Step 6: Commit**

```bash
git add tools/py2rust src/reader/arxml_reader/dispatch_tables.rs src/writer/arxml_writer/dispatch_tables.rs src/reader/arxml_reader/mod.rs src/writer/arxml_writer/mod.rs
git commit -m "feat(py2rust): --emit-dispatch-tables (P5 Task 0-2)"
```

### Task 3: Reader strangler migration — one domain per merge

The roadmap locks "strangler, one domain at a time" (§6), so the hand match shrinks arm by
arm with the 32-fixture harness green at every merge. The table-first probe coexists with
the shrinking hand match until the last arm moves.

**Files:**
- Modify: `src/reader/arxml_reader/mod.rs` (`read_element_payload`, ~line 316)

The current twelve arms, grouped into the nine migration units (domain modules with
dispatch arms):

| # | Unit | Arms moved |
|---|---|---|
| 1 | compu_method | `ElementRef::CompuMethod` |
| 2 | data_constr | `ElementRef::DataConstr` |
| 3 | keyword | `ElementRef::KeywordSet` |
| 4 | base_types | `ElementRef::SwBaseType` |
| 5 | unit | `ElementRef::Unit` |
| 6 | physical_dimension | `ElementRef::PhysicalDimension` |
| 7 | collection | `ElementRef::Collection` |
| 8 | life_cycle | `ElementRef::LifeCycleInfoSet` |
| 9 | datatypes | `ApplicationPrimitiveDataType`, `ApplicationArrayDataType`, `ApplicationRecordDataType`, `ImplementationDataType` |

- [ ] **Step 1: Insert the table-first probe ahead of the hand match**

```rust
fn read_element_payload(
    &mut self,
    element: &Node,
    element_ref: ElementRef,
    document: &mut Document,
) -> Result<(), ParseError> {
    if let Some(handler) = dispatch_tables::lookup_read_handler(element.name.as_str()) {
        return handler(self, element, element_ref, document);
    }
    match element_ref {
        // ... the twelve arms, unchanged for now ...
        _ => Ok(()),
    }
}
```

- [ ] **Step 2: Run the full harness** — same command block as Task 3 of the P2–P4 plan
  (`--check`, build, fmt, clippy, test). Expected: green (behavior-neutral: every tag the
  table covers was already an arm; the probe only reorders lookup).

- [ ] **Step 3: Move unit 1 (compu_method) and commit**

Delete the `ElementRef::CompuMethod(id) => self.read_compu_method(element, id, document),`
arm from the hand match (the table now serves that tag), then:

```bash
set -o pipefail
cargo build && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test
git add -A && git commit -m "refactor(p5): reader strangler — compu_method via dispatch table"
```

Expected: green.

- [ ] **Step 4: Repeat for units 2–9**, one commit each, message
  `refactor(p5): reader strangler — <unit> via dispatch table`. After unit 9 the hand match
  is `match element_ref { _ => Ok(()) }` — delete the whole match and reduce the seam to:

```rust
fn read_element_payload(
    &mut self,
    element: &Node,
    element_ref: ElementRef,
    document: &mut Document,
) -> Result<(), ParseError> {
    match dispatch_tables::lookup_read_handler(element.name.as_str()) {
        Some(handler) => handler(self, element, element_ref, document),
        None => Ok(()), // unknown/unported tag: warning-and-skip lives upstream
    }
}
```

- [ ] **Step 5: Remove the temporary `#[allow(dead_code)]`** from Task 2 Step 5 (lookups are
  now used). Harness once more, then:

```bash
git add -A && git commit -m "refactor(p5): reader dispatch fully table-driven"
```

### Task 4: Writer strangler migration — dyn Write, then one domain per merge

**Files:**
- Modify: `src/writer/arxml_writer/mod.rs` (`write_ar_package_element` ~line 315, `save()`)
- Modify: every `src/writer/arxml_writer/<domain>.rs` emitter (signature change only)

- [ ] **Step 1: Convert handler signatures to `&mut Writer<&mut dyn Write>`** (one commit;
  must precede any table use — generic fn pointers cannot be table entries)

Mechanical, compiler-guided: in every emitter, change

```rust
pub(super) fn write_compu_method<W: Write>(
    &self,
    writer: &mut Writer<W>,
    ...
```

to

```rust
pub(super) fn write_compu_method(
    &self,
    writer: &mut Writer<&mut dyn Write>,
    ...
```

At the entry point (`save()`, the one generic fn):

```rust
let mut dyn_writer: &mut dyn Write = &mut file;
let mut serializer = Writer::new(&mut dyn_writer);
```

`cargo build` errors list every call site; `writer.write_event(...)` calls need no change.
Harness green, then:

```bash
git add -A && git commit -m "refactor(p5): writer handlers on dyn Write (table prerequisite)"
```

- [ ] **Step 1b: Declare the writer table module + extend the gate test**

```rust
// src/writer/arxml_writer/mod.rs — after the last `mod <domain>;` line
mod dispatch_tables;
```

Append to `tools/py2rust/tests/test_dispatch_tables.py`:

```python
def test_writer_table_entries_carry_variant_index_and_handler(tmp_path):
    _, writer = _emit(tmp_path)
    assert "ElementRef::CompuMethod(_) =>" in writer   # discriminant arm
    assert "write_compu_method" in writer              # handler now fn-pointer-stable
```

```bash
python3 -m pytest tools/py2rust/tests/test_dispatch_tables.py -v
```

Expected: PASS. The writer lookup fns are not consumed until Step 2 — put the same
temporary `#[allow(dead_code)]` (with the same removal note) on `variant_index` and
`lookup_write_handler` in the **emitter template** (Task 2 Step 1's `WRITER_TEMPLATE`),
regenerate, and run the full harness:

```bash
set -o pipefail
cargo build && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test
git add -A && git commit -m "refactor(p5): writer dispatch table declared (dyn handlers)"
```

- [ ] **Step 2: Insert the table-first probe ahead of the hand match** (same coexistence
  pattern as reader Task 3 Step 1):

```rust
fn write_ar_package_element(
    &self,
    writer: &mut Writer<&mut dyn Write>,
    element_ref: ElementRef,
    document: &Document,
) -> Result<(), WriteError> {
    if let Some(handler) = dispatch_tables::variant_index(&element_ref)
        .and_then(dispatch_tables::lookup_write_handler)
    {
        return handler(self, writer, element_ref, document);
    }
    match element_ref {
        // ... the twelve arms, unchanged for now ...
        _ => {}
    }
    Ok(())
}
```

Harness green; commit `refactor(p5): writer table probe ahead of hand match`.

- [ ] **Step 3: Move the nine units one commit each** (same unit table as reader Task 3):
  delete the arm from the hand match, harness green, commit
  `refactor(p5): writer strangler — <unit> via dispatch table`. After the last unit, reduce
  the seam to the final shape:

```rust
fn write_ar_package_element(
    &self,
    writer: &mut Writer<&mut dyn Write>,
    element_ref: ElementRef,
    document: &Document,
) -> Result<(), WriteError> {
    match dispatch_tables::variant_index(&element_ref)
        .and_then(dispatch_tables::lookup_write_handler)
    {
        Some(handler) => handler(self, writer, element_ref, document),
        None => Ok(()), // unported variant: hand chain's fallback was `_ => {}`
    }
}
```

- [ ] **Step 4: Full harness green** — same command block. Commit:

```bash
git add -A && git commit -m "refactor(p5): writer dispatch fully table-driven"
```

### Task 5: Contract test — table invariants

The table items are `pub(crate)`, so the contract test must live **inside the generated
module** (in-crate visibility) — the generator emits a `#[cfg(test)]` block with the table.
This keeps the invariant checked on every `cargo test` run without widening the public API
(roadmap: public API unchanged).

**Files:**
- Modify: `tools/py2rust/dispatch_tables.py` (`READER_TEMPLATE` gains the test block)

- [ ] **Step 1: Add the test block to `READER_TEMPLATE`**

```python
READER_TEST_BLOCK = """
#[cfg(test)]
mod contract {{
    use super::*;

    /// P5 contract: the tag table stays sorted + duplicate-free — binary-search
    /// correctness and generator hygiene in one assertion.
    #[test]
    fn reader_table_entries_are_sorted_and_unique() {{
        let table = &*READ_TABLE;
        assert!(table.windows(2).all(|w| w[0].0 < w[1].0));
        assert!(READ_TABLE.len() > 0);
    }}
}}
"""
```

Append `READER_TEST_BLOCK` to the emitted reader file text in `emit_dispatch_tables`.
(Writer-side index coverage is already enforced by the generator's pytest gate — the index
arms are an exhaustive match emitted from the same variant list.)

- [ ] **Step 2: Extend the pytest gate** — add to
  `tools/py2rust/tests/test_dispatch_tables.py`:

```python
def test_reader_table_carries_contract_test(tmp_path):
    reader, _ = _emit(tmp_path)
    assert "mod contract" in reader
    assert "reader_table_entries_are_sorted_and_unique" in reader
```

- [ ] **Step 3: Regenerate, run, commit**

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --emit-dispatch-tables
python3 -m pytest tools/py2rust/tests/test_dispatch_tables.py -v
cargo test --lib dispatch
git add -A && git commit -m "test(p5): dispatch table contract gate in generated module"
```

Expected: all green (`reader_table_entries_are_sorted_and_unique` runs in the lib suite).

### Task 6: Phase exit — P5 done

- [ ] **Step 1: Full gate suite**

```bash
set -o pipefail
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check && \
python3 -m pytest tools/py2rust/tests -q && \
cargo build && cargo fmt --all -- --check && \
cargo clippy --all-targets -- -D warnings && cargo test
```

Expected: everything green; `grep -n "ElementRef::" src/reader/arxml_reader/mod.rs
src/writer/arxml_writer/mod.rs` returns no dispatch arms (only imports, if any).

- [ ] **Step 2: Update the roadmap status line** in
  `docs/superpowers/specs/2026-10-03-p2-p6-migration-roadmap-design.md` (§6): table-driven
  dispatch landed, both seams migrated, harness green at every merge.

- [ ] **Step 3: Commit**

```bash
git add -A && git commit -m "docs(p5): phase exit — dispatch is table-driven"
```
