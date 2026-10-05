# Parser/Writer Domain-Split Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename the reader layer to `reader` (matching the port vocabulary) and split the hand-written `arxml_reader.rs` (2,318 lines) and `arxml_writer.rs` (2,240 lines) into per-domain modules so the P2–P4 mass port (~2,400 more methods) lands in small, reviewable files instead of two multi-thousand-method impl blocks.

**Architecture:** Task 0 is a mechanical rename (`src/parser/` → `src/reader/`, `ARXMLParser` → `ARXMLReader`, `ParserOptions` → `ReaderOptions`); everything after is a pure-move refactor per the approved spec (`docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md`): `arxml_reader.rs` becomes `arxml_reader/mod.rs` + 12 domain files (same for the writer). Every method stays on the same struct — `impl ARXMLReader { … }` blocks simply live in different files. No structural API, call site, harness, or fixture changes beyond Task 0's rename. Dispatch arms stay in `mod.rs` and only ever call `pub(super)` methods.

**Tech Stack:** Rust stable / edition 2021; no new dependencies. Gate after every task: `cargo build && cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test` — all must be green before the task's commit.

**Branch:** `feature/parser-writer-domain-split` (already checked out; spec committed here). Open a tracking issue before Task 1 and reference it in every commit (`refactor: … (Closes #N)` on the final commit).

---

## Ground truth (verified at commit `01331e1`, the parent of this branch)

Line ranges below refer to `src/reader/arxml_reader.rs` / `src/writer/arxml_writer.rs` at that commit. After earlier tasks move code, **locate each function with `grep -n "fn <name>" <file>`** — the ranges are a reference map, not stable coordinates. Each moved function's body is copied **verbatim**: same signature, same body, same doc comments.

### Parser inventory → domain

| Domain file | Functions (range @01331e1) | `pub(super)` |
|---|---|---|
| `common.rs` | `read_ar_object` (211), `read_xml_space` (221), `read_identifiable_payload` (740), `struct IdentifiablePayload` (85) | all three fns |
| `documentation.rs` | `get_multilanguage_long_name` (537), `get_multi_language_overview_paragraph` (584), `get_documentation_block` (622), `read_documentation_block` (638), `read_paginateable` (706) | the three `get_*` |
| `admin_data.rs` | `read_admin_data` (239), `read_sdg` (301) | `read_admin_data` |
| `compu_method.rs` | `get_compu_const_content` (797), `get_compu_const` (839), `read_compu_scale` (857), `get_compu_scales` (940), `get_compu` (964), `read_compu_method` (985) | `read_compu_method` |
| `data_constr.rs` | `read_scale_constr` (1028), `read_data_constr` (1063), `get_child_limit_element` (1884) | `read_data_constr` |
| `keyword.rs` | `read_keyword` (1185), `read_keyword_set` (1236) | `read_keyword_set` |
| `datatypes.rs` | `get_sw_data_def_props` (1271), `read_application_primitive_data_type` (1362), `read_application_array_data_type` (1391), `read_application_record_data_type` (1487), `read_implementation_data_type` (1561) | the four `read_application*/read_implementation*` |
| `base_types.rs` | `read_sw_base_type` (1604), `read_base_type_direct_definition` (1912) | `read_sw_base_type` |
| `collection.rs` | `read_collection` (1651) | `read_collection` |
| `life_cycle.rs` | `read_life_cycle_info_set` (1722), `get_life_cycle_period` (1946), `read_life_cycle_info` (1969), `read_life_cycle_info_set_payload` (2001) | `read_life_cycle_info_set` |
| `physical_dimension.rs` | `read_physical_dimension` (1763) | `read_physical_dimension` |
| `unit.rs` | `read_unit` (1820) | `read_unit` |
| `mod.rs` keeps | `ReaderOptions`, `default_options`, struct `ARXMLReader`, `load`, `load_from_reader`, `raise_error`, `not_implemented`, `read_ar_packages` (375), `read_ar_package` (396), `read_element_payload` (759), free fns (`xsd_to_version` 2043, `element_factory_for_tag`, `element_set_*`), tests: `parse_sample`, `load_sets_schema_location_and_release`, `load_rejects_wrong_root`, `reference_bases_parse_into_the_arena`, `packages_nest_through_the_document_factory` | — |

