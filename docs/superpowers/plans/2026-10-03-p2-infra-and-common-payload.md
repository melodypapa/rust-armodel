# P2 Infrastructure + Common Payload Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the P2 porting infrastructure (port checklist, zero-warnings gate, model-gap fixes) and port the shared `Identifiable` payload plus seven small element families, graduating 10 of the 32 fixtures to byte-identical round-trip.

**Architecture:** Hand-port py-armodel's reader/writer methods one-for-one into the existing `ARXMLParser`/`ARXMLWriter` (strategy A from the roadmap spec). Per-class payload dispatch extends the existing registry loop in `read_ar_package` / `write_ar_package`. Model gaps go through `tools/py2rust` (typemap/extractor/templates) + regeneration, never hand-edits to `src/m2/**`.

**Tech Stack:** Rust stable / edition 2021, quick-xml 0.42, slotmap, thiserror; Python 3 stdlib `ast` for the py2rust tool; pytest-style asserts live in `tools/py2rust/tests`.

**Spec:** `docs/superpowers/specs/2026-10-03-p2-p6-migration-roadmap-design.md` (§5). Execute on branch `p2-infra-and-common-payload` **after PR #12 merges**.

---

## Ground truth established by recon (2026-10-03)

These facts were verified against the pinned py-armodel (`target/py-armodel`, SHA in `tools/py2rust/PY_ARMODEL_VERSION`) and the generated tree. Trust them; re-verify only where a step says to.

1. **This plan's graduation set is 10 fixtures** (spec batch 2 minus the 9 that need the compu/data-constr/keyword/application-datatype families, which a later plan covers):
   `AUTOSAR_MOD_AISpecification_BaseTypes_Standard`, `AUTOSAR_MOD_AISpecification_Collection_{Body,Chassis,MmedTelmHmi,OccptPedSfty,Pt}_Blueprint`, `AUTOSAR_MOD_AISpecification_ApplicationDataType_LifeCycle_Standard`, `AUTOSAR_MOD_AISpecification_Keyword_LifeCycle_Standard`, `AUTOSAR_MOD_AISpecification_PhysicalDimension_Standard`, `AUTOSAR_MOD_AISpecification_Unit_Standard`.
2. **Writer element order is insertion order.** py `writeARPackageElements` iterates `pkg.getElements()` and picks the per-class writer from an isinstance chain; it does *not* group by type. The Rust writer's existing `for element_ref in elements` loop is already correct; per-class emitters slot into it.
3. **Attribute orders that matter for bytes** (verified in py writers):
   - Identifiable tag: `S`, `T`, `UUID` (py comment: "historical attribute order S, T, UUID"). Already matched by the Rust writer.
   - RefType child (`setChildElementOptionalRefType`): `BASE`, then `DEST`, then text value.
   - `L-2` (`setLOverviewParagraph`→`setLanguageSpecific`): `S`, `T` (via writeARObject), then `L`. `L-1` (`writeLParagraphs`): `S`, `T`, text, then `L`. `L-4` (`setLLongName`): `L` first, then text, then `S`/`T` via `writeMixedContentForLongName`. (No fixture carries S/T on these — port the order anyway.)
   - `MultiLanguageParagraph` (`<P>`): attribute `HELP-ENTRY`; children are `L-1`s.
4. **Attribute census over the 10 fixtures:** only `DEST`, `BASE`, `L`, the three namespace decls, XML declaration `version`/`encoding`, and exactly one `T=` (on an AR-PACKAGE, i.e. ARObject timestamp — already handled). No `S`/`T` on primitives, no `SHORT-LABEL` on numericals, no `SUP`/`SUB`/`E`/`IE`/`TT` inline content. The port reads/writes them where py does, but nothing in this batch exercises them beyond ARObject.
5. **py's round-trip bar** (`tests/integration_tests/test_roundtrip.py` step 5) is a line-level byte comparison with only two normalizations (`&quot;`/`&apos;` → characters, XML declaration/root folding). Rust reproduces it exactly with the existing `format_byte_roundtrip.rs` harness (byte-exact, stricter — allowed).
6. **py primitives:** `ARLiteral`/`Float`/`Numerical` carry a `T` attribute (`readARType`/`writeARType`) and `Numerical` a `SHORT-LABEL` attribute. The generated Rust model stores these values as `Option<String>` (no T/SHORT-LABEL). Safe for this batch per fact 4; recorded as a known model limitation for later batches.
7. **Model gaps found (root-caused):**
   - `RefType`-typed fields collapse to `Option<String>`/`Vec<String>` (e.g. `Unit.physical_dimension_ref`) — loses `BASE`/`DEST` on `<PHYSICAL-DIMENSION-REF DEST=... BASE=...>`. Fix: typemap emits the existing value struct `crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::RefType`.
   - `BaseType` has no `base_type_definition` — py assigns `self.baseTypeDefinition = BaseTypeDirectDefinition()` (a default-constructed, non-`None` field the extractor skips).
   - `ReferenceBase` is a pinned P0 placeholder in `tools/py2rust/templates.py` with no fields; py has `shortLabel`, `isDefault`, `isGlobal`, `baseIsThisPackage`, `globalInPackageRefs`, `globalElements`, `packageRef`.
8. **Extension points:** parser dispatch is `read_ar_package`'s `ELEMENTS` loop (`element_factory_for_tag` + common-part setters, `src/parser/arxml_parser.rs:349-376`); writer dispatch is `write_ar_package`'s `ELEMENTS` loop (`src/writer/arxml_parser.rs:307-333` — per-class emitters go where SHORT-NAME/CATEGORY are written today). Text-model arenas already exist on `Document` (`multilanguage_long_names`, `l_long_names`, `multi_language_overview_paragraphs`, `l_overview_paragraphs`, `multi_language_paragraphs`, `l_paragraphs`, `documentation_blocks`, `ar_lists`, `items`); insert directly (`document.<arena>.insert(item)`), the P0 pattern — no factory methods needed for non-package classes.
9. **Verification protocol for every family task:** `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` must be clean before the commit; regenerating the model requires `python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check` to pass afterwards.

## Execution notes

- Fixture files are **never** edited (AGENTS.md). All gates are lists in test files.
- `cargo test` through a pipe needs `set -o pipefail` (memory: p0-delivery-workflow).
- `slotmap` custom keys: `Document::default()` constructs arenas with `with_key()` — never call `SlotMap::new()` for them.
- Warnings must mirror py's wording where py logs one (e.g. `"Unsupported ARPackage <%s>"`, `"Unsupported Life Cycle Info <%s>"`).

---

### Task 0: py2rust model-gap fixes + regeneration

py's model has three things the generated Rust is missing (fact 7). Fix the tool, not the output.

**Files:**
- Modify: `tools/py2rust/typemap.py` (RefType field mapping)
- Modify: `tools/py2rust/extract.py` (default-constructed typed fields)
- Modify: `tools/py2rust/templates.py` (pinned `ReferenceBase`)
- Regenerate: `src/m2/**`
- Test: `tools/py2rust/tests/test_model_gaps.py` (new)

- [ ] **Step 1: Write the failing tool test**

Create `tools/py2rust/tests/test_model_gaps.py`:

```python
"""Regression tests for the P2 model-gap fixes (plan 2026-10-03, Task 0).

Each test names one generated-Rust fact that P2 families need and that the
pre-fix tool got wrong. They parse the generated tree, so run after a
regeneration.
"""
import pathlib
import sys

SRC = pathlib.Path(__file__).resolve().parents[3] / "src" / "m2"


def read(rel: str) -> str:
    return (SRC / rel).read_text()


def test_reftyped_fields_use_the_reftype_value_struct():
    # Unit.physical_dimension_ref must keep BASE/DEST (bytes on the wire),
    # so the field is the value struct, not a bare String.
    units = read("msr/asam_hdo/units.rs")
    assert "physical_dimension_ref: Option<RefType>" in units, (
        "Unit.physical_dimension_ref must be Option<RefType>; "
        "a String drops the DEST attribute and breaks byte round-trip"
    )


def test_base_type_carries_base_type_definition():
    base_types = read("msr/asam_hdo/base_types.rs")
    assert "base_type_definition" in base_types, (
        "BaseType must expose baseTypeDefinition (py assigns "
        "BaseTypeDirectDefinition() in __init__)"
    )


def test_reference_base_has_its_seven_fields():
    ar_package = read(
        "autosar_templates/generic_structure/general_template_classes/ar_package.rs"
    )
    for field in (
        "short_label",
        "is_default",
        "is_global",
        "base_is_this_package",
        "global_in_package_refs",
        "global_elements",
        "package_ref",
    ):
        assert field in ar_package, f"ReferenceBase is missing {field}"
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cd tools/py2rust && python3 -m pytest tests/test_model_gaps.py -v 2>&1 | tail -8`
Expected: 3 failures (each assertion missing).

- [ ] **Step 3: Fix the typemap (RefType → value struct)**

In `tools/py2rust/typemap.py`, find where field types resolve to Rust strings and add a class-name → Rust-type entry so a field whose Python type is `RefType` maps to the value struct instead of `String`. Locate the existing mapping table (the file already knows the name `RefType` at line 19 — that list is recognized class names; the *field mapping* is where `RefType` falls through to the default `String`). Add:

```python
# Value structs from PrimitiveTypes.py keep their attributes (BASE/DEST on
# the wire); collapsing them to String loses bytes. They are pure-value
# types with no arena, so they are stored by value.
VALUE_TYPE_FIELDS = {
    "RefType": "RefType",
}
```

…and in the function that renders a field's Rust type (returns `Option<String>` today for these), resolve through `VALUE_TYPE_FIELDS` first, producing `Option<RefType>` / `Vec<RefType>` per multiplicity, and make the emitter add the import
`use crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::RefType;`
to the generated file when the type is used (the emitter already collects imports — follow the pattern used for `XmlSpace`/`AdminData` imports).

- [ ] **Step 4: Fix the extractor (default-constructed fields)**

In `tools/py2rust/extract.py`, the field walk skips assignments whose value is not `None`. py's `BaseType.__init__` has `self.baseTypeDefinition: BaseTypeDirectDefinition = BaseTypeDirectDefinition()` — a typed, default-constructed field. Where assignments are classified, treat `ast.Call(func=ast.Name(id=T))` as a field of type `T` exactly like `None` defaults are treated (same Optionality rules — a default-constructed composite stays a plain required field in py, but the generated model uses `Option<...>` for referenced classes per code_guide §6, so emit `Option<TId>`/`Option<T>` like every other referenced-class field). Follow the existing `None`-default branch and reuse its IR construction; the only difference is the extracted type name.

- [ ] **Step 5: Fill in the pinned ReferenceBase**

`tools/py2rust/templates.py` pins the P0 `ReferenceBase` placeholder (comment block starting `//! ReferenceBase (P0 design §5)` around lines 394–716). Replace the pinned struct body with the py fields, keeping the pin (it stays hand-maintained like `Document`):

```rust
/// spec class `ReferenceBase` — `ReferenceBase : ARObject`
/// This meta-class establishes a basis for relative references. Reference
/// bases are identified by the short Label which shall be unique in the
/// current package.
#[derive(Debug, Default)]
pub struct ReferenceBase {
    base: ARObject,
    base_is_this_package: Option<bool>,
    global_elements: Vec<String>,
    global_in_package_refs: Vec<RefType>,
    is_default: Option<bool>,
    is_global: Option<bool>,
    package_ref: Option<RefType>,
    short_label: Option<String>,
}
```

