# rust-armodel — P0 Walking Skeleton Design

Date: 2026-10-01
Status: Approved for planning
Scope: Phase P0 only

## 1. Context

`rust-armodel` is a Rust library to parse and generate AUTOSAR ARXML. Today the crate is a
stub (~108 LOC): an `ARObject`/`PackageableElement`/`ReferenceBase`/`ARPackage` skeleton, an
empty `AUTOSAR` struct, and an `arxml-dump` binary that parses CLI args and does nothing.

`py-armodel` (https://github.com/melodypapa/py-armodel/) is the mature reference:
~1,987 model classes (107k LOC) under `src/armodel/models/M2/`, a 15.8k-line hand-written
parser (`src/armodel/parser/arxml_parser.py`), and a 15.4k-line writer
(`src/armodel/writer/arxml_writer.py`). Its contract is round-trip integrity, enforced over
133 authentic ARXML files in `tests/integration_tests/test_files/`.

The long-term goal is a **full 1:1 parity port** of py-armodel to Rust, with a **redesigned
parser and writer**. This document specifies only the first increment.

## 2. Goal and non-goals

### Goal (P0)

Build a **walking skeleton** that proves the full pipeline end to end on one real ARXML file:

parse `AdminDataWhitespace.arxml` → typed Rust model → write ARXML → re-parse → the two
models compare equal.

P0 exists to validate four foundational choices on real data before investing in the
converter and the mass port: the arena-based typed model, quick-xml I/O, the
Document-centric mutation API, and the round-trip test harness.

### Non-goals (P0)

- No Python→Rust converter. P0 model classes are hand-written. (Converter is P1.)
- No schema-driven engine. P0 parser/writer are hand-written, mirroring py-armodel's
  structure so the later mechanical port stays mechanical. (Redesign is P5.)
- No CLI parity, no docs publishing.
- No coverage of the other 132 ARXML files.
- No general `ARPackage` element directory (the hundreds of `createXxx` factories).

### Acceptance

A Rust integration test passes:

1. Parse `tests/integration/test_files/AdminDataWhitespace.arxml` into a `Document`.
2. Write it to a temp file.
3. Parse the temp file into a second `Document`.
4. Assert the two `Document`s are **structurally equal** (see §7).
5. Assert the whitespace-preserving field round-trips text exactly: `"special   data"`
   (three internal spaces) is unchanged.

The bar is **model equality only** — written-text comparison is explicitly out of scope for
P0.

## 3. Locked decisions

| Decision | Choice |
|---|---|
| Target | Full 1:1 parity with py-armodel (north star); P0 is the first slice |
| Parser/writer improvement | Rust-side redesign, delivered later (P5); P0 mirrors py's shape |
| Parity bar | Pass py's integration round-trip semantics on the authentic files |
| Model layer | Typed struct per AUTOSAR class, converted from Python M2 (P1) |
| Struct style | Private typed fields + `get_`/`set_` methods mapped from py `getX`/`setX` |
| Links | `Id<T>` handles into per-type arenas (no `unsafe`) |
| XML library | `quick-xml` (pure Rust, stable, pull parser + writer) |
| Rust | Stable toolchain, no nightly; edition 2021 |
| Generated code | Committed `.rs` (applies from P1 onward) |
| Test assertion | Model equality only |
| Module structure | Derived from the AUTOSAR spec markdown `\| Package \| M2::… \|` rows in `py-armodel/autosar/<release>/markdown/` |
| Interface-marked types | Concrete Rust structs, matching py-armodel |

### Structural source of truth

Each AUTOSAR class is defined in the spec markdown (`py-armodel/autosar/R23-11/markdown/*.md`)
with a class table containing an explicit package row, e.g.:

```
| Class   | ARPackage |
| Package | M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ARPackage |
```

**Rule:** every `::` segment of the `Package` row becomes a `snake_case` Rust module; one
module per package; the class name is used unchanged as the Rust type name. Non-leaf
packages (those containing only sub-packages) are `mod.rs` module declarations. This mirrors
py-armodel's own `Foo.py` / `Foo/__init__.py` convention and makes the Rust tree verifiable
against the spec.

Some types are marked `(interface)` in the spec but implemented as concrete classes in
py-armodel (e.g. `LPlainText`, `SdgContents`). Per the locked decision, they are modelled as
**concrete Rust structs** so behaviour and round-trip parity match py-armodel; the divergence
is recorded here rather than resolved in favour of the spec.

## 4. Architecture

Single library crate `armodel`. The module tree mirrors the AUTOSAR spec package paths
(§3), so the later mechanical port maps cleanly.

```
rust-armodel/
├── Cargo.toml                      # edition 2021; deps: quick-xml, id_arena, getopts
├── src/
│   ├── lib.rs
│   ├── m2/                                     # module tree = spec Package paths
│   │   ├── autosar_templates/
│   │   │   ├── autosar_top_level_structure.rs  # M2::AUTOSARTemplates::AutosarTopLevelStructure
│   │   │   └── generic_structure/
│   │   │       ├── mod.rs                      # M2::AUTOSARTemplates::GenericStructure
│   │   │       └── general_template_classes/
│   │   │           ├── mod.rs                  # …::GeneralTemplateClasses
│   │   │           ├── ar_object.rs            #   ::ArObject          -> ARObject
│   │   │           ├── identifiable.rs         #   ::Identifiable      -> Referrable, MultilanguageReferrable, Identifiable, Describable, …
│   │   │           ├── element_collection.rs   #   ::ElementCollection -> CollectableElement, Collection
│   │   │           └── ar_package.rs           #   ::ARPackage         -> PackageableElement, ARElement, ARPackage, ReferenceBase
│   │   └── msr/
│   │       ├── asam_hdo/
│   │       │   ├── admin_data.rs               # M2::MSR::AsamHdo::AdminData   -> AdminData, DocRevision, Modification
│   │       │   └── special_data.rs             # M2::MSR::AsamHdo::SpecialData -> Sd, Sdf, Sdg, SdgCaption, SdgContents
│   │       └── documentation/
│   │           ├── mod.rs                      # M2::MSR::Documentation
│   │           └── text_model/
│   │               ├── mod.rs                  # M2::MSR::Documentation::TextModel
│   │               ├── multilanguage_data.rs   #   ::MultilanguageData  -> MultiLanguagePlainText, MultiLanguageOverviewParagraph
│   │               └── language_data_model.rs  #   ::LanguageDataModel  -> LPlainText, WhitespaceControlled, LanguageSpecific
│   ├── parser/
│   │   ├── abstract_arxml_parser.rs             # Node DOM + find/findall/get_child_element helpers
│   │   └── arxml_parser.rs                      # load(), read_admin_data(), read_ar_packages()
│   └── writer/
│       ├── abstract_arxml_writer.rs             # quick-xml Writer + element emitters
│       └── arxml_writer.rs                      # save(), write_admin_data(), write_ar_packages()
└── tests/integration/
    ├── test_files/AdminDataWhitespace.arxml     # copied from py-armodel
    └── roundtrip.rs
```

`Document` (the root type, replacing py's `AUTOSAR`) lives in
`autosar_top_level_structure.rs`, the module for spec class `AUTOSAR`
(`M2::AUTOSARTemplates::AutosarTopLevelStructure`).

`src/bin/arxml-dump.rs` is retained and updated to compile against the new model API; its
behaviour is unchanged in P0.

### Data flow

```
file ──quick-xml events──> Node DOM ──ARXMLParser──> Document (arena of typed structs)
Document ──ARXMLWriter──> ARXML text ──ARXMLParser──> Document'   (compared for equality)
```

## 5. Model layer (P0 class surface)

P0 implements only the classes `AdminDataWhitespace.arxml` needs, plus the base chain. All
fields are private; accessors are `get_`/`set_` (setters return `&mut Self` for chaining),
mapped from py's `getX`/`setX`.

Base chain (hand-written, defines the pattern). Each entry notes its spec package → Rust
module; the class name is used unchanged:

- `ARObject { checksum: Option<String>, timestamp: Option<String> }`
  — `…GeneralTemplateClasses::ArObject` → `general_template_classes::ar_object`
- `Referrable : ARObject { parent: Option<ElementRef>, short_name: Option<String> }`
  — `…GeneralTemplateClasses::Identifiable` → `general_template_classes::identifiable`
- `MultilanguageReferrable : Referrable` (no new fields in P0) — same module
- `Identifiable : MultilanguageReferrable { uuid: Option<String>, category: Option<String>,
  admin_data: Option<Id<AdminData>> }` — same module; the remaining spec fields (desc,
  introduction, annotations, long_name) arrive in P1 from the converter.
- `CollectableElement : Identifiable`
  — `…GeneralTemplateClasses::ElementCollection` → `general_template_classes::element_collection`
- `PackageableElement : CollectableElement`
  — `…GeneralTemplateClasses::ARPackage` → `general_template_classes::ar_package`
- `ARElement : PackageableElement` — same module
- `ARPackage : CollectableElement { elements: Vec<ElementRef>, ar_packages: Vec<Id<ARPackage>>,
  reference_bases: Vec<Id<ReferenceBase>> }` — same module
- `ReferenceBase : ARElement` — same module
- `Document` (root, in place of py's `AUTOSAR`): specified by spec class `AUTOSAR`
  (`M2::AUTOSARTemplates::AutosarTopLevelStructure` → `autosar_templates::autosar_top_level_structure`).
  Owns every arena, plus `admin_data: Option<Id<AdminData>>`,
  `root_ar_packages: Vec<Id<ARPackage>>`, `schema_location: String`, `ar_release: String`.

Content classes (driven by the file; field names and shapes taken from py-armodel, module
placement from the spec `Package` rows):

- `AdminData { doc_revisions: Vec<Id<DocRevision>>, language: Option<String>,
  sdgs: Vec<Id<Sdg>>, used_languages: Option<Id<MultiLanguagePlainText>> }`
  — `M2::MSR::AsamHdo::AdminData` → `msr::asam_hdo::admin_data`. Note `used_languages` is a
  **single** `MultiLanguagePlainText`, not a list.
- `MultiLanguagePlainText { l10s: Vec<Id<LPlainText>> }`
  — `M2::MSR::Documentation::TextModel::MultilanguageData` → `…::text_model::multilanguage_data`
- `LPlainText { l: Option<String>, xml_space: Option<XmlSpace>, value: Option<String> }`
  — `M2::MSR::Documentation::TextModel::LanguageDataModel` → `…::text_model::language_data_model`
  (`l` from `LanguageSpecific`, `xml_space` from `WhitespaceControlled`, `value` from the
  mixed-content base). This is the `<L-10 L="EN" xml:space="preserve">English</L-10>` element.
  The spec marks `LPlainText` as an interface; per §3 it is a concrete struct.
- `Sd { gid: Option<String>, value: Option<String>, xml_space: Option<XmlSpace> }`
  — `M2::MSR::AsamHdo::SpecialData` → `msr::asam_hdo::special_data`. `xml:space` is an
  **explicit field**, which is how whitespace fidelity is preserved.
- `Sdf { gid: Option<String>, value: Option<String> }` — same module
- `SdgCaption : MultilanguageReferrable { desc: Option<Id<MultiLanguageOverviewParagraph>> }`
  — same module
- `SdgContents { sd: Vec<Id<Sd>>, sdf: Vec<Id<Sdf>>, sdg: Vec<Id<Sdg>> }` — same module. The
  spec marks it as an interface; modelled as a concrete struct per §3.
- `Sdg { gid: Option<String>, sdg_caption: Option<Id<SdgCaption>>,
  sdg_contents_type: Option<Id<SdgContents>> }` — same module

`DocRevision` and `MultiLanguageOverviewParagraph` are referenced by type but do not occur in
`AdminDataWhitespace.arxml`; they may remain empty placeholder structs in P0 and are filled in
P1 by the converter.

### Arenas and handles

- Each concrete class `T` has an arena `Arena<T>` (crate `id_arena`); instances are
  referenced by `Id<T>`.
- **Typed child links** use `Id<T>` (e.g. `ar_packages: Vec<Id<ARPackage>>`).
- **Heterogeneous links** — `Referrable::parent` and `ARPackage::elements` — use a
  generated-style `ElementRef` enum with one variant per element kind
  (`ElementRef::ARPackage(Id<ARPackage>)`, …). In P0 the enum is hand-written with a single
  `ARPackage(Id<ARPackage>)` variant (the only kind `AdminDataWhitespace.arxml` can produce);
  the P1 converter generates the full variant list. This is the type-erased handle that
  replaces py's untyped `parent: ARObject`.
- Mutation that creates a link goes through `Document`, because it owns the arenas, e.g.
  `doc.add_ar_package(parent, short_name) -> Id<ARPackage>` and
  `doc.add_element(pkg: Id<ARPackage>, e: ElementRef)`. Owned scalar fields keep plain
  struct accessors (`set_short_name`). This Document-centric shape is the deliberate cost of
  arena modelling; P0 validates that it is workable.

## 6. Parser (P0)

`abstract_arxml_parser.rs`:

- Build a `Node { name: String, attrs: BTreeMap<String, String>, children: Vec<Node>,
  text: Option<String> }` DOM from `quick_xml::Reader` events. Element names are
  namespace-stripped to local names (`{ns}TAG` → `TAG`), matching py's `getTagName`.
- Whitespace rule: when an element carries `xml:space="preserve"`, its text is captured
  verbatim (no trimming). Otherwise text is trimmed. This is required for the `SD` value
  `"special   data"`.
- Helpers mirroring py: `find(node, name) -> Option<&Node>`,
  `find_all(node, name) -> Vec<&Node>`, `get_child_element_string`. (Ref/UUID helpers
  arrive with the phases that need them.)

`arxml_parser.rs`:

- `load(path, doc: &mut Document) -> Result<(), ParseError>`:
  parse to DOM; assert root `AUTOSAR`; read `xmlns` + `xsi:schemaLocation`; map the XSD
  filename to a release using py's table (`AUTOSAR_00050.xsd` → `R21-11`,
  `default → R23-11`); set `doc.schema_location` / `doc.ar_release`.
- `read_admin_data` → `USED-LANGUAGES` → `L-10` elements collected into a
  `MultiLanguagePlainText`; `SDGS` → `SDG` elements. Each `SDG` reads its `GID`, an optional
  `SDG-CAPTION`, and collects child `SD`/`SDF`/`SDG` into an `SdgContents` (mirroring py's
  `readSd(self, element, contents: SdgContents)` at `arxml_parser.py:1403`, which receives
  the owning `SdgContents`). Each `SD` reads `GID` and `xml:space`, and captures its text.
- `read_ar_packages` → `AR-PACKAGE` (recursive: `AR-PACKAGES` may nest); each package reads
  `SHORT-NAME`, and for P0 the `ELEMENTS` directory is not yet dispatched (no elements occur
  in this file).
- Error model mirrors py: `ARXMLParser::new(options)` where `warning: true` (default)
  collects warnings and continues; `warning: false` returns `Err`. Unknown tags produce an
  "unsupported element" warning rather than a hard failure.

## 7. Equality semantics

py's `assert_models_equal` (tests/integration_tests/test_roundtrip.py) compares objects
recursively by exact `type()` then `__dict__`, ignoring the `parent` attribute.

In Rust the two `Document`s own **independent arenas**, so `Id<T>` values are not comparable
across documents. Equality is therefore implemented as a method on `Document`:

```rust
impl Document {
    /// Structural equality: walk both models, resolving Id<T> links within each
    /// Document, ignoring parent links.
    pub fn assert_structurally_equal(&self, other: &Document) -> Result<(), String>;
}
```

Semantics:

- Compare the ordered `root_ar_packages` list.
- Recurse into referenced objects by resolving ids in the owning `Document`.
- Compare scalar fields value-by-value, `Vec<Id<T>>` element-wise in order.
- Ignore `Referrable::parent` (mirrors py's `ignored_attrs = ["parent"]`).

Per-type `PartialEq` is not derived for linked fields; the comparison lives on `Document`
where the arenas are reachable.

## 8. Test harness (P0)

`tests/integration/roundtrip.rs`:

1. Copy `AdminDataWhitespace.arxml` from py-armodel into
   `tests/integration/test_files/` (verbatim).
2. `let mut doc1 = Document::default(); ARXMLParser::new(default_options()).load(path, &mut doc1)?;`
3. `ARXMLWriter::save(&tmp_path, &doc1)?;`
4. `let mut doc2 = Document::default(); ARXMLParser::new(default_options()).load(&tmp_path, &mut doc2)?;`
5. `doc1.assert_structurally_equal(&doc2)?;`
6. Assert the `SD` text value equals `"special   data"` exactly.

No written-text diff is asserted in P0 (decision: model equality only). A normalized text
diff may be added in a later phase.

## 9. Implementation steps

1. **Scaffold** — edition 2021; `Cargo.toml` deps `quick-xml`, `id_arena`, `getopts`;
   create the module tree per the spec `Package` paths (§3–§4); remove dead stubs
   (`autosar_top_level_structure.rs`'s invalid `str&` field) and the unused
   `regex`/`libxml` references.
2. **Model** — implement the base chain, `ElementRef`, `Document` with arenas, and the
   content classes from §5, with `get_`/`set_` accessors.
3. **DOM + abstract parser** — quick-xml `Node` DOM and the find/get helpers from §6.
4. **Parser** — `ARXMLParser::load` with release detection, `read_admin_data`,
   `read_ar_packages`.
5. **Writer** — `ARXMLWriter::save` emitting XML declaration, `AUTOSAR` with namespaces,
   `ADMIN-DATA`, `AR-PACKAGES`; 2-space indent; empty elements expanded.
6. **Harness** — copy the test file, implement `roundtrip.rs`, make it pass.
7. **Fix `arxml-dump.rs`** — update to the new `Document`/`ARPackage` API so `cargo build`
   and `cargo test` are clean.

## 10. Risks

| Risk | Mitigation |
|---|---|
| Heterogeneous `parent`/`elements` links need a type-erased handle | `ElementRef` enum, hand-written in P0, converter-generated in P1 |
| quick-xml DOM may not preserve text/whitespace exactly | Explicit `xml:space="preserve"` handling + the P0 whitespace assertion |
| Arena cross-type mutation is verbose | Document-centric API validated in P0 before committing to it in P1 |
| Release/XSD mapping drift | Table copied from py's `conftest.py::xsd_to_version_mapping` |
| `arxml-dump` bin breaks on the new model API | Explicit step 7; kept compiling throughout |

## 11. Follow-on phases (out of scope here)

- **P1** Converter: `tools/py2rust/` (Python `ast`) emits committed typed Rust model from
  py's `models/M2/**`, with **module placement taken from the spec markdown `Package` rows**
  (`py-armodel/autosar/<release>/markdown/*.md`), plus the generated `ElementRef` enum and
  the tag → constructor registry.
- **P2–P4** Mechanical parser and writer port, batched by AUTOSAR domain, expanded until all
  133 files round-trip.
- **P5** Redesign into a schema-driven single engine (the "improve the parser and writer"
  goal).
- **P6** CLI parity, docs, publishing.