Why `pub(super)`: a child module's private items are invisible to the parent — `read_element_payload` (in `mod.rs`) calls each domain's dispatch-target reader, so those must be `pub(super)` (visible throughout the `arxml_reader` module tree). Functions called only within their own file stay private.

### Writer inventory → domain

| Domain file | Functions (range @01331e1) | `pub(super)` |
|---|---|---|
| `common.rs` | `write_ar_object_attributes` (160), `write_identifiable_attributes` (445), `write_identifiable_parts` (465) | all three |
| `documentation.rs` | `set_multi_long_name` (505), `set_multi_language_overview_paragraph` (540), `write_documentation_block` (576), `write_documentation_block_content` (589), `write_paginateable_attrs` (651) | `set_multi_long_name`, `set_multi_language_overview_paragraph`, `write_documentation_block` |
| `admin_data.rs` | `write_admin_data` (170), `write_sdg` (239), `write_sd` (299) | `write_admin_data`, `write_sdg` |
| `compu_method.rs` | `write_compu_method` (665), `set_compu` (727), `set_compu_scales` (756), `write_compu_scale` (773), `write_nominator_denominator` (860), `set_compu_const` (875) | `write_compu_method` |
| `data_constr.rs` | `write_data_constr` (908), `write_data_constr_rule` (963), `write_internal_constrs` (990), `write_phys_constrs` (1032), `write_scale_constrs` (1081) | `write_data_constr` |
| `keyword.rs` | `write_keyword` (1127), `write_keyword_set` (1180) | `write_keyword_set` |
| `datatypes.rs` | `set_sw_data_def_props` (1233), `write_autosar_data_type_parts` (1315), `write_application_primitive_data_type` (1330), `write_composite_element_prototype_body` (1373), `write_application_array_data_type` (1399), `write_application_record_data_type` (1497), `write_implementation_data_type` (1578) | `write_application_primitive_data_type`, `write_application_array_data_type`, `write_application_record_data_type`, `write_implementation_data_type` |
| `base_types.rs` | `write_sw_base_type` (1695) | `write_sw_base_type` |
| `collection.rs` | `write_collection` (1761) | `write_collection` |
| `life_cycle.rs` | `write_life_cycle_period` (1837), `write_life_cycle_info` (1857), `write_life_cycle_info_set` (1911) | `write_life_cycle_info_set` |
| `physical_dimension.rs` | `write_physical_dimension` (1990) | `write_physical_dimension` |
| `unit.rs` | `write_unit` (2047) | `write_unit` |
| `mod.rs` keeps | struct `ARXMLWriter`, `WriterOptions`, `write_ar_packages` (314), `write_ar_package` (334), `write_ar_package_element` (1633), tests: `fixture_document`, `save_emits_namespaced_root_and_preserved_whitespace`, `unescape_entities_option_unescapes_attribute_quotes` | — |

### The uniform move recipe (used by every task)

Every domain file starts with a one-line header and `use super::*;` — a child module's glob import brings the parent's items (struct, `Node`, `ParseError`, `Reader`, arena types), so no import hunting is needed. Glob imports never trigger unused-import warnings, so clippy stays clean even as imports become unnecessary in `mod.rs` (remove a `mod.rs` `use` only when the compiler names it as unused).

```rust
//! <Domain> readers (py read<Family> family). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // moved functions, verbatim, with the pub(super)/private visibility from the table
}
```

Test modules move inside the same file: `#[cfg(test)] mod tests { use super::*; … }`.

**Registration:** an unregistered file is not compiled — every task that creates a domain file also adds its `mod <name>;` line to the parent `mod.rs` (under the existing `use` block) in the same step. Tasks 9/18 consolidate and verify the full list.

**Visibility convention (standardized after Task 2):** cross-module payload structs are `pub(super) struct` with `pub(super)` fields — compiler-forced when `mod.rs` destructures the payload and when a `pub(super)` fn returns the type; do not "fix" this to private. Payload structs live near the TOP of their domain file (matches `LifeCycleInfoSetPayload` in mod.rs).