with `new()`, `base()`/`base_mut()`, and `get_`/`set_` accessors for every field exactly in the generated style (setters take `impl Into<String>` for string fields, `RefType` by value, `bool` plain, vec fields get `push_<field>`; forwarded `get_checksum`/`set_checksum`/`get_timestamp`/`set_timestamp` to `base`). py stores `globalElements` as `ReferrableSubtypesEnum` literals — the model keeps `Vec<String>` for this batch (the enum is a string-valued literal; no fixture writes a Rust enum mismatch here and py's own reader stores `.value`).

- [ ] **Step 6: Regenerate and run the tool test**

Run:
```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src
cd tools/py2rust && python3 -m pytest tests/test_model_gaps.py -v 2>&1 | tail -5
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check && echo CHECK-OK
```
Expected: 3 passes; `CHECK-OK`.

- [ ] **Step 7: Make the crate compile again**

The new field types break nothing structurally, but clippy/dead-code may complain about unused accessors on regenerated types (generated files carry `#![allow(dead_code)]`; hand-written files must not). Run and fix fallout in hand-written code only:

Run: `cargo test 2>&1 | tail -5`
Expected: all tests pass (P0/P1 suites are model-level and unaffected by additive fields).

- [ ] **Step 8: Commit**

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test > /dev/null && echo GREEN
git add tools/py2rust src/m2
git commit -m "feat(py2rust): RefType value fields, default-constructed fields, full ReferenceBase (P2 Task 0)"
```

---

### Task 1: py2rust `--emit-port-checklist`

The progress checklist required by roadmap spec §5.2: every py `readXxx`/`writeXxx` method, its snake_case Rust name, and a status column derived by scanning `src/`.

**Files:**
- Modify: `tools/py2rust/main.py` (new flag), `tools/py2rust/extract.py` or new `tools/py2rust/port_checklist.py` (AST walk)
- Create: `docs/port_checklist.md` (generated, committed)
- Test: `tools/py2rust/tests/test_port_checklist.py`

- [ ] **Step 1: Write the failing tool test**

Create `tools/py2rust/tests/test_port_checklist.py`:

```python
"""Tests for --emit-port-checklist (plan 2026-10-03, Task 1)."""
import ast
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))

from py2rust.port_checklist import collect_py_methods, rust_name, check_ported

PY_ROOT = pathlib.Path(__file__).resolve().parents[3] / "target" / "py-armodel" / "src" / "armodel"
SRC_ROOT = pathlib.Path(__file__).resolve().parents[3] / "src"


def test_collect_finds_reader_and_writer_methods():
    methods = collect_py_methods(PY_ROOT)
    names = [name for name, _ in methods]
    assert "readCompuMethod" in names
    assert "writeCompuMethod" in names
    assert "readAdminData" in names
    # non reader/writer methods are excluded
    assert "load" not in names
    assert "getChildElementOptionalLiteral" not in names


def test_snake_case_mapping():
    assert rust_name("readCompuMethod") == "read_compu_method"
    assert rust_name("readARPackage") == "read_ar_package"
    assert rust_name("writeSwBaseType") == "write_sw_base_type"


def test_status_scan_sees_ported_methods(tmp_path):
    (tmp_path / "lib.rs").write_text("mod parser { fn read_admin_data() {} }\n")
    ported, total = check_ported("readAdminData", tmp_path), True
    assert ported
```

- [ ] **Step 2: Run it to verify it fails**

Run: `cd tools/py2rust && python3 -m pytest tests/test_port_checklist.py -v 2>&1 | tail -5`
Expected: ImportError (`port_checklist` does not exist).

- [ ] **Step 3: Implement `tools/py2rust/port_checklist.py`**

```python
"""Emit docs/port_checklist.md — the P2-P4 progress checklist (spec §5.2).

Deliberately dumb: a method is "ported" iff a `fn <snake_case>` exists under
src/. Correctness is the harness's job, not this file's.
"""
import ast
import pathlib
import re


def collect_py_methods(py_root: pathlib.Path):
    """Yield (name, (file, line)) for every def read*/write* in the parser and
    writer packages, skipping the abstract helper classes."""
    results = []
    for rel in ("parser/arxml_parser.py", "writer/arxml_writer.py"):
        path = py_root / rel
        tree = ast.parse(path.read_text())
        for node in ast.walk(tree):
            if isinstance(node, ast.FunctionDef) and re.match(r"^(read|write)[A-Z]", node.name):
                results.append((node.name, (rel, node.lineno)))
    return results


def rust_name(py_name: str) -> str:
    out = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", py_name)
    out = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", out)
    return out.lower()


def fn_exists(rust_fn: str, src_root: pathlib.Path) -> bool:
    pattern = re.compile(rf"\bfn {re.escape(rust_fn)}\b")
    return any(
        pattern.search(p.read_text(errors="ignore"))
        for p in src_root.rglob("*.rs")
        if "/target/" not in str(p)
    )


def check_ported(py_name: str, src_root: pathlib.Path) -> bool:
    return fn_exists(rust_name(py_name), src_root)


def render(methods, src_root: pathlib.Path) -> str:
    lines = [
        "<!-- @generated by tools/py2rust --emit-port-checklist. Do not edit. -->",
        "# Port checklist (py reader/writer methods → Rust)",
        "",
        "| py method | Rust name | ported | py location |",
        "|---|---|---|---|",
    ]
    for name, (rel, lineno) in sorted(methods):
        done = "x" if check_ported(name, src_root) else " "
        lines.append(f"| `{name}` | `{rust_name(name)}` | [{done}] | `{rel}:{lineno}` |")
    ported = sum(1 for name, _ in methods if check_ported(name, src_root))
    lines += ["", f"**{ported}/{len(methods)} ported.**", ""]
    return "\n".join(lines)
```

- [ ] **Step 4: Wire the flag into `tools/py2rust/main.py`**

Add an argparse flag `--emit-port-checklist` (the CLI already uses argparse for `--py-armodel/--out/--check`). When set, after (or instead of) model generation:

```python
if args.emit_port_checklist:
    from py2rust.port_checklist import collect_py_methods, render

    methods = collect_py_methods(py_armodel / "src" / "armodel")
    out_root = args.out if args.out else pathlib.Path("src")
    doc_root = out_root.parent / "docs"
    (doc_root / "port_checklist.md").write_text(render(methods, out_root))
```

(adapt names to the existing main.py structure — the parser already computes `py_armodel` and `out`.)

- [ ] **Step 5: Run tests, generate, commit**

Run:
```bash
cd tools/py2rust && python3 -m pytest tests/ -v 2>&1 | tail -5
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --emit-port-checklist
head -12 docs/port_checklist.md
```
Expected: tests pass; the checklist renders with `readAdminData` marked `[x]` (P0 ported it as `read_admin_data`) and `readCompuMethod` `[ ]`.

Commit:
```bash
git add tools/py2rust docs/port_checklist.md
git commit -m "feat(py2rust): --emit-port-checklist progress report (P2 Task 1)"
```

---

### Task 2: zero-warnings harness gate

All-fixtures gains the parser-coverage gate (roadmap spec §5.3): a fixture is listed in `WARNING_FREE_SOURCES` exactly when the parser consumes every element it uses. Format mirrors `FORMAT_SOURCES` in `format_byte_roundtrip.rs`.

**Files:**
- Modify: `tests/integration/all_fixtures.rs`
- Modify: `Cargo.toml` only if `all_fixtures` lacks a `[[test]]` target (it already has one — verify, don't add)

- [ ] **Step 1: Write the failing test change**

In `tests/integration/all_fixtures.rs`, next to `FIXTURE_COUNT`, add the list:

```rust
/// Fixtures whose parse must produce zero warnings — i.e. the parser
/// consumes every element they contain. A fixture joins this list exactly
/// when its batch lands (roadmap spec §5.3); byte-identity additionally
/// requires a FORMAT_SOURCES entry in format_byte_roundtrip.rs.
const WARNING_FREE_SOURCES: &[&str] = &["AdminDataWhitespace.arxml"];
```

Leave the existing `round_trip` helper and model-equality test untouched (they cover all 32 and must not grow a warnings dependency). Then add a second test function after the existing one:

```rust
/// Fixtures listed in WARNING_FREE_SOURCES must parse without a single
/// "unsupported element"-class warning. Everything else still round-trips
/// at model level via the warnings-and-skip path.
#[test]
fn warning_free_sources_parse_without_warnings() {
    for source in WARNING_FREE_SOURCES {
        let path = std::path::Path::new(FIXTURE_DIR).join(source);
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let mut document = Document::new();
        let mut parser = ARXMLParser::new(default_options());
        parser
            .load(&path, &mut document)
            .unwrap_or_else(|error| panic!("{name}: parse failed: {error}"));
        assert!(
            parser.get_warnings().is_empty(),
            "{name}: expected zero warnings, got: {:?}",
            parser.get_warnings()
        );
    }
}
```

(Keep the existing model-equality test untouched; it covers all 32.)

- [ ] **Step 2: Run**

Run: `set -o pipefail; cargo test --test all_fixtures 2>&1 | tail -5`
Expected: PASS (AdminDataWhitespace already parses clean).

- [ ] **Step 3: Commit**

```bash
git add tests/integration/all_fixtures.rs
git commit -m "test(harness): WARNING_FREE_SOURCES zero-warnings gate (P2 Task 2)"
```

---

### Task 3: the Identifiable payload chain (LONG-NAME / DESC / INTRODUCTION)

py reads these in `readIdentifiable` (`arxml_parser.py:1762`) via the base chain, and writes them in `writeIdentifiable` (`arxml_writer.py:1874`). This task ports the chain and the text model beneath it, and wires both dispatch sites. No fixture graduates yet — everything needs one more family.

**Files:**
- Modify: `src/parser/arxml_parser.rs` (chain methods + ELEMENTS dispatch), `src/parser/abstract_arxml_parser.rs` (ref-type/bool/positive-int helpers)
- Modify: `src/writer/arxml_writer.rs` (chain emitters + per-element dispatch), `src/writer/abstract_arxml_writer.rs` (text emitters if needed)

**py methods being ported (read side):** `readReferrable` 1738, `readMultilanguageReferrable` 1745, `readIdentifiable` 1762; helpers `getMultilanguageLongName` 2001, `readLLongName` 1796, `readMixedContentForLongName` 1805, `getMultiLanguageOverviewParagraph` 2104, `readLOverviewParagraph` 2092, `getDocumentationBlock` 6908, `readDocumentationBlock` 6891, `getMultiLanguageParagraphs` 6298, `getLParagraphs` 6286, `getListElements` 6320, `readPaginateable` 6631, `readDocumentViewSelectable` (see abstract parser), `readMixedStringText` 13743.
**Write side:** `writeReferrable` 1507, `writeMultilanguageReferrable` 1788, `writeIdentifiable` 1874, `setMultiLongName` 1768, `setLLongName` 1553, `writeMixedContentForLongName` 1558, `setMultiLanguageOverviewParagraph` 1781, `setLOverviewParagraph` 1775, `setLanguageSpecific` 1538, `writeMixedContentForOverviewParagraph` 1697, `writeDocumentationBlock` 3161, `writeDocumentationBlockContent` 3166, `setMultiLanguageParagraphs` 2847, `writeLParagraphs` 2837, `setListElement` 2856, `writePaginateable` 3137.

**Model note:** `LLongName`/`LOverviewParagraph` embed both a MixedContent base and a `LanguageSpecific` (`l` + `value`); py's reader order sets value/L first, then mixed content (SUP/SUB attrs, E/IE/TT children — none occur in this batch, but port the loop). Verify exact vec-accessor names before compiling (they follow the `push_<field>` convention — e.g. `push_l4`, `push_l2`, `push_l1`, `push_ps`, `push_items`):

```bash
grep -n "pub fn push_" src/m2/msr/documentation/text_model/multilanguage_data.rs src/m2/msr/documentation/text_model/block_elements.rs src/m2/msr/documentation/block_elements/list_elements.rs
```

- [ ] **Step 1: abstract parser helpers**

Append to `src/parser/abstract_arxml_parser.rs` (public fns, mirroring py's names):

```rust
/// py getChildElementOptionalBooleanValue — text "true" → true, anything
/// else false; empty/absent → None.
pub fn get_child_element_optional_boolean(element: &Node, name: &str) -> Option<bool> {
    let text = get_child_element_string(element, name)?;
    if text.is_empty() { None } else { Some(text == "true") }
}

/// py getChildElementOptionalRefType / _getChildElementRefTypeDestAndValue.
pub fn get_child_element_optional_ref_type(element: &Node, name: &str) -> Option<RefType> {
    let child = find(element, name)?;
    Some(get_ref_type_dest_and_value(child))
}

/// py getChildElementRefTypeList — `key` may be a nested path like
/// "ELEMENT-REFS/ELEMENT-REF" (find_all splits on '/').
pub fn get_child_element_ref_type_list(element: &Node, key: &str) -> Vec<RefType> {
    find_all(element, key)
        .into_iter()
        .map(get_ref_type_dest_and_value)
        .collect()
}

fn get_ref_type_dest_and_value(element: &Node) -> RefType {
    let mut r#ref = RefType::new();
    if let Some(base) = element.attrs.get("BASE") {
        r#ref.set_base(base.as_str());
    }
    if let Some(dest) = element.attrs.get("DEST") {
        r#ref.set_dest(dest.as_str());
    }
    if let Some(text) = &element.text {
        r#ref.set_value(text.as_str());
    }
    r#ref
}
```

(If `find_all` does not yet split `A/B` paths, extend it the same way py's `findall` does — split on `/`, walk segments. Check its current signature first; `read_admin_data` already calls `find_all(sdgs_node, "*")`.)

Import `RefType` from `crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::RefType`.

- [ ] **Step 2: reader chain in arxml_parser.rs**

Add methods to `impl ARXMLParser` (bodies mirror the py methods by line number):

```rust
/// py getMultilanguageLongName — LONG-NAME wrapper + L-4 entries.
fn get_multilanguage_long_name(
    &mut self,
    element: &Node,
    document: &mut Document,
) -> Result<Option<MultilanguageLongNameId>, ParseError> {
    let Some(long_name_node) = find(element, "LONG-NAME") else {
        return Ok(None);
    };
    let mut long_name = MultilanguageLongName::new();
    self.read_ar_object(long_name_node, long_name.base_mut());
    // py readLLongName
    for l4_node in find_all(long_name_node, "L-4") {
        let mut l4 = LLongName::new();
        // py readMixedContentForLongName reads the L-4's ARObject (S/T) —
        // two hops: LLongName.base = MixedContentForLongName, whose base is
        // ARObject.
        self.read_ar_object(l4_node, l4.base_mut().base_mut());
        if let Some(text) = &l4_node.text {
            l4.set_value(text.as_str());
        }
        if let Some(l) = l4_node.attrs.get("L") {
            l4.set_l(l.as_str());
        }
        // SUP/SUB live on the mixed-content base (verify the forwarding
        // accessor with the grep below; if it is not forwarded, call
        // l4.base_mut().set_sup(...) on the MixedContentForLongName field).
        if let Some(sup) = l4_node.attrs.get("SUP") {
            l4.base_mut().set_sup(sup.as_str());
        }
        if let Some(sub) = l4_node.attrs.get("SUB") {
            l4.base_mut().set_sub(sub.as_str());
        }
        for inline in &l4_node.children {
            match inline.name.as_str() {
                "E" | "IE" | "TT" => {
                    let message =
                        format!("Unsupported inline element <{}> in L-4", inline.name);
                    self.not_implemented(message)?;
                }
                _ => {}
            }
        }
        let l4_id = document.l_long_names.insert(l4);
        long_name.push_l4(l4_id);
    }
    Ok(Some(document.multilanguage_long_names.insert(long_name)))
}
```

**Deviation to resolve before writing this method:** the generated `LLongName` embeds `MixedContentForLongName` (which may carry `sup`/`sub`/`e`/`ie`/`tt` fields) *plus* `LanguageSpecific`. Check the actual generated accessor names:

```bash
grep -n "pub fn get_\|pub fn set_\|pub fn push_" src/m2/msr/documentation/text_model/language_data_model.rs | head -40
```

Write the body against those names. If `LLongName` has no `sup`/`sub` fields, drop those two `if let`s (the model cannot hold them — record in the task summary as a checklist note, do not hand-edit `src/m2`), and keep the E/IE/TT warning loop.

Then the overview-paragraph and documentation-block readers, same pattern:

```rust
/// py getMultiLanguageOverviewParagraph / readLOverviewParagraph.
fn get_multi_language_overview_paragraph(
    &mut self,
    element: &Node,
    document: &mut Document,
) -> Result<Option<MultiLanguageOverviewParagraphId>, ParseError> {
    let Some(desc_node) = find(element, "DESC") else {
        return Ok(None);
    };
    let mut paragraph = MultiLanguageOverviewParagraph::new();
    self.read_ar_object(desc_node, paragraph.base_mut());
    for l2_node in find_all(desc_node, "L-2") {
        let mut l2 = LOverviewParagraph::new();
        self.read_ar_object(l2_node, l2.base_mut().base_mut()); // MixedContent base is ARObject-bearing
        if let Some(text) = &l2_node.text {
            l2.set_value(text.as_str());
        }
        if let Some(l) = l2_node.attrs.get("L") {
            l2.set_l(l.as_str());
        }
        if let Some(bp) = l2_node.attrs.get("BLUEPRINT-VALUE") {
            l2.set_blueprint_value(bp.as_str());
        }
        let l2_id = document.l_overview_paragraphs.insert(l2);
        paragraph.push_l2(l2_id);
    }
    Ok(Some(document.multi_language_overview_paragraphs.insert(paragraph)))
}

/// py getDocumentationBlock / readDocumentationBlock — P + LIST (+ ITEM
/// recursion). The other block kinds (DEF-LIST, FORMULA, …) do not occur in
/// any pinned fixture; py only reads them when present, so their absence
/// here is behavior-identical. The port-checklist tracks them.
fn get_documentation_block(
    &mut self,
    element: &Node,
    document: &mut Document,
) -> Result<Option<DocumentationBlockId>, ParseError> {
    let Some(block_node) = find(element, "INTRODUCTION") else {
        return Ok(None);
    };
    let block = self.read_documentation_block(block_node, document)?;
    Ok(Some(block))
}

fn read_documentation_block(
    &mut self,
    element: &Node,
    document: &mut Document,
) -> Result<DocumentationBlockId, ParseError> {
    let mut block = DocumentationBlock::new();
    self.read_ar_object(element, block.base_mut());
    // py getMultiLanguageParagraphs(element, "P")
    for p_node in find_all(element, "P") {
        let mut paragraph = MultiLanguageParagraph::new();
        self.read_paginateable(p_node, paragraph.base_mut())?;
        if let Some(help) = p_node.attrs.get("HELP-ENTRY") {
            paragraph.set_help_entry(help.as_str());
        }
        // py getLParagraphs(child_element, "L-1")
        for l1_node in find_all(p_node, "L-1") {
            let mut l1 = LParagraph::new();
            self.read_ar_object(l1_node, l1.base_mut())?;
            if let Some(text) = &l1_node.text {
                l1.set_value(text.as_str());
            }
            if let Some(l) = l1_node.attrs.get("L") {
                l1.set_l(l.as_str());
            }
            let l1_id = document.l_paragraphs.insert(l1);
            paragraph.push_l1(l1_id);
        }
        let paragraph_id = document.multi_language_paragraphs.insert(paragraph);
        block.push_ps(paragraph_id);
    }
    // py getListElements(element, "LIST") — recursive DocumentationBlock in ITEM.
    for list_node in find_all(element, "LIST") {
        let mut list = ARList::new();
        self.read_paginateable(list_node, list.base_mut())?;
        if let Some(t) = list_node.attrs.get("TYPE") {
            match ListEnum::try_from(t.as_str()) {
                Ok(value) => {
                    list.set_type(value);
                }
                Err(_) => {
                    let message = format!("Unsupported LIST TYPE <{t}>");
                    self.not_implemented(message)?;
                }
            }
        }
        for item_node in find_all(list_node, "ITEM") {
            let mut item = Item::new();
            self.read_paginateable(item_node, item.base_mut())?;
            let contents = self.read_documentation_block(item_node, document)?;
            item.set_item_contents(contents);
            let item_id = document.items.insert(item);
            list.push_items(item_id);
        }
        let list_id = document.ar_lists.insert(list);
        block.push_lists(list_id);
    }
    Ok(document.documentation_blocks.insert(block))
}

/// py readPaginateable — BREAK / KEEP-WITH-PREVIOUS attributes.
fn read_paginateable(&mut self, element: &Node, base: &mut Paginateable) -> Result<(), ParseError> {
    if let Some(break_attr) = element.attrs.get("BREAK") {
        match ChapterEnumBreak::try_from(break_attr.as_str()) {
            Ok(value) => {
                base.set_break(value);
            }
            Err(_) => {
                let message = format!("Unsupported BREAK <{break_attr}>");
                self.not_implemented(message)?;
            }
        }
    }
    if let Some(keep) = element.attrs.get("KEEP-WITH-PREVIOUS") {
        match KeepWithPreviousEnum::try_from(keep.as_str()) {
            Ok(value) => {
                base.set_keep_with_previous(value);
            }
            Err(_) => {
                let message = format!("Unsupported KEEP-WITH-PREVIOUS <{keep}>");
                self.not_implemented(message)?;
            }
        }
    }
    Ok(())
}
```

`Paginateable` lives in `src/m2/msr/documentation/block_elements/pagination_and_view.rs`; `LParagraph`, `ListEnum`, `ChapterEnumBreak`, `KeepWithPreviousEnum` — confirm exact names/paths with:

```bash
grep -rn "pub struct LParagraph\|pub enum ListEnum\|pub enum ChapterEnumBreak\|pub enum KeepWithPreviousEnum" src/m2 | head
```

If an enum doesn't exist (the generator may have kept it `Option<String>`), use the generated type — never invent one. If `ListEnum` is `Option<String>`-typed in `ARList` (`r#type: Option<ListEnum>` per recon — it IS an enum), port `TryFrom` from its generated impl.

Then `read_identifiable`, and the chain entry:

```rust
/// py readMultilanguageReferrable — LONG-NAME (SHORT-NAME-FRAGMENTS do not
/// occur in any pinned fixture; py only reads them when present).
fn read_multilanguage_referrable(
    &mut self,
    element: &Node,
    document: &mut Document,
    identifiable: &mut impl IdentifiableAccess,
) -> Result<(), ParseError> {
    if let Some(long_name) = self.get_multilanguage_long_name(element, document)? {
        identifiable.set_long_name(long_name);
    }
    Ok(())
}
```

Because generated classes don't share a trait for the Identifiable payload, **do not invent one** (code_guide §3: traits only for shared behaviour across unrelated types). Instead give each family's read method its own inline sequence: every family reader (Task 4+) calls, in order:

```rust
self.read_identifiable_payload(element, document, &mut |document, payload| {
    // per-class closure writes payload fields into the arena entry
})?;
```

Concretely, use this shape (no trait): each family reader allocates its struct, then calls one shared method that fills a `IdentifiablePayload` struct, then copies the pieces in:

```rust
/// The Identifiable-owned XML payload, filled by read_identifiable_payload.
struct IdentifiablePayload {
    long_name: Option<MultilanguageLongNameId>,
    desc: Option<MultiLanguageOverviewParagraphId>,
    introduction: Option<DocumentationBlockId>,
    admin_data: Option<AdminDataId>,
}

impl ARXMLParser {
    /// py readIdentifiable — everything under the MultilanguageReferrable
    /// chain except SHORT-NAME/UUID/CATEGORY (already read by callers) and
    /// annotations (no pinned fixture carries ANNOTATIONS; tracked on the
    /// port checklist).
    fn read_identifiable_payload(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<IdentifiablePayload, ParseError> {
        Ok(IdentifiablePayload {
            long_name: self.get_multilanguage_long_name(element, document)?,
            desc: self.get_multi_language_overview_paragraph(element, document)?,
            introduction: self.get_documentation_block(element, document)?,
            admin_data: match find(element, "ADMIN-DATA") {
                Some(node) => Some(self.read_admin_data(node, document)?),
                None => None,
            },
        })
    }
}
```

- [ ] **Step 3: wire the ELEMENTS dispatch (reader)**

In `read_ar_package`'s `ELEMENTS` loop, after the common-part setters, call a payload dispatcher and implement it as a match that grows one arm per family (Tasks 4–9). Initial version with the two kinds this task can already fill (none — so start with a no-op match to establish the seam):

```rust
                // py readARPackageElements dispatches per tag; the common
                // Identifiable parts are above, per-class payload here.
                self.read_element_payload(child, element_ref, document)?;
```

with:

```rust
    /// py's per-class read dispatch (the tag→create+read chain in
    /// readARPackageElements). Grows one arm per ported family.
    fn read_element_payload(
        &mut self,
        element: &Node,
        element_ref: ElementRef,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        match element_ref {
            // Arms land with Tasks 4-9; the wildcard keeps unported families
            // on the warnings-and-skip path.
            _ => Ok(()),
        }
    }
```

(`ARObject`/`ElementRef` are already imported in this file.)

- [ ] **Step 4: writer chain**

In `src/writer/arxml_writer.rs` add the emitters (order matters — see ground truth fact 3) and replace the ELEMENTS-loop body's per-element section with a dispatcher:

```rust
    /// py setMultiLongName / setLLongName.
    fn set_multi_long_name<W: Write>(
        &self,
        writer: &mut Writer<W>,
        long_name: &MultilanguageLongName,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("LONG-NAME");
        self.write_ar_object_attributes(&mut element, long_name.base());
        writer.write_event(Event::Start(element))?;
        for l4_id in long_name.get_l4() {
            if let Some(l4) = document.l_long_names.get(*l4_id) {
                // py setLLongName: L attr, text, then S/T (mixed content).
                let mut l4_element = BytesStart::new("L-4");
                if let Some(l) = l4.get_l() {
                    l4_element.push_attribute(("L", l));
                }
                if let Some(sup) = l4.get_sup() {
                    l4_element.push_attribute(("SUP", sup));
                }
                if let Some(sub) = l4.get_sub() {
                    l4_element.push_attribute(("SUB", sub));
                }
                // S/T last (py: writeMixedContentForLongName runs after L and
                // the text are set) — two hops to the ARObject base.
                self.write_ar_object_attributes(&mut l4_element, l4.base().base());
                write_text_element(writer, "L-4", l4_element, l4.get_value())?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("LONG-NAME")))?;
        Ok(())
    }
```

(Same rule as the reader: write against the actual generated accessors — `get_l4()`/`get_l2()`/`get_l1()`/`get_ps()`/`get_lists()`/`get_items()` naming per the grep in Step 2; if the model lacks `sup`/`sub`, omit those lines.)

Then `set_multi_language_overview_paragraph` (DESC wrapper + L-2s: S/T then L on each L-2 — py `setLanguageSpecific` writes ARObject attrs first), `write_documentation_block`/`write_documentation_block_content` (P with HELP-ENTRY attr, L-1 children: S/T, text, then L; LIST with TYPE upper-cased: `type.to_string().to_uppercase()` per py `type.getValue().upper()`; ITEM recursion via `write_documentation_block_content`), and the chain entry:

```rust
    /// py writeIdentifiable payload — LONG-NAME, DESC, INTRODUCTION,
    /// ADMIN-DATA. SHORT-NAME/CATEGORY/UUID are written by callers.
    fn write_identifiable_payload<W: Write>(
        &self,
        writer: &mut Writer<W>,
        /* per-class accessor closures, see below */
    ) -> Result<(), WriteError> { ... }
```

Because the writer needs per-class *getters*, implement the chain as one private method per family call-site instead of a generic one: `write_identifiable_parts(writer, long_name: Option<&MultilanguageLongName>, desc: Option<&MultiLanguageOverviewParagraph>, introduction: Option<&DocumentationBlock>, admin_data: Option<&AdminData>, document)` — each family emitter (Tasks 4–9) resolves its own ids through `document` and calls it after writing SHORT-NAME/CATEGORY. Then in `write_ar_package`'s ELEMENTS loop replace the common SHORT-NAME/CATEGORY tail with a dispatcher:

```rust
                self.write_ar_package_element(writer, element_ref, document)?;
```

```rust
    /// py writeARPackageElement's isinstance chain. Grows one arm per ported
    /// family; the wildcard keeps unported families on the P0 shape
    /// (SHORT-NAME + CATEGORY only).
    fn write_ar_package_element<W: Write>(
        &self,
        writer: &mut Writer<W>,
        element_ref: ElementRef,
        document: &Document,
    ) -> Result<(), WriteError> {
        match element_ref {
            _ => {
                // P0 shape — replaced arm by arm in Tasks 4-9. This is the
                // ELEMENTS-loop body moved verbatim from write_ar_package
                // (src/writer/arxml_writer.rs:307-333): the S/T/UUID
                // attribute block, SHORT-NAME, CATEGORY, Event::End.
                let tag = element_registry::element_tag(&element_ref);
                let mut element = BytesStart::new(tag);
                if let Some(checksum) = element_registry::element_checksum(document, element_ref) {
                    element.push_attribute(("S", checksum));
                }
                if let Some(timestamp) = element_registry::element_timestamp(document, element_ref) {
                    element.push_attribute(("T", timestamp));
                }
                if let Some(uuid) = element_registry::element_uuid(document, element_ref) {
                    element.push_attribute(("UUID", uuid));
                }
                writer.write_event(Event::Start(element))?;
                let short_name = element_registry::element_short_name(document, element_ref);
                let short_name_element = BytesStart::new("SHORT-NAME");
                write_text_element(writer, "SHORT-NAME", short_name_element, short_name)?;
                if let Some(category) = element_registry::element_category(document, element_ref) {
                    let category_element = BytesStart::new("CATEGORY");
                    write_text_element(writer, "CATEGORY", category_element, Some(category))?;
                }
                writer.write_event(Event::End(BytesEnd::new(tag)))?;
                Ok(())
            }
        }
    }
```

(Delete the moved body from the `write_ar_package` ELEMENTS loop, leaving the loop a pure `write_ar_package_element` call.)

- [ ] **Step 5: run, fmt, clippy, commit**

Run: `set -o pipefail; cargo test 2>&1 | tail -6`
Expected: all previous tests still pass (the new helpers are not yet exercised by a dispatch arm — Task 4 adds the first one and its unit test).

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings
git add src/parser src/writer
git commit -m "feat(p2): Identifiable payload chain — LONG-NAME/DESC/INTRODUCTION read+write (P2 Task 3)"
```

---

### Task 4: abstract writer helpers + SwBaseType → graduates `BaseTypes_Standard`

**Files:**
- Modify: `src/writer/abstract_arxml_writer.rs` (optional-element helpers)
- Modify: `src/parser/arxml_parser.rs`, `src/writer/arxml_writer.rs` (dispatch arms)

**py methods:** `readSwBaseType` (arxml_parser.py:7205) + `readBaseTypeDirectDefinition` (7198); `writeSwBaseType` (arxml_writer.py:3603) + `setBaseTypeDirectDefinition` (mirror — verify with `grep -n "def setBaseTypeDirectDefinition" -A 8 target/py-armodel/src/armodel/writer/arxml_writer.py`; it writes the same five children in the same order via the optional setters).

- [ ] **Step 1: writer optional-element helpers**

Append to `src/writer/abstract_arxml_writer.rs`:

```rust
use quick_xml::events::{BytesStart, Event};

use crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::RefType;
use crate::writer::arxml_writer::WriteError;

pub(crate) fn write_optional_text_element<W: std::io::Write>(
    writer: &mut quick_xml::writer::Writer<W>,
    tag: &str,
    value: Option<&str>,
) -> Result<(), WriteError> {
    // py setChildElementOptionalLiteral — nothing is emitted when None.
    if let Some(value) = value {
        write_text_element(writer, tag, BytesStart::new(tag), Some(value))?;
    }
    Ok(())
}

pub(crate) fn write_optional_boolean_element<W: std::io::Write>(
    writer: &mut quick_xml::writer::Writer<W>,
    tag: &str,
    value: Option<bool>,
) -> Result<(), WriteError> {
    // py setChildElementOptionalBooleanValue — text is the Boolean's text
    // ("true"/"false" as stored by the reader's literal text).
    match value {
        Some(true) => write_optional_text_element(writer, tag, Some("true")),
        Some(false) => write_optional_text_element(writer, tag, Some("false")),
        None => Ok(()),
    }
}

pub(crate) fn write_optional_ref_type<W: std::io::Write>(
    writer: &mut quick_xml::writer::Writer<W>,
    tag: &str,
    r#ref: Option<&RefType>,
) -> Result<(), WriteError> {
    // py setChildElementOptionalRefType — BASE, then DEST, then text.
    if let Some(r#ref) = r#ref {
        let mut element = BytesStart::new(tag);
        if let Some(base) = r#ref.get_base() {
            element.push_attribute(("BASE", base));
        }
        if let Some(dest) = r#ref.get_dest() {
            element.push_attribute(("DEST", dest));
        }
        write_text_element(writer, tag, element, r#ref.get_value())?;
    }
    Ok(())
}