**Branch note (recorded after Task 2):** the S/T tests (`AR_OBJECT_S_T_SAMPLE` / `ar_object_s_t_attributes_parse_on_every_read_path` reader-side, `ar_object_s_t_attributes_emit_in_py_order` writer-side) exist only on `feature/sync-arobject` (PR #18) — not an ancestor of this branch. When that PR merges, those tests arrive in the moved file's test module and must be relocated into the `common.rs` test modules (reader: `src/reader/arxml_reader/common.rs`; writer: `src/writer/arxml_writer/common.rs`), not left in `mod.rs`. Task 10/18 Step 1 includes the check.

**After every task, run the full gate:**

```bash
cargo build && cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test
```

Expected final line of `cargo test`: `test result: ok.` for every suite (8 suites at plan time); expected clippy line: `Finished \`dev\` profile ...` with zero warnings. Fix nothing else; if a gate fails for a reason this plan does not predict, stop and investigate — do not adapt the plan silently.

---

### Task 0: rename the reader layer (`parser` → `reader`)

**Files:**
- Rename: `src/parser/` → `src/reader/`, `src/parser/abstract_arxml_parser.rs` → `src/reader/abstract_arxml_reader.rs`, `src/parser/arxml_parser.rs` → `src/reader/arxml_reader.rs`
- Modify: `src/lib.rs`, `src/bin/arxml-dump.rs`, `src/bin/arxml-format.rs`, `tests/integration/all_fixtures.rs`, `tests/integration/format_byte_roundtrip.rs`, `tests/integration/roundtrip.rs`, `tests/integration/arxml_format.rs`

- [ ] **Step 1: Mechanical rename** — move the files, then rewrite exactly five identifier mappings per file (everything else — `ParseError`, prose, py's own `ARXMLParser` class name — stays):

```bash
git mv src/parser src/reader
git mv src/reader/abstract_arxml_parser.rs src/reader/abstract_arxml_reader.rs
git mv src/reader/arxml_parser.rs src/reader/arxml_reader.rs

grep -rl 'ARXMLParser\|ParserOptions\|arxml_parser\|mod parser\|parser::' src tests --include='*.rs' | while read -r f; do
  sed -i '' -e 's/ARXMLParser/ARXMLReader/g' \
            -e 's/ParserOptions/ReaderOptions/g' \
            -e 's/abstract_arxml_parser/abstract_arxml_reader/g' \
            -e 's/arxml_parser/arxml_reader/g' \
            -e 's/mod parser/mod reader/' \
            -e 's/parser::/reader::/g' "$f"
done
```

(`sed` applies the expressions in order, so `arxml_parser::` collapses to `arxml_reader::` before the generic `parser::` rule runs; lib.rs becomes `pub mod reader;` / `pub use reader::arxml_reader::{default_options, ARXMLReader, ReaderOptions};` / `pub use reader::abstract_arxml_reader::ParseError;`.)

- [ ] **Step 2: Verify nothing was missed**

```bash
grep -rn 'ARXMLParser\|ParserOptions\|arxml_parser\|mod parser\|parser::' src tests --include='*.rs' | grep -v 'ParseError' | grep -v 'AbstractARXMLParser'
```

Expected: only py-citation comment lines survive (`AbstractARXMLParser`, `py ARXMLParser`, `arxml_parser.py` / `abstract_arxml_parser.py` citations) — those keep py-armodel's real names; every code token is renamed.

- [ ] **Step 3: Full gate** — expected: all green. The harness exercises the renamed API end-to-end (all four integration files import `armodel::reader::arxml_reader`).

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "refactor!: rename the reader layer (src/parser -> src/reader, ARXMLParser -> ARXMLReader)"
```

(The crate is 0.x with no external consumers; the public-API rename is deliberate per the spec's Non-goals.)

### Task 1: reader — convert `arxml_reader.rs` to `arxml_reader/mod.rs`

**Files:**
- Create: `src/reader/arxml_reader/mod.rs` (from `src/reader/arxml_reader.rs`)

- [ ] **Step 1: Pure file move**

```bash
mkdir -p src/reader/arxml_reader
git mv src/reader/arxml_reader.rs src/reader/arxml_reader/mod.rs
```

`src/reader/mod.rs` keeps `pub mod arxml_reader;` — Rust resolves the directory module automatically. `lib.rs` and all call sites are untouched.

- [ ] **Step 2: Verify checklist scanner still finds methods in the directory module**

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --emit-port-checklist
git diff --stat docs/port_checklist.md   # expected: no output (unchanged)
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check && echo CHECK-OK
```

Expected: `CHECK-OK`. If the diff is non-empty, the scanner does not recurse into directories — fix `tools/py2rust/port_checklist.py`'s source scan to walk recursively, re-run, and note it in the commit message.

- [ ] **Step 3: Full gate** — expected: all green (nothing semantic changed).

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "refactor(reader): arxml_reader.rs -> arxml_reader/mod.rs (domain split 1/2, pure move)"
```

### Task 2: reader — `common.rs` (abstract-level helpers)

**Files:**
- Create: `src/reader/arxml_reader/common.rs`
- Modify: `src/reader/arxml_reader/mod.rs`

- [ ] **Step 1: Create the file with moved content**

```rust
//! ARObject-level helpers shared by every domain reader (py's abstract-level
//! readARObject / readIdentifiable chain). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    /// py `readARObject` — reads the `S` (checksum) and `T` (timestamp)
    /// attributes.
    pub(super) fn read_ar_object(&mut self, element: &Node, ar_object: &mut ARObject) {
        // body verbatim from mod.rs
    }

    // read_xml_space — body verbatim; pub(super)
    // read_identifiable_payload — body verbatim; pub(super)
}

/// The `Identifiable`-owned XML payload read by `read_identifiable_payload`
/// (py `readIdentifiable` minus SHORT-NAME/UUID/CATEGORY, which the
/// allocation path already consumed, and minus annotations — no pinned
/// fixture carries `ANNOTATIONS`; tracked on the port checklist).
struct IdentifiablePayload {
    // fields verbatim from mod.rs
}
```

Move these out of `mod.rs` (locate with `grep -n "fn read_ar_object\|fn read_xml_space\|fn read_identifiable_payload\|struct IdentifiablePayload" src/reader/arxml_reader/mod.rs`): `read_ar_object` (~208-219), `read_xml_space` (~220-238), `read_identifiable_payload` (~736-758 incl. doc comment), `struct IdentifiablePayload` (~85-92 incl. doc comment). Make the three functions `pub(super)`. Delete them from `mod.rs`.

- [ ] **Step 2: Move the S/T test** — move `ar_object_s_t_attributes_parse_on_every_read_path` + `const AR_OBJECT_S_T_SAMPLE` from `mod.rs`'s `mod tests` into a new `#[cfg(test)] mod tests { use super::*; }` in `common.rs` (bodies verbatim).

- [ ] **Step 3: Full gate.** Expected: compiler names nothing extra; if a sibling already used one of these via the same-module path and now errors, mark that call site's target `pub(super)` (it will be `pub(super)` already per the table).

- [ ] **Step 4: Commit** — `git add -A && git commit -m "refactor(reader): common domain module (read_ar_object/xml_space/identifiable_payload)"`

### Task 3: reader — `documentation.rs`

**Files:** Create `src/reader/arxml_reader/documentation.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! Multilanguage long-name / overview-paragraph / documentation-block readers
//! (py readMultilanguageReferrable chain). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // get_multilanguage_long_name (537-583) — verbatim; pub(super)
    // get_multi_language_overview_paragraph (584-621) — verbatim; pub(super)
    // get_documentation_block (622-637) — verbatim; pub(super)
    // read_documentation_block (638-705) — verbatim; private
    // read_paginateable (706-735) — verbatim; private
}
```

- [ ] **Step 2: Move the text-node test** — `identifiable_payload_round_trips_text_nodes` + `const IDENTIFIABLE_SAMPLE` move from `mod.rs` tests into `documentation.rs`'s test module (verbatim; it exercises exactly these helpers).

- [ ] **Step 3: Full gate.**
- [ ] **Step 4: Commit** — `refactor(reader): documentation domain module (long-name/desc/introduction readers)`

### Task 4: reader — `admin_data.rs`

**Files:** Create `src/reader/arxml_reader/admin_data.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! ADMIN-DATA / SDG readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // read_admin_data (239-300) — verbatim; pub(super)
    // read_sdg (301-374) — verbatim; private
}
```

- [ ] **Step 2: Move the admin-data test with a self-contained sample** (it currently reuses `mod.rs`'s `parse_sample()`; replace that call with a local const so the test module is independent):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::msr::documentation::text_model::language_data_model::XmlSpace;

    const ADMIN_DATA_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd">
  <ADMIN-DATA>
    <USED-LANGUAGES>
      <L-10 L="EN" xml:space="preserve">English</L-10>
    </USED-LANGUAGES>
    <SDGS>
      <SDG GID="demo">
        <SD GID="purpose" xml:space="preserve">special   data</SD>
      </SDG>
    </SDGS>
  </ADMIN-DATA>
</AUTOSAR>"#;

    #[test]
    fn admin_data_round_trips_through_the_model() {
        let mut document = Document::new();
        ARXMLReader::new(default_options())
            .load_from_reader(Reader::from_str(ADMIN_DATA_SAMPLE), &mut document)
            .unwrap();

        let admin_data = document.get_admin_data().unwrap();

        let sdg = document.get_sdg(admin_data.get_sdgs()[0]).unwrap();
        assert_eq!(sdg.get_gid(), Some("demo"));

        let contents = document
            .get_sdg_contents(sdg.get_sdg_contents_type().unwrap())
            .unwrap();
        let sd = document.get_sd(contents.get_sd()[0]).unwrap();
        assert_eq!(sd.get_gid(), Some("purpose"));
        assert_eq!(sd.get_xml_space(), Some(XmlSpace::Preserve));
        assert_eq!(sd.get_value(), Some("special   data"));

        let mlpt = document
            .get_multi_language_plain_text(admin_data.get_used_languages().unwrap())
            .unwrap();
        let l10 = document.get_l_plain_text(mlpt.get_l10s()[0]).unwrap();
        assert_eq!(l10.get_l(), Some("EN"));
        assert_eq!(l10.get_value(), Some("English"));
    }
}
```

(The original `ADMIN-DATA` block in `SAMPLE` is unchanged by this substitution — same elements, same values.)

- [ ] **Step 3: Full gate.**
- [ ] **Step 4: Commit** — `refactor(reader): admin_data domain module (read_admin_data/read_sdg)`

### Task 5: reader — `compu_method.rs`

**Files:** Create `src/reader/arxml_reader/compu_method.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! COMPU-METHOD readers (py readCompuMethod → getCompu → getCompuScales →
//! readCompuScale → contents). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // get_compu_const_content (797-838) — verbatim; private
    // get_compu_const (839-856) — verbatim; private
    // read_compu_scale (857-939) — verbatim; private
    // get_compu_scales (940-963) — verbatim; private
    // get_compu (964-984) — verbatim; private
    // read_compu_method (985-1027) — verbatim; pub(super)  [dispatch target]
}
```

- [ ] **Step 2: Full gate.**
- [ ] **Step 3: Commit** — `refactor(reader): compu_method domain module`

### Task 6: reader — `data_constr.rs`

**Files:** Create `src/reader/arxml_reader/data_constr.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! DATA-CONSTR readers (py readDataConstr → readDataConstrRule → constrs;
//! get_child_limit_element is the shared LIMIT helper). Part of the
//! arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // read_scale_constr (1028-1062) — verbatim; private
    // read_data_constr (1063-1184) — verbatim; pub(super)  [dispatch target]
    // get_child_limit_element (1884-1911) — verbatim; private
}
```

- [ ] **Step 2: Full gate.**
- [ ] **Step 3: Commit** — `refactor(reader): data_constr domain module`

### Task 7: reader — `keyword.rs`

**Files:** Create `src/reader/arxml_reader/keyword.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! KEYWORD / KEYWORD-SET readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // read_keyword (1185-1235) — verbatim; private
    // read_keyword_set (1236-1270) — verbatim; pub(super)  [dispatch target]
}
```

- [ ] **Step 2: Full gate.**
- [ ] **Step 3: Commit** — `refactor(reader): keyword domain module`

### Task 8: reader — `datatypes.rs`

**Files:** Create `src/reader/arxml_reader/datatypes.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! Application/Implementation datatype readers incl. the shared
//! SW-DATA-DEF-PROPS wrapper. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // get_sw_data_def_props (1271-1361) — verbatim; private
    // read_application_primitive_data_type (1362-1390) — verbatim; pub(super)
    // read_application_array_data_type (1391-1486) — verbatim; pub(super)
    // read_application_record_data_type (1487-1560) — verbatim; pub(super)
    // read_implementation_data_type (1561-1603) — verbatim; pub(super)
}
```

- [ ] **Step 2: Full gate.**
- [ ] **Step 3: Commit** — `refactor(reader): datatypes domain module`

### Task 9: reader — leaf domains (`base_types`, `collection`, `life_cycle`, `physical_dimension`, `unit`)

**Files:** Create `src/reader/arxml_reader/{base_types,collection,life_cycle,physical_dimension,unit}.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the five files**, each with the standard skeleton (`//!` header, `use super::*;`, `impl ARXMLReader { … }`):

`base_types.rs`:
```rust
//! SW-BASE-TYPE readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // read_sw_base_type (1604-1650) — verbatim; pub(super)  [dispatch target]
    // read_base_type_direct_definition (1912-1945) — verbatim; private
}
```

`collection.rs`:
```rust
//! Collection readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // read_collection (1651-1721) — verbatim; pub(super)  [dispatch target]
}
```

`life_cycle.rs`:
```rust
//! LifeCycleInfoSet readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // read_life_cycle_info_set (1722-1762) — verbatim; pub(super)  [dispatch target]
    // get_life_cycle_period (1946-1968) — verbatim; private
    // read_life_cycle_info (1969-2000) — verbatim; private
    // read_life_cycle_info_set_payload (2001-2042) — verbatim; private
}
```

`physical_dimension.rs`:
```rust
//! PhysicalDimension readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // read_physical_dimension (1763-1819) — verbatim; pub(super)  [dispatch target]
}
```

`unit.rs`:
```rust
//! Unit readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    // read_unit (1820-1883) — verbatim; pub(super)  [dispatch target]
}
```

- [ ] **Step 2: Register the child modules in `mod.rs`** — add directly under the existing `use` block in `src/reader/arxml_reader/mod.rs`:

```rust
mod admin_data;
mod base_types;
mod collection;
mod common;
mod compu_method;
mod data_constr;
mod datatypes;
mod documentation;
mod keyword;
mod life_cycle;
mod physical_dimension;
mod unit;
```