pub(crate) fn write_ref_type_list<W: std::io::Write>(
    writer: &mut quick_xml::writer::Writer<W>,
    wrapper: &str,
    tag: &str,
    refs: &[RefType],
) -> Result<(), WriteError> {
    if refs.is_empty() {
        return Ok(());
    }
    writer.write_event(Event::Start(BytesStart::new(wrapper)))?;
    for r#ref in refs {
        write_optional_ref_type(writer, tag, Some(r#ref))?;
    }
    writer.write_event(Event::End(quick_xml::events::BytesEnd::new(wrapper)))?;
    Ok(())
}
```

(Import `BytesEnd` alongside `BytesStart`/`Event` at the top of the file, matching `arxml_writer.rs`'s existing style.)

The boolean asymmetry: py's reader stores the *text* ("true"/"false") on a Boolean; the model stores `Option<bool>`; `writeOptionalBooleanValue` writes `value.getText()` which for `Boolean.setValue("true")` is `"true"`. "false" text: py `_convertStringToBooleanValue` maps anything not `"true"` to `False`, and the Boolean writer's `getText()` returns the stored string — fixtures only carry `true`/`false`; verify with `grep -c ">false<" tests/integration/test_files/*.arxml | grep -v :0` and if any `false` exists, switch the model field handling to round-trip the text (notes section, Task 10).

- [ ] **Step 2: reader arm**

In `arxml_parser.rs` add:

```rust
    /// py readBaseTypeDirectDefinition.
    fn read_base_type_direct_definition(
        &mut self,
        element: &Node,
        definition: &mut BaseTypeDirectDefinition,
    ) -> Result<(), ParseError> {
        if let Some(size) = get_child_element_string(element, "BASE-TYPE-SIZE") {
            definition.set_base_type_size(size);
        }
        if let Some(encoding) = get_child_element_string(element, "BASE-TYPE-ENCODING") {
            definition.set_base_type_encoding(encoding);
        }
        if let Some(alignment) = get_child_element_string(element, "MEM-ALIGNMENT") {
            definition.set_mem_alignment(alignment);
        }
        if let Some(order) = get_child_element_string(element, "BYTE-ORDER") {
            if let Ok(value) = ByteOrderEnum::try_from(order) {
                definition.set_byte_order(value);
            } else {
                let message = format!("Unsupported BYTE-ORDER <{order}>");
                self.not_implemented(message)?;
            }
        }
        if let Some(native) = get_child_element_string(element, "NATIVE-DECLARATION") {
            definition.set_native_declaration(native);
        }
        Ok(())
    }
```

and the payload-dispatch arm (Task 3's `read_element_payload` match):

```rust
            ElementRef::SwBaseType(id) => {
                let payload = self.read_identifiable_payload(element, document)?;
                let Some(sw_base_type) = document.sw_base_types.get_mut(id) else {
                    return Ok(());
                };
                if let Some(long_name) = payload.long_name {
                    sw_base_type.base_mut().base_mut().base_mut().set_long_name(long_name);
                }
                if let Some(desc) = payload.desc {
                    sw_base_type.base_mut().base_mut().base_mut().set_desc(desc);
                }
                if let Some(introduction) = payload.introduction {
                    sw_base_type.base_mut().base_mut().base_mut().set_introduction(introduction);
                }
                if let Some(admin_data) = payload.admin_data {
                    sw_base_type.base_mut().base_mut().base_mut().set_admin_data(admin_data);
                }
                // py readBaseTypeDirectDefinition(element, data_type.getBaseTypeDefinition())
                let definition_id = sw_base_type.base().get_base_type_definition();
                let definition = match definition_id {
                    Some(definition_id) => definition_id,
                    None => {
                        let definition = BaseTypeDirectDefinition::new();
                        let definition_id = document.base_type_direct_definitions.insert(definition);
                        sw_base_type.base_mut().set_base_type_definition(definition_id);
                        definition_id
                    }
                };
                if let Some(definition) = document.base_type_direct_definitions.get_mut(definition) {
                    self.read_base_type_direct_definition(element, definition)?;
                }
            }
```

(Hop count: `SwBaseType.base = BaseType`, `BaseType.base = ARElement`, ARElement chain → Identifiable getters are 3 hops out — verify against the generated file and adapt. Arena/getter names: `grep -n "base_type_direct_definitions\|pub fn get_sw_base_type" src/m2/autosar_templates/autosar_top_level_structure.rs src/m2/msr/asam_hdo/base_types.rs`.)

- [ ] **Step 3: writer arm**

```rust
    /// py writeSwBaseType.
    fn write_sw_base_type<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: SwBaseTypeId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(sw_base_type) = document.sw_base_types.get(id) else {
            return Ok(());
        };
        let identifiable = sw_base_type.base().base().base(); // → Identifiable
        let mut element = BytesStart::new("SW-BASE-TYPE");
        self.write_identifiable_attributes(&mut element, identifiable);
        writer.write_event(Event::Start(element))?;

        if let Some(short_name) = identifiable.get_short_name() {
            write_text_element(writer, "SHORT-NAME", BytesStart::new("SHORT-NAME"), Some(short_name))?;
        }
        if let Some(category) = identifiable.get_category() {
            write_text_element(writer, "CATEGORY", BytesStart::new("CATEGORY"), Some(category))?;
        }
        // LONG-NAME / DESC / INTRODUCTION / ADMIN-DATA via the Task 3 chain
        self.write_identifiable_payload(writer, identifiable, document)?;

        // py setBaseTypeDirectDefinition
        if let Some(definition) = sw_base_type
            .base()
            .get_base_type_definition()
            .and_then(|id| document.base_type_direct_definitions.get(id))
        {
            write_optional_text_element(writer, "BASE-TYPE-SIZE", definition.get_base_type_size())?;
            write_optional_text_element(writer, "BASE-TYPE-ENCODING", definition.get_base_type_encoding())?;
            write_optional_text_element(writer, "MEM-ALIGNMENT", definition.get_mem_alignment())?;
            if let Some(order) = definition.get_byte_order() {
                write_optional_text_element(writer, "BYTE-ORDER", Some(&order.to_string()))?;
            }
            write_optional_text_element(writer, "NATIVE-DECLARATION", definition.get_native_declaration())?;
        }

        writer.write_event(Event::End(BytesEnd::new("SW-BASE-TYPE")))?;
        Ok(())
    }
```

`write_identifiable_attributes` is the S/T/UUID attr helper extracted from the current `write_ar_package` prologue (same attribute order S, T, UUID); `write_identifiable_payload` is the Task 3 writer-chain entry taking `&Identifiable`-shaped getters — since generated classes embed rather than share, type the parameter as the concrete `&Identifiable`-equivalent accessor set by passing the four `Option<Id>`s explicitly:

```rust
    fn write_identifiable_payload<W: Write>(
        &self,
        writer: &mut Writer<W>,
        long_name: Option<MultilanguageLongNameId>,
        desc: Option<MultiLanguageOverviewParagraphId>,
        introduction: Option<DocumentationBlockId>,
        admin_data: Option<AdminDataId>,
        document: &Document,
    ) -> Result<(), WriteError> {
        if let Some(long_name) = long_name.and_then(|id| document.multilanguage_long_names.get(id)) {
            self.set_multi_long_name(writer, long_name, document)?;
        }
        if let Some(desc) = desc.and_then(|id| document.multi_language_overview_paragraphs.get(id)) {
            self.set_multi_language_overview_paragraph(writer, desc, document)?;
        }
        if let Some(introduction) = introduction.and_then(|id| document.documentation_blocks.get(id)) {
            self.write_documentation_block(writer, introduction, document)?;
        }
        if let Some(admin_data) = admin_data.and_then(|id| document.admin_datas.get(id)) {
            self.write_admin_data(writer, admin_data, document)?;
        }
        Ok(())
    }
```

Add the dispatch arm:

```rust
            ElementRef::SwBaseType(id) => self.write_sw_base_type(writer, id, document),
```

- [ ] **Step 4: unit test — the first payload arm end to end**

Add to `#[cfg(test)] mod tests` in `arxml_parser.rs` (this test needs the Task 3 chain *and* this task's `ElementRef::SwBaseType` arm, which is why it lives here):

```rust
    const IDENTIFIABLE_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00052.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <ELEMENTS>
        <SW-BASE-TYPE>
          <SHORT-NAME>T</SHORT-NAME>
          <LONG-NAME>
            <L-4 L="DE">Ein Typ</L-4>
          </LONG-NAME>
          <DESC>
            <L-2 L="EN" xml:space="preserve">A  type</L-2>
          </DESC>
          <INTRODUCTION>
            <P>
              <L-1 L="EN">Intro text</L-1>
            </P>
            <LIST TYPE="LIST">
              <ITEM>
                <P><L-1>Nested</L-1></P>
              </ITEM>
            </LIST>
          </INTRODUCTION>
        </SW-BASE-TYPE>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    #[test]
    fn identifiable_payload_round_trips_text_nodes() {
        let mut document = Document::new();
        ARXMLParser::new(default_options())
            .load_from_reader(Reader::from_str(IDENTIFIABLE_SAMPLE), &mut document)
            .unwrap();

        let pkg = document.get_ar_package(document.get_ar_packages()[0]).unwrap();
        let ElementRef::SwBaseType(id) = pkg.get_elements()[0] else {
            panic!("expected SwBaseType element");
        };
        let base_type = document.get_sw_base_type(id).unwrap();
        let long_name = document
            .get_multilanguage_long_name(base_type.base().base().base().get_long_name().unwrap())
            .unwrap();
        let l4 = document.get_l_long_name(long_name.get_l4()[0]).unwrap();
        assert_eq!(l4.get_l(), Some("DE"));
        assert_eq!(l4.get_value(), Some("Ein Typ"));

        let desc = document
            .get_multi_language_overview_paragraph(
                base_type.base().base().base().get_desc().unwrap(),
            )
            .unwrap();
        let l2 = document.get_l_overview_paragraph(desc.get_l2()[0]).unwrap();
        assert_eq!(l2.get_value(), Some("A  type"));

        let introduction = document
            .get_documentation_block(
                base_type.base().base().base().get_introduction().unwrap(),
            )
            .unwrap();
        assert_eq!(introduction.get_ps().len(), 1);
        assert_eq!(introduction.get_lists().len(), 1);
    }
```

**Note:** `get_sw_base_type`, `get_multilanguage_long_name` etc. are the Document arena getters — confirm their exact names via `grep -n "pub fn get_sw_base_type\|pub fn get_l_long_name\|pub fn get_l_overview_paragraph\|pub fn get_multi_language_overview_paragraph\|pub fn get_documentation_block\|pub fn get_l_paragraph\|pub fn get_ar_list\|pub fn get_item" src/m2/autosar_templates/autosar_top_level_structure.rs` and adapt. The `base().base().base()` hop count is the SwBaseType→BaseType→ARElement→…→Identifiable chain; count hops from the generated `SwBaseType`. The LIST parse in this sample also pins `ListEnum`/`Item` wiring from Task 3.

- [ ] **Step 5: graduate the fixture**

In `tests/integration/all_fixtures.rs` add `"AUTOSAR_MOD_AISpecification_BaseTypes_Standard.arxml"` to `WARNING_FREE_SOURCES`; in `tests/integration/format_byte_roundtrip.rs` add it to `FORMAT_SOURCES`.

Run: `set -o pipefail; cargo test --test all_fixtures --test format_byte_roundtrip 2>&1 | tail -8`
Expected: PASS. If the byte test fails, use `cargo run --bin arxml-format -- tests/integration/test_files/AUTOSAR_MOD_AISpecification_BaseTypes_Standard.arxml /tmp/out.arxml` and `diff /tmp/out.arxml tests/integration/test_files/AUTOSAR_MOD_AISpecification_BaseTypes_Standard.arxml` — fix the emitter order before proceeding.

- [ ] **Step 6: commit**

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && set -o pipefail; cargo test > /dev/null && echo GREEN
git add src/parser src/writer tests/integration
git commit -m "feat(p2): SwBaseType family; graduate BaseTypes_Standard (P2 Task 4)"
```

---

### Task 5: ReferenceBase (AR-PACKAGE child) — enables the Collection/LifeCycle/Unit fixtures

**Files:**
- Modify: `tools/py2rust/templates.py` (add `push_reference_base` to the pinned `ARPackage`)
- Modify: `src/parser/arxml_parser.rs`, `src/writer/arxml_writer.rs`
- Regenerate: `src/m2/**`

**py methods:** `readReferenceBases` (arxml_parser.py:16054), `writeReferenceBases` (arxml_writer.py:15498). Emission order inside `writeARPackage`: SHORT-LABEL, IS-DEFAULT, IS-GLOBAL, BASE-IS-THIS-PACKAGE, GLOBAL-IN-PACKAGE-REFS, GLOBAL-ELEMENTS, PACKAGE-REF. **Placement: REFERENCE-BASES is written between the Identifiable payload and ELEMENTS** (py `writeARPackage` line 15827: writeIdentifiable → writeReferenceBases → writeARPackageElements → writeARPackages).

- [ ] **Step 1: pinned-template accessor**

In `tools/py2rust/templates.py`, next to `get_reference_bases` (line 607), add:

```rust
    pub fn push_reference_base(&mut self, id: ReferenceBaseId) -> &mut Self {
        self.reference_bases.push(id);
        self
    }
```

Also extend the pinned `Document`-side keep-compares (`assert_structurally_equal` — templates.py line 1012 already compares lengths; verify it compares element-wise and add the per-element comparison if missing, following the `ar_packages` pattern). Regenerate (`python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src`) and run `--check`.

- [ ] **Step 2: reader**

Replace the `REFERENCE-BASES` `not_implemented` block in `read_ar_package` with:

```rust
        // py readReferenceBases
        if let Some(bases_node) = find(element, "REFERENCE-BASES") {
            for base_node in find_all(bases_node, "REFERENCE-BASE") {
                let mut base = ReferenceBase::new();
                self.read_ar_object(base_node, base.base_mut());
                if let Some(label) = get_child_element_string(base_node, "SHORT-LABEL") {
                    base.set_short_label(label);
                }
                if let Some(value) = get_child_element_optional_boolean(base_node, "IS-DEFAULT") {
                    base.set_is_default(value);
                }
                if let Some(value) = get_child_element_optional_boolean(base_node, "IS-GLOBAL") {
                    base.set_is_global(value);
                }
                if let Some(value) = get_child_element_optional_boolean(base_node, "BASE-IS-THIS-PACKAGE") {
                    base.set_base_is_this_package(value);
                }
                for r#ref in get_child_element_ref_type_list(base_node, "GLOBAL-IN-PACKAGE-REFS/GLOBAL-IN-PACKAGE-REF") {
                    base.push_global_in_package_ref(r#ref);
                }
                for global in find_all(base_node, "GLOBAL-ELEMENTS/GLOBAL-ELEMENT") {
                    if let Some(text) = &global.text {
                        base.push_global_elements(text.as_str());
                    }
                }
                if let Some(r#ref) = get_child_element_optional_ref_type(base_node, "PACKAGE-REF") {
                    base.set_package_ref(r#ref);
                }
                let base_id = document.reference_bases.insert(base);
                if let Some(package) = document.ar_packages.get_mut(id) {
                    package.push_reference_base(base_id);
                }
            }
        }
```

(py's `GLOBAL-ELEMENT` text becomes a `ReferrableSubtypesEnum` literal; the model keeps `Vec<String>` per Task 0.)

- [ ] **Step 3: writer**

In `write_ar_package`, insert between the ADMIN-DATA emission and the ELEMENTS block:

```rust
        // py writeReferenceBases — REFERENCE-BASES comes before ELEMENTS.
        let reference_bases = package.get_reference_bases();
        if !reference_bases.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("REFERENCE-BASES")))?;
            for base_id in reference_bases {
                if let Some(base) = document.reference_bases.get(*base_id) {
                    writer.write_event(Event::Start(BytesStart::new("REFERENCE-BASE")))?;
                    write_optional_text_element(writer, "SHORT-LABEL", base.get_short_label())?;
                    write_optional_boolean_element(writer, "IS-DEFAULT", base.get_is_default())?;
                    write_optional_boolean_element(writer, "IS-GLOBAL", base.get_is_global())?;
                    write_optional_boolean_element(writer, "BASE-IS-THIS-PACKAGE", base.get_base_is_this_package())?;
                    write_ref_type_list(writer, "GLOBAL-IN-PACKAGE-REFS", "GLOBAL-IN-PACKAGE-REF", base.get_global_in_package_refs())?;
                    let global_elements = base.get_global_elements();
                    if !global_elements.is_empty() {
                        writer.write_event(Event::Start(BytesStart::new("GLOBAL-ELEMENTS")))?;
                        for element_text in global_elements {
                            write_text_element(writer, "GLOBAL-ELEMENT", BytesStart::new("GLOBAL-ELEMENT"), Some(element_text))?;
                        }
                        writer.write_event(Event::End(BytesEnd::new("GLOBAL-ELEMENTS")))?;
                    }
                    write_optional_ref_type(writer, "PACKAGE-REF", base.get_package_ref())?;
                    writer.write_event(Event::End(BytesEnd::new("REFERENCE-BASE")))?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("REFERENCE-BASES")))?;
        }
```

- [ ] **Step 4: unit test + commit (no graduation — Collection fixtures need Task 6 too)**

Add a parser unit test (same style as Task 3's) asserting a `REFERENCE-BASES` block parses into `short_label`/`is_global`/`package_ref` values, then:

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && set -o pipefail; cargo test > /dev/null && echo GREEN
git add tools/py2rust src/m2 src/parser src/writer tests
git commit -m "feat(p2): ReferenceBase read+write (P2 Task 5)"
```

---

### Task 6: Collection → graduates the five `Collection_*` blueprints

**Files:**
- Modify: `src/parser/arxml_parser.rs`, `src/writer/arxml_writer.rs`
- Modify: `tests/integration/all_fixtures.rs`, `tests/integration/format_byte_roundtrip.rs`

**py methods:** `readCollection` (arxml_parser.py:16185), `writeCollection`/`writeCollectionElementRefs`/`writeCollectionSourceElementRefs` (arxml_writer.py:15683/15711/15718). `AUTO_COLLECT_XML_MAP = {refAll: REF-ALL, refNone: REF-NONE, refNonStandard: REF-NON-STANDARD}` (arxml_parser.py:1316). `COLLECTED-INSTANCE-IREFS`/`SOURCE-INSTANCE-IREFS` never occur in the pinned fixtures and need `AnyInstanceRef` plumbing — both sides no-op when absent (py reads/writes only when present); tracked on the port checklist for a later batch.

- [ ] **Step 1: reader arm** (in `read_element_payload`)

```rust
            ElementRef::Collection(id) => {
                let payload = self.read_identifiable_payload(element, document)?;
                // py readCollection — AUTO-COLLECT maps XML token → enum
                // literal; unknown tokens are a warning, matching py.
                let auto_collect = match find(element, "AUTO-COLLECT") {
                    Some(node) => match node.text.as_deref().map(str::trim) {
                        Some("REF-ALL") => Some(AutoCollectEnum::RefAll),
                        Some("REF-NONE") => Some(AutoCollectEnum::RefNone),
                        Some("REF-NON-STANDARD") => Some(AutoCollectEnum::RefNonStandard),
                        other => {
                            let message =
                                format!("Unsupported AUTO-COLLECT <{}>", other.unwrap_or(""));
                            self.not_implemented(message)?;
                            None
                        }
                    },
                    None => None,
                };
                let collection_semantics =
                    get_child_element_string(element, "COLLECTION-SEMANTICS").map(str::to_string);
                let element_role =
                    get_child_element_string(element, "ELEMENT-ROLE").map(str::to_string);
                let element_refs = get_child_element_ref_type_list(element, "ELEMENT-REFS/ELEMENT-REF");
                let source_element_refs =
                    get_child_element_ref_type_list(element, "SOURCE-ELEMENT-REFS/SOURCE-ELEMENT-REF");

                if let Some(collection) = document.collections.get_mut(id) {
                    if let Some(long_name) = payload.long_name {
                        collection.base_mut().set_long_name(long_name);
                    }
                    if let Some(desc) = payload.desc {
                        collection.base_mut().set_desc(desc);
                    }
                    if let Some(introduction) = payload.introduction {
                        collection.base_mut().set_introduction(introduction);
                    }
                    if let Some(admin_data) = payload.admin_data {
                        collection.base_mut().set_admin_data(admin_data);
                    }
                    if let Some(auto_collect) = auto_collect {
                        collection.set_auto_collect(auto_collect);
                    }
                    if let Some(semantics) = collection_semantics {
                        collection.set_collection_semantics(semantics);
                    }
                    if let Some(role) = element_role {
                        collection.set_element_role(role);
                    }
                    for r#ref in element_refs {
                        collection.push_element_refs(r#ref);
                    }
                    for r#ref in source_element_refs {
                        collection.push_source_element_refs(r#ref);
                    }
                }
            }
```

(`AutoCollectEnum` variant spelling: confirm with `grep -n "enum AutoCollectEnum" -A 8 src/m2/autosar_templates/generic_structure/general_template_classes/element_collection.rs`; `Collection.base = Identifiable`, so the payload setters are one hop in. `push_element_refs`: vec-of-`RefType` after Task 0 — confirm the generated push name.)

- [ ] **Step 2: writer arm**

```rust
    /// py writeCollection.
    fn write_collection<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: CollectionId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(collection) = document.collections.get(id) else {
            return Ok(());
        };
        let identifiable = collection.base();
        let mut element = BytesStart::new("COLLECTION");
        self.write_identifiable_attributes(&mut element, identifiable);
        writer.write_event(Event::Start(element))?;

        if let Some(short_name) = identifiable.get_short_name() {
            write_text_element(writer, "SHORT-NAME", BytesStart::new("SHORT-NAME"), Some(short_name))?;
        }
        if let Some(category) = identifiable.get_category() {
            write_text_element(writer, "CATEGORY", BytesStart::new("CATEGORY"), Some(category))?;
        }
        self.write_identifiable_payload(
            writer,
            identifiable.get_long_name(),
            identifiable.get_desc(),
            identifiable.get_introduction(),
            identifiable.get_admin_data(),
            document,
        )?;

        // py writeCollection: AUTO-COLLECT, COLLECTION-SEMANTICS,
        // ELEMENT-ROLE, ELEMENT-REFS, SOURCE-ELEMENT-REFS.
        if let Some(auto_collect) = collection.get_auto_collect() {
            let token = match auto_collect {
                AutoCollectEnum::RefAll => "REF-ALL",
                AutoCollectEnum::RefNone => "REF-NONE",
                AutoCollectEnum::RefNonStandard => "REF-NON-STANDARD",
            };
            write_text_element(writer, "AUTO-COLLECT", BytesStart::new("AUTO-COLLECT"), Some(token))?;
        }
        write_optional_text_element(writer, "COLLECTION-SEMANTICS", collection.get_collection_semantics())?;
        write_optional_text_element(writer, "ELEMENT-ROLE", collection.get_element_role())?;
        write_ref_type_list(writer, "ELEMENT-REFS", "ELEMENT-REF", collection.get_element_refs())?;
        write_ref_type_list(writer, "SOURCE-ELEMENT-REFS", "SOURCE-ELEMENT-REF", collection.get_source_element_refs())?;

        writer.write_event(Event::End(BytesEnd::new("COLLECTION")))?;
        Ok(())
    }
```

and `ElementRef::Collection(id) => self.write_collection(writer, id, document),`.

- [ ] **Step 3: graduate the five fixtures**

Add to both `WARNING_FREE_SOURCES` and `FORMAT_SOURCES`:

```rust
    "AUTOSAR_MOD_AISpecification_Collection_Body_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_Chassis_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_MmedTelmHmi_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_OccptPedSfty_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_Pt_Blueprint.arxml",
```

Run: `set -o pipefail; cargo test --test all_fixtures --test format_byte_roundtrip 2>&1 | tail -8`
Expected: PASS (debug any failure with `arxml-format` + `diff` as in Task 4).

- [ ] **Step 4: commit**

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && set -o pipefail; cargo test > /dev/null && echo GREEN
git add src/parser src/writer tests/integration
git commit -m "feat(p2): Collection family; graduate 5 Collection blueprints (P2 Task 6)"
```

---

### Task 7: LifeCycleInfoSet family → graduates the two `*_LifeCycle_Standard` fixtures

**Files:**
- Modify: `src/parser/arxml_parser.rs`, `src/writer/arxml_writer.rs`
- Modify: `tests/integration/all_fixtures.rs`, `tests/integration/format_byte_roundtrip.rs`

**py methods:** `getLifeCyclePeriod` 14965, `readLifeCycleInfoUseInsteadRefs` 14975, `readLifeCycleInfo` 14979, `readLifeCycleInfoSetLifeCycleInfos` 14988, `readLifeCycleInfoSet` 14998; `writeLifeCycleInfoUseInsteadRefs` 13671, `writeLifeCycleInfo` 13678, `writeLifeCycleInfoSetLifeCycleInfos` 13689, `writeLifeCycleInfoSet` 13699, plus `setLifeCyclePeriod` (mirror of the reader — verify: `grep -n "def setLifeCyclePeriod" -A 8 target/py-armodel/src/armodel/writer/arxml_writer.py`; it writes DATE, AR-RELEASE-VERSION, PRODUCT-RELEASE as text children in that order).

**Model:** `LifeCyclePeriod { date, ar_release_version, product_release }`, `LifeCycleInfo { lc_object_ref: Option<RefType>, lc_state_ref: Option<RefType>, period_begin/end: Option<LifeCyclePeriodId>, remark: Option<DocumentationBlockId>, use_instead_refs: Vec<RefType> }` (refs become `RefType` in Task 0), `LifeCycleInfoSet { default_lc_state_ref, default_period_begin/end, life_cycle_infos, used_life_cycle_state_definition_group_ref }`. `LifeCycleInfo` is **not** an AR-PACKAGE element — it lives inside `LIFE-CYCLE-INFOS`; no dispatch arm, only arena inserts.

- [ ] **Step 1: reader methods** (in `arxml_parser.rs`)

```rust
    /// py getLifeCyclePeriod — `key` is the wrapper element ("PERIOD-BEGIN",
    /// "PERIOD-END", "DEFAULT-PERIOD-BEGIN", …).
    fn get_life_cycle_period(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<LifeCyclePeriodId>, ParseError> {
        let Some(period_node) = find(element, key) else {
            return Ok(None);
        };
        let mut period = LifeCyclePeriod::new();
        if let Some(date) = get_child_element_string(period_node, "DATE") {
            period.set_date(date);
        }
        if let Some(version) = get_child_element_string(period_node, "AR-RELEASE-VERSION") {
            period.set_ar_release_version(version);
        }
        if let Some(release) = get_child_element_string(period_node, "PRODUCT-RELEASE") {
            period.set_product_release(release);
        }
        Ok(Some(document.life_cycle_periods.insert(period)))
    }

    /// py readLifeCycleInfo — called for each LIFE-CYCLE-INFOS/LIFE-CYCLE-INFO.
    fn read_life_cycle_info(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<LifeCycleInfoId, ParseError> {
        let mut info = LifeCycleInfo::new();
        self.read_ar_object(element, info.base_mut());
        if let Some(r#ref) = get_child_element_optional_ref_type(element, "LC-OBJECT-REF") {
            info.set_lc_object_ref(r#ref);
        }
        if let Some(r#ref) = get_child_element_optional_ref_type(element, "LC-STATE-REF") {
            info.set_lc_state_ref(r#ref);
        }
        if let Some(period) = self.get_life_cycle_period(element, "PERIOD-BEGIN", document)? {
            info.set_period_begin(period);
        }
        if let Some(period) = self.get_life_cycle_period(element, "PERIOD-END", document)? {
            info.set_period_end(period);
        }
        if let Some(remark_node) = find(element, "REMARK") {
            let remark = self.read_documentation_block(remark_node, document)?;
            info.set_remark(remark);
        }
        for r#ref in get_child_element_ref_type_list(element, "USE-INSTEAD-REFS/USE-INSTEAD-REF") {
            info.push_use_instead_refs(r#ref);
        }
        Ok(document.life_cycle_infos.insert(info))
    }
```

(Arena names to confirm: `grep -n "SlotMap<LifeCyclePeriodId\|SlotMap<LifeCycleInfoId\|SlotMap<LifeCycleInfoSetId" src/m2/autosar_templates/autosar_top_level_structure.rs`.)

```rust
    /// py readLifeCycleInfoSet (payload part; SHORT-NAME/UUID/CATEGORY are the
    /// caller's). Returns the pieces to copy into the arena entry.
    fn read_life_cycle_info_set_payload(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<LifeCycleInfoSetPayload, ParseError> {
        let default_lc_state_ref = get_child_element_optional_ref_type(element, "DEFAULT-LC-STATE-REF");
        let default_period_begin = self.get_life_cycle_period(element, "DEFAULT-PERIOD-BEGIN", document)?;
        let default_period_end = self.get_life_cycle_period(element, "DEFAULT-PERIOD-END", document)?;
        let mut life_cycle_infos = Vec::new();
        if let Some(infos_node) = find(element, "LIFE-CYCLE-INFOS") {
            for child in find_all(infos_node, "*") {
                if child.name == "LIFE-CYCLE-INFO" {
                    life_cycle_infos.push(self.read_life_cycle_info(child, document)?);
                } else {
                    let message = format!("Unsupported Life Cycle Info <{}>", child.name);
                    self.not_implemented(message)?;
                }
            }
        }
        let used_ref = get_child_element_optional_ref_type(
            element,
            "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF",
        );
        Ok(LifeCycleInfoSetPayload {
            default_lc_state_ref,
            default_period_begin,
            default_period_end,
            life_cycle_infos,
            used_life_cycle_state_definition_group_ref: used_ref,
        })
    }
```

(`LifeCycleInfoSetPayload` is a plain struct of those five pieces, declared next to `IdentifiablePayload`.)

- [ ] **Step 2: dispatch arm + writer**

Reader arm:

```rust
            ElementRef::LifeCycleInfoSet(id) => {
                let payload = self.read_identifiable_payload(element, document)?;
                let set_payload = self.read_life_cycle_info_set_payload(element, document)?;
                if let Some(info_set) = document.life_cycle_info_sets.get_mut(id) {
                    /* copy Identifiable payload (one base() hop: base: ARElement) */
                    if let Some(r#ref) = set_payload.default_lc_state_ref {
                        info_set.set_default_lc_state_ref(r#ref);
                    }
                    if let Some(period) = set_payload.default_period_begin {
                        info_set.set_default_period_begin(period);
                    }
                    if let Some(period) = set_payload.default_period_end {
                        info_set.set_default_period_end(period);
                    }
                    for info in set_payload.life_cycle_infos {
                        info_set.push_life_cycle_infos(info);
                    }
                    if let Some(r#ref) = set_payload.used_life_cycle_state_definition_group_ref {
                        info_set.set_used_life_cycle_state_definition_group_ref(r#ref);
                    }
                }
            }
```

Writer (order per py `writeLifeCycleInfoSet`: DEFAULT-LC-STATE-REF, DEFAULT-PERIOD-BEGIN, DEFAULT-PERIOD-END, LIFE-CYCLE-INFOS, USED-…-REF; each LIFE-CYCLE-INFO: S/T attrs, LC-OBJECT-REF, LC-STATE-REF, PERIOD-BEGIN, PERIOD-END, REMARK, USE-INSTEAD-REFS):

```rust
    fn write_life_cycle_period<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        period: &LifeCyclePeriod,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new(key);
        self.write_ar_object_attributes(&mut element, period.base());
        writer.write_event(Event::Start(element))?;
        write_optional_text_element(writer, "DATE", period.get_date())?;
        write_optional_text_element(writer, "AR-RELEASE-VERSION", period.get_ar_release_version())?;
        write_optional_text_element(writer, "PRODUCT-RELEASE", period.get_product_release())?;
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }
```

plus `write_life_cycle_info`, `write_life_cycle_info_set` and the `ElementRef::LifeCycleInfoSet` arm — each a direct mirror of the py writer bodies cited above (LIFE-CYCLE-INFOS wrapper only when non-empty; REMARK via `write_documentation_block`; USE-INSTEAD-REFS via `write_ref_type_list`).

- [ ] **Step 3: graduate**

Add `"AUTOSAR_MOD_AISpecification_ApplicationDataType_LifeCycle_Standard.arxml"` and `"AUTOSAR_MOD_AISpecification_Keyword_LifeCycle_Standard.arxml"` to both gate lists. Run and verify as Tasks 4/6.

- [ ] **Step 4: commit**

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && set -o pipefail; cargo test > /dev/null && echo GREEN
git add src/parser src/writer tests/integration
git commit -m "feat(p2): LifeCycleInfoSet family; graduate 2 LifeCycle fixtures (P2 Task 7)"
```