(`common`/`documentation`/`admin_data` etc. were created without registration in Tasks 2–8 only if the compiler didn't demand it — with this task, all twelve are declared once, here. If Tasks 2–8 already needed the `mod` lines to compile, consolidate them here exactly as shown and note it.)

- [ ] **Step 3: Full gate.**
- [ ] **Step 4: Commit** — `refactor(reader): leaf domain modules (base_types/collection/life_cycle/physical_dimension/unit)`

### Task 10: reader close-out — `mod.rs` budget + checklist regen

**Files:** Modify `src/reader/arxml_reader/mod.rs`.

- [ ] **Step 1: Verify the mod.rs budget + refresh the header doc** — `wc -l src/reader/arxml_reader/mod.rs`. Expected: < 800 lines (struct + walk + dispatch + remaining tests; spec budget ~500 excluding tests). If larger, list what remains and move stragglers per Rule 1 of the spec before continuing. Also rewrite mod.rs's leading `//!` doc comment to describe what the file now holds (struct + options + load + tag walk + dispatch seams) — the original text predates the split and rots otherwise. If PR #18 has merged by now, relocate its S/T tests into `common.rs` per the branch note above (separate commit `refactor(reader): relocate S/T tests from PR #18 into common.rs`).

- [ ] **Step 2: Regenerate + verify the checklist**

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --emit-port-checklist
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check && echo CHECK-OK
git diff --stat docs/port_checklist.md   # expected: unchanged
```

- [ ] **Step 3: Full gate + commit** — `refactor(reader): domain split complete (12 domain modules, pure moves)`

### Task 11: writer — convert `arxml_writer.rs` to `arxml_writer/mod.rs`

**Files:** Create `src/writer/arxml_writer/mod.rs` (from `src/writer/arxml_writer.rs`).

- [ ] **Step 1: Pure file move**

```bash
mkdir -p src/writer/arxml_writer
git mv src/writer/arxml_writer.rs src/writer/arxml_writer/mod.rs
```

- [ ] **Step 2: Checklist verify** — same commands as Task 1 Step 2; expected `CHECK-OK`, checklist unchanged.
- [ ] **Step 3: Full gate.**
- [ ] **Step 4: Commit** — `refactor(writer): arxml_writer.rs -> arxml_writer/mod.rs (domain split 2/2, pure move)`

### Task 12: writer — `common.rs`

**Files:** Create `src/writer/arxml_writer/common.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! ARObject/Identifiable attribute emitters shared by every domain writer
//! (py writeARObject / writeIdentifiable attribute parts). Part of the
//! arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLWriter {
    // write_ar_object_attributes (160-169) — verbatim; pub(super)
    // write_identifiable_attributes (445-464) — verbatim; pub(super)
    // write_identifiable_parts (465-504) — verbatim; pub(super)
}
```

- [ ] **Step 2: Move the S/T writer test** — `ar_object_s_t_attributes_emit_in_py_order` moves from `mod.rs` tests into `common.rs`'s `#[cfg(test)] mod tests { use super::*; }` (body verbatim).
- [ ] **Step 3: Full gate.**
- [ ] **Step 4: Commit** — `refactor(writer): common domain module (ar_object/identifiable attribute emitters)`

### Task 13: writer — `documentation.rs`

**Files:** Create `src/writer/arxml_writer/documentation.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! Multilanguage / documentation-block emitters. Part of the arxml_writer
//! domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLWriter {
    // set_multi_long_name (505-539) — verbatim; pub(super)
    // set_multi_language_overview_paragraph (540-575) — verbatim; pub(super)
    // write_documentation_block (576-588) — verbatim; pub(super)
    // write_documentation_block_content (589-650) — verbatim; private
    // write_paginateable_attrs (651-664) — verbatim; private
}
```

- [ ] **Step 2: Full gate.**
- [ ] **Step 3: Commit** — `refactor(writer): documentation domain module`

### Task 14: writer — `admin_data.rs`

**Files:** Create `src/writer/arxml_writer/admin_data.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! ADMIN-DATA / SDG emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLWriter {
    // write_admin_data (170-238 incl. the empty-element self-closing branch) — verbatim; pub(super)
    // write_sdg (239-298) — verbatim; pub(super)
    // write_sd (299-313) — verbatim; private
}
```

- [ ] **Step 2: Full gate.**
- [ ] **Step 3: Commit** — `refactor(writer): admin_data domain module`

### Task 15: writer — `compu_method.rs`

**Files:** Create `src/writer/arxml_writer/compu_method.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! COMPU-METHOD emitters (py writeCompuMethod / setCompu family). Part of the
//! arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLWriter {
    // write_compu_method (665-726) — verbatim; pub(super)  [dispatch target]
    // set_compu (727-755) — verbatim; private
    // set_compu_scales (756-772) — verbatim; private
    // write_compu_scale (773-859) — verbatim; private
    // write_nominator_denominator (860-874) — verbatim; private
    // set_compu_const (875-907) — verbatim; private
}
```