---

### Task 8: PhysicalDimension → graduates `PhysicalDimension_Standard`

**Files:** parser/writer dispatch arms + both gate lists.

**py methods:** `readPhysicalDimension` (arxml_parser.py:13986), `writePhysicalDimension` (arxml_writer.py:12570). Seven numerical children, fixed order: LENGTH-EXP, LUMINOUS-INTENSITY-EXP, MASS-EXP, MOLAR-AMOUNT-EXP, TEMPERATURE-EXP, TIME-EXP, CURRENT-EXP. Numericals are text (`Option<String>`); SHORT-LABEL attr never occurs in the batch (fact 6).

- [ ] **Step 1: both arms**

Reader arm in `read_element_payload` (`PhysicalDimension.base = ARElement`; the payload setters are the same hop count as `SwBaseType` — verify once against the generated struct):

```rust
            ElementRef::PhysicalDimension(id) => {
                let payload = self.read_identifiable_payload(element, document)?;
                // py readPhysicalDimension — fixed child order.
                let length_exp = get_child_element_string(element, "LENGTH-EXP").map(str::to_string);
                let luminous = get_child_element_string(element, "LUMINOUS-INTENSITY-EXP").map(str::to_string);
                let mass_exp = get_child_element_string(element, "MASS-EXP").map(str::to_string);
                let molar = get_child_element_string(element, "MOLAR-AMOUNT-EXP").map(str::to_string);
                let temperature = get_child_element_string(element, "TEMPERATURE-EXP").map(str::to_string);
                let time_exp = get_child_element_string(element, "TIME-EXP").map(str::to_string);
                let current = get_child_element_string(element, "CURRENT-EXP").map(str::to_string);

                if let Some(dimension) = document.physical_dimensions.get_mut(id) {
                    if let Some(long_name) = payload.long_name {
                        dimension.base_mut().base_mut().base_mut().set_long_name(long_name);
                    }
                    if let Some(desc) = payload.desc {
                        dimension.base_mut().base_mut().base_mut().set_desc(desc);
                    }
                    if let Some(introduction) = payload.introduction {
                        dimension.base_mut().base_mut().base_mut().set_introduction(introduction);
                    }
                    if let Some(admin_data) = payload.admin_data {
                        dimension.base_mut().base_mut().base_mut().set_admin_data(admin_data);
                    }
                    if let Some(value) = length_exp {
                        dimension.set_length_exp(value);
                    }
                    if let Some(value) = luminous {
                        dimension.set_luminous_intensity_exp(value);
                    }
                    if let Some(value) = mass_exp {
                        dimension.set_mass_exp(value);
                    }
                    if let Some(value) = molar {
                        dimension.set_molar_amount_exp(value);
                    }
                    if let Some(value) = temperature {
                        dimension.set_temperature_exp(value);
                    }
                    if let Some(value) = time_exp {
                        dimension.set_time_exp(value);
                    }
                    if let Some(value) = current {
                        dimension.set_current_exp(value);
                    }
                }
            }
```