- [ ] **Step 2: Full gate.**
- [ ] **Step 3: Commit** — `refactor(writer): compu_method domain module`

### Task 16: writer — `data_constr.rs` + `keyword.rs`

**Files:** Create `src/writer/arxml_writer/data_constr.rs`, `src/writer/arxml_writer/keyword.rs`; modify `mod.rs`.

- [ ] **Step 1: Create `data_constr.rs`**

```rust
//! DATA-CONSTR emitters (py writeDataConstr family; writer rule order is
//! CONSTR-LEVEL, PHYS-CONSTRS, INTERNAL-CONSTRS). Part of the arxml_writer
//! domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLWriter {
    // write_data_constr (908-962) — verbatim; pub(super)  [dispatch target]
    // write_data_constr_rule (963-989) — verbatim; private
    // write_internal_constrs (990-1031) — verbatim; private
    // write_phys_constrs (1032-1080) — verbatim; private
    // write_scale_constrs (1081-1126) — verbatim; private
}
```

- [ ] **Step 2: Create `keyword.rs`**

```rust
//! KEYWORD / KEYWORD-SET emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLWriter {
    // write_keyword (1127-1179) — verbatim; private
    // write_keyword_set (1180-1232) — verbatim; pub(super)  [dispatch target]
}
```

- [ ] **Step 3: Full gate.**
- [ ] **Step 4: Commit** — `refactor(writer): data_constr + keyword domain modules`

### Task 17: writer — `datatypes.rs`

**Files:** Create `src/writer/arxml_writer/datatypes.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the file**

```rust
//! Application/Implementation datatype emitters incl. the shared
//! SW-DATA-DEF-PROPS wrapper. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLWriter {
    // set_sw_data_def_props (1233-1314) — verbatim; private
    // write_autosar_data_type_parts (1315-1329) — verbatim; private
    // write_application_primitive_data_type (1330-1372) — verbatim; pub(super)
    // write_composite_element_prototype_body (1373-1398) — verbatim; private
    // write_application_array_data_type (1399-1496) — verbatim; pub(super)
    // write_application_record_data_type (1497-1577) — verbatim; pub(super)
    // write_implementation_data_type (1578-1632) — verbatim; pub(super)
}
```

- [ ] **Step 2: Full gate.**
- [ ] **Step 3: Commit** — `refactor(writer): datatypes domain module`

### Task 18: writer leaf domains + module registration

**Files:** Create `src/writer/arxml_writer/{base_types,collection,life_cycle,physical_dimension,unit}.rs`; modify `mod.rs`.

- [ ] **Step 1: Create the five files** with the standard skeleton:

`base_types.rs`: `write_sw_base_type` (1695-1760) — `pub(super)`.
`collection.rs`: `write_collection` (1761-1836) — `pub(super)`.
`life_cycle.rs`: `write_life_cycle_period` (1837-1856, private), `write_life_cycle_info` (1857-1910, private), `write_life_cycle_info_set` (1911-1989, `pub(super)`).
`physical_dimension.rs`: `write_physical_dimension` (1990-2046, `pub(super)`).
`unit.rs`: `write_unit` (2047-~2100, `pub(super)`).

Each file's exact shape (using base_types as the template; the others differ only in the `//!` line naming their family and the functions listed above):

```rust
//! SW-BASE-TYPE emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLWriter {
    // write_sw_base_type — body verbatim from mod.rs; pub(super)  [dispatch target]
}
```

- [ ] **Step 2: Register all writer child modules in `mod.rs`** (under the existing `use` block, mirroring Task 9 Step 2):

```rust
mod admin_data;
mod base_types;
mod collection;
mod common;
mod compu_method;
mod data_constr;
mod datatypes;
mod documentation;
mod keyword;
mod life_cycle;
mod physical_dimension;
mod unit;
```

- [ ] **Step 3: Full gate.**
- [ ] **Step 4: Commit** — `refactor(writer): leaf domain modules (base_types/collection/life_cycle/physical_dimension/unit)`

### Task 19: rules into `docs/code_guide.md` + close-out

**Files:** Modify `docs/code_guide.md` (§8 Parser and writer patterns — append at the end of the section).

- [ ] **Step 1: Align the §8 heading with the rename, then append the placement/visibility rules** — retitle `## 8. Parser and writer patterns` to `## 8. Reader and writer patterns` and fix §8's prose `parser` references to `reader` (code_guide lines referencing `src/parser/` paths became stale in Task 0). Then append:

```markdown
### Domain split (added 2026-10-04)

The hand-written reader/writer are split into per-domain modules
(`src/reader/arxml_reader/<domain>.rs`, `src/writer/arxml_writer/<domain>.rs`):

- A method lives in the domain module of the family it reads/writes; shared
  base helpers live in `common.rs`. Placement is a pure move — methods stay on
  the same struct; no call site changes.
- Methods default private; anything called from `mod.rs` dispatch or a sibling
  domain module is `pub(super)` — never `pub(crate)` or `pub`.
- Tests live with their domain; `tests/integration/` is untouched and remains
  the only contract.
- `mod.rs` holds struct + options + load + tag walk + dispatch arms only; past
  ~500 lines (excluding tests) something belongs in a domain module.
- New P2–P4 batches add their methods to the owning domain module and, for
  package-element tags, one delegating dispatch arm in `mod.rs`.
```

- [ ] **Step 2: Budget check both mod.rs files** — `wc -l src/reader/arxml_reader/mod.rs src/writer/arxml_writer/mod.rs`; both < 800 lines including tests.

- [ ] **Step 3: Final gate + checklist**

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --emit-port-checklist
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check && echo CHECK-OK
cargo build && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test
```

Expected: `CHECK-OK`, checklist unchanged, all suites `test result: ok.`.

- [ ] **Step 4: Commit, push, PR**

```bash
git add -A && git commit -m "docs: domain-split placement rules in code_guide §8 (Closes #<issue>)"
git push -u origin feature/parser-writer-domain-split
gh pr create --base main --title "refactor: rename reader layer + split reader/writer into per-domain modules" \
  --body "Closes #<issue> — pure-move domain split per docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md. Includes the parser→reader rename (Task 0) and the pure-move domain split; every task ran the full gate. Prepares the P2–P4 mass port to land ~2,400 more methods in small files."
```

---

**Done when:** both `mod.rs` files hold only struct/walk/dispatch/tests; 12 domain modules exist on each side; the full harness (model equality 32/32, zero-warning gate, byte-identity gate) is green at every commit; port-checklist regen + `--check` unchanged throughout; code_guide documents the placement rules.