Writer arm (`PhysicalDimension` emitter, mirror of py 12570):

```rust
    fn write_physical_dimension<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: PhysicalDimensionId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(dimension) = document.physical_dimensions.get(id) else {
            return Ok(());
        };
        let identifiable = dimension.base().base().base(); // → Identifiable
        let mut element = BytesStart::new("PHYSICAL-DIMENSION");
        self.write_identifiable_attributes(&mut element, identifiable);
        writer.write_event(Event::Start(element))?;

        if let Some(short_name) = identifiable.get_short_name() {
            write_text_element(writer, "SHORT-NAME", BytesStart::new("SHORT-NAME"), Some(short_name))?;
        }
        if let Some(category) = identifiable.get_category() {
            write_text_element(writer, "CATEGORY", BytesStart::new("CATEGORY"), Some(category))?;
        }
        self.write_identifiable_payload(
            writer,
            identifiable.get_long_name(),
            identifiable.get_desc(),
            identifiable.get_introduction(),
            identifiable.get_admin_data(),
            document,
        )?;

        write_optional_text_element(writer, "LENGTH-EXP", dimension.get_length_exp())?;
        write_optional_text_element(writer, "LUMINOUS-INTENSITY-EXP", dimension.get_luminous_intensity_exp())?;
        write_optional_text_element(writer, "MASS-EXP", dimension.get_mass_exp())?;
        write_optional_text_element(writer, "MOLAR-AMOUNT-EXP", dimension.get_molar_amount_exp())?;
        write_optional_text_element(writer, "TEMPERATURE-EXP", dimension.get_temperature_exp())?;
        write_optional_text_element(writer, "TIME-EXP", dimension.get_time_exp())?;
        write_optional_text_element(writer, "CURRENT-EXP", dimension.get_current_exp())?;

        writer.write_event(Event::End(BytesEnd::new("PHYSICAL-DIMENSION")))?;
        Ok(())
    }
```

and the dispatch arms `ElementRef::PhysicalDimension(id) => self.write_physical_dimension(writer, id, document),` / the reader arm above.

- [ ] **Step 2: graduate** `"AUTOSAR_MOD_AISpecification_PhysicalDimension_Standard.arxml"` in both lists; verify.

- [ ] **Step 3: commit**

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && set -o pipefail; cargo test > /dev/null && echo GREEN
git add src/parser src/writer tests/integration
git commit -m "feat(p2): PhysicalDimension family; graduate PhysicalDimension_Standard (P2 Task 8)"
```

---

### Task 9: Unit → graduates `Unit_Standard`

**Files:** parser/writer dispatch arms + both gate lists.

**py methods:** `readUnit` (arxml_parser.py:8796), `writeUnit` (arxml_writer.py:3895); helpers `getSingleLanguageUnitNames` 2022, `readSingleLanguageUnitNames` 2010 (`readMixedStringText` 13743 + `readMixedContentForUnitNames`), `setSingleLanguageUnitNames` 1734 (`writeMixedStringText` 12949 sets element text, then `writeMixedContentForUnitNames` 1690 adds SUP/SUB attrs — neither attr occurs in the batch).

Emission order: SHORT-NAME, CATEGORY, payload (LONG-NAME/DESC/INTRODUCTION/ADMIN-DATA), DISPLAY-NAME, FACTOR-SI-TO-UNIT, OFFSET-SI-TO-UNIT, PHYSICAL-DIMENSION-REF.

- [ ] **Step 1: reader arm** — in `read_element_payload` (`Unit.base = ARElement`; same hop count as `PhysicalDimension`):

```rust
            ElementRef::Unit(id) => {
                let payload = self.read_identifiable_payload(element, document)?;
                // py getSingleLanguageUnitNames — DISPLAY-NAME wrapper whose
                // text is the mixed string (SUP/SUB attrs never occur here).
                let display_name = match find(element, "DISPLAY-NAME") {
                    Some(node) => {
                        let mut names = SingleLanguageUnitNames::new();
                        if let Some(text) = &node.text {
                            names.set_mixed_string(text.as_str());
                        }
                        let names_id = document.single_language_unit_names.insert(names);
                        Some(names_id)
                    }
                    None => None,
                };
                let factor = get_child_element_string(element, "FACTOR-SI-TO-UNIT").map(str::to_string);
                let offset = get_child_element_string(element, "OFFSET-SI-TO-UNIT").map(str::to_string);
                let dimension_ref = get_child_element_optional_ref_type(element, "PHYSICAL-DIMENSION-REF");

                if let Some(unit) = document.units.get_mut(id) {
                    if let Some(long_name) = payload.long_name {
                        unit.base_mut().base_mut().base_mut().set_long_name(long_name);
                    }
                    if let Some(desc) = payload.desc {
                        unit.base_mut().base_mut().base_mut().set_desc(desc);
                    }
                    if let Some(introduction) = payload.introduction {
                        unit.base_mut().base_mut().base_mut().set_introduction(introduction);
                    }
                    if let Some(admin_data) = payload.admin_data {
                        unit.base_mut().base_mut().base_mut().set_admin_data(admin_data);
                    }
                    if let Some(names_id) = display_name {
                        unit.set_display_name(names_id);
                    }
                    if let Some(factor) = factor {
                        unit.set_factor_si_to_unit(factor);
                    }
                    if let Some(offset) = offset {
                        unit.set_offset_si_to_unit(offset);
                    }
                    if let Some(r#ref) = dimension_ref {
                        unit.set_physical_dimension_ref(r#ref);
                    }
                }
            }
```

Confirm two names before compiling: the text accessor (`set_mixed_string` vs `set_value` — py stores it via `getMixedString()`) with
`grep -n "pub fn set_mixed_string\|pub fn set_value" src/m2/msr/asam_hdo/units.rs src/m2/autosar_templates/generic_structure/general_template_classes/stereotype_mixins.rs`
and the Document arena name (`units` vs `unit_list`) with `grep -n "SlotMap<UnitId" src/m2/autosar_templates/autosar_top_level_structure.rs`.

- [ ] **Step 2: writer arm**

```rust
    fn write_unit<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: UnitId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(unit) = document.units.get(id) else {
            return Ok(());
        };
        let identifiable = unit.base().base().base(); // → Identifiable
        let mut element = BytesStart::new("UNIT");
        self.write_identifiable_attributes(&mut element, identifiable);
        writer.write_event(Event::Start(element))?;

        if let Some(short_name) = identifiable.get_short_name() {
            write_text_element(writer, "SHORT-NAME", BytesStart::new("SHORT-NAME"), Some(short_name))?;
        }
        if let Some(category) = identifiable.get_category() {
            write_text_element(writer, "CATEGORY", BytesStart::new("CATEGORY"), Some(category))?;
        }
        self.write_identifiable_payload(
            writer,
            identifiable.get_long_name(),
            identifiable.get_desc(),
            identifiable.get_introduction(),
            identifiable.get_admin_data(),
            document,
        )?;

        // py setSingleLanguageUnitNames — wrapper element with the mixed
        // string as text; SUP/SUB attrs only when set (never in this batch).
        if let Some(names) = unit
            .get_display_name()
            .and_then(|names_id| document.single_language_unit_names.get(names_id))
        {
            let display_element = BytesStart::new("DISPLAY-NAME");
            write_text_element(writer, "DISPLAY-NAME", display_element, names.get_mixed_string())?;
        }
        write_optional_text_element(writer, "FACTOR-SI-TO-UNIT", unit.get_factor_si_to_unit())?;
        write_optional_text_element(writer, "OFFSET-SI-TO-UNIT", unit.get_offset_si_to_unit())?;
        write_optional_ref_type(writer, "PHYSICAL-DIMENSION-REF", unit.get_physical_dimension_ref())?;

        writer.write_event(Event::End(BytesEnd::new("UNIT")))?;
        Ok(())
    }
```

Dispatch arm: `ElementRef::Unit(id) => self.write_unit(writer, id, document),`.

- [ ] **Step 3: graduate** `"AUTOSAR_MOD_AISpecification_Unit_Standard.arxml"` in both lists; verify.

- [ ] **Step 4: commit**

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && set -o pipefail; cargo test > /dev/null && echo GREEN
git add src/parser src/writer tests/integration
git commit -m "feat(p2): Unit family; graduate Unit_Standard (P2 Task 9)"
```

---

### Task 10: batch close-out

- [ ] **Step 1: regenerate the port checklist**

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --emit-port-checklist
git diff --stat docs/port_checklist.md
```

Expected: the read/write methods ported by Tasks 3–9 flip to `[x]` (readSwBaseType, writeSwBaseType, readCollection, writeCollection, readUnit, writeUnit, readPhysicalDimension, writePhysicalDimension, readLifeCycleInfoSet, writeLifeCycleInfoSet, readLifeCycleInfo, writeLifeCycleInfo, readReferenceBases…, plus whatever the Identifiable chain covered).

- [ ] **Step 2: full verification**

```bash
set -o pipefail; cargo build && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test 2>&1 | tail -12
```

Expected: clean. `format_byte_roundtrip` now runs `arxml-format` over 10 sources; `all_fixtures` keeps all 32 at model equality with 10 warning-free.

- [ ] **Step 3: checklist note for deferred items**

Append to the PR description (not a code file): deferred items discovered but not ported — annotations (`getAnnotations`), SHORT-NAME-FRAGMENTS, VARIATION-POINT, E/IE/TT inline content, DEF-LIST/FORMULA/FIGURE documentation kinds, `AnyInstanceRef` instance-refs, numerical `SHORT-LABEL`/primitive `T` attributes. None occur in the 32 pinned fixtures; all are py no-ops when absent. Each gets its checklist line when a fixture needs it.

- [ ] **Step 4: commit + PR**

```bash
git add docs/port_checklist.md
git commit -m "chore(p2): regenerate port checklist after infra+payload batch (P2 Task 10)"
git push -u origin p2-infra-and-common-payload
gh pr create --title "P2: port infrastructure + Identifiable payload, graduate 10 fixtures" --body-file <(printf '%s\n' \
  "Closes #<tracking-issue>" \
  "" \
  "First P2 batch per the roadmap spec (§5): py2rust model-gap fixes (RefType value fields, default-constructed fields, full ReferenceBase), --emit-port-checklist, WARNING_FREE_SOURCES gate, the Identifiable payload chain (LONG-NAME/DESC/INTRODUCTION), and the SwBaseType/ReferenceBase/Collection/LifeCycleInfoSet/PhysicalDimension/Unit families." \
  "" \
  "Gates: WARNING_FREE_SOURCES = 10 fixtures, FORMAT_SOURCES = 10 fixtures, all 32 model-equality, checklist regenerated.")
```

(Create the tracking issue first per repo convention and substitute its number.)
