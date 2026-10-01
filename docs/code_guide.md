# armodel Rust Code Guide

Coding guidelines for the rust-armodel port. The reference implementation is
py-armodel (https://github.com/melodypapa/py-armodel/); the architecture decisions are locked in
`docs/superpowers/specs/2026-10-01-rust-armodel-p0-walking-skeleton-design.md`.
This guide translates those decisions into concrete Rust rules.

---

## 1. Ground rules

- Stable toolchain, **edition 2021**, no `unsafe`, no nightly features.
- XML I/O via **quick-xml**; identity via **id_arena** (`Arena<T>` / `Id<T>`).
- Errors with **thiserror**; the library never `panic!`s, `unwrap`s, or `expect`s
  on external input. Panics are allowed only in tests.
- `cargo fmt` + `cargo clippy -- -D warnings` must be clean before commit.
- Class names keep the AUTOSAR spelling (`ARPackage`, `AdminData`); modules,
  fields, and functions are `snake_case`. No Python `camelCase` leaks into
  function names (`getShortName` → `get_short_name`).

## 2. Module layout

Module paths mirror the AUTOSAR spec `Package` rows
(`py-armodel/autosar/<release>/markdown/*.md`):

```
| Package | M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ARPackage |
```

- Each `::` segment becomes a `snake_case` Rust module:
  `src/m2/autosar_templates/generic_structure/general_template_classes/ar_package.rs`.
- One class per file when the spec package is a leaf; non-leaf packages are
  `mod.rs` with only `pub mod` declarations.
- The class name is used unchanged as the Rust type name. Types the spec marks
  `(interface)` are still concrete structs (matching py-armodel).
- `lib.rs` re-exports only the public surface (`Document`, `ARPackage`,
  parser/writer types). Everything else is `pub(crate)` until needed.

Never leave a file outside the module tree (the current `identifier.rs` /
`autosar_top_level_structure.rs` orphans are dead code the compiler cannot
check).

## 3. Modelling: composition instead of inheritance

Python uses a deep base-class chain
(`ARObject → Referrable → MultilanguageReferrable → Identifiable → …`).
Rust has no inheritance; use **composition + accessor forwarding**:

```rust
/// spec: M2::…::GeneralTemplateClasses::ArObject
pub struct ARObject {
    checksum: Option<String>,
    timestamp: Option<String>,
}

/// spec: …::Identifiable — Referrable : ARObject
pub struct Referrable {
    base: ARObject,
    parent: Option<ElementRef>,
    short_name: String,
}

/// Identifiable : MultilanguageReferrable : Referrable
pub struct Identifiable {
    base: Referrable,
    uuid: Option<String>,
    category: Option<String>,
    admin_data: Option<Id<AdminData>>,
}
```

Rules:

- Exactly one embedded `base` field per level; never duplicate a base field
  (e.g. never re-declare `checksum` in a subclass).
- Accessors forward to `base` so the public API matches py-armodel 1:1
  (`Identifiable::get_short_name` forwards to `Referrable::get_short_name`).
- Use a **trait** only for shared *behaviour* across unrelated types (e.g.
  `ElementCollection` behaviour), never to emulate the class chain.
- Abstract classes in Python simply don't get a Rust struct if nothing
  instantiates them; only model classes that appear in the data flow.
- `Abstract`-marker Python classes that only exist for `isinstance` checks
  (see `ArObject.py`) become Rust `enum`s at the usage site (see §5).

## 4. Arenas, handles, and mutation

- Every concrete class `T` gets an `Arena<T>` owned by `Document`; instances
  are referenced by `Id<T>`.
- Typed links use `Id<T>` (`ar_packages: Vec<Id<ARPackage>>`).
- Heterogeneous links (`Referrable::parent`, `ARPackage::elements`) use the
  generated `ElementRef` enum:

```rust
pub enum ElementRef {
    ARPackage(Id<ARPackage>),
    // … one variant per element kind (P1 converter emits the full list)
}
```

- **All cross-type mutation goes through `Document`**, which owns the arenas:

```rust
impl Document {
    pub fn add_ar_package(&mut self, parent: Option<Id<ARPackage>>, short_name: &str)
        -> Id<ARPackage>;
    pub fn add_element(&mut self, pkg: Id<ARPackage>, e: ElementRef);
}
```

- Scalar field access stays on the struct with `get_`/`set_`:

```rust
impl Referrable {
    pub fn get_short_name(&self) -> &str { &self.short_name }
    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.short_name = value.into();
        self
    }
}
```

- Setters return `&mut Self` for chaining (py returns `self`).
- Do NOT store `Vec<ARPackage>` by value inside `ARPackage` (the current stub
  does this) — ownership recursion forces `Box` gymnastics and fights the
  parser. Use `Vec<Id<ARPackage>>`.
- Do NOT define `fn init()` traits to mimic Python `__init__`; construction is
  `Type::new(...)` or a `Document` factory method.

## 5. Enums

AUTOSAR enumeration types are proper Rust enums, not string fields:

```rust
pub enum XmlSpace { Preserve, Default }

pub enum ARRelease { R2111, R2203, /* … */ R2311 }
```

- Parse from text with `TryFrom<&str>`; format with `Display`.
- Unknown/invalid values are a parse *warning* (see §7), not a panic.

## 6. Optionality and multiplicity

Map the spec multiplicity directly:

| Spec        | Rust                            |
|-------------|---------------------------------|
| 0..1        | `Option<Id<T>>` / `Option<T>`   |
| 1           | plain field (set in `new`)      |
| 0..*        | `Vec<Id<T>>` / `Vec<T>`         |
| referenced  | `Id<T>` (never `Box<T>`)        |

Whitespace fidelity: mixed-content text elements (`SD`, `L-10`) keep
`xml_space: Option<XmlSpace>` as an **explicit field** — this is how
`"special   data"` survives the round trip.

## 7. Errors and warnings

```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected root element `{0}`")]
    UnexpectedRoot(String),
    #[error("invalid {element}: {reason}")]
    InvalidElement { element: String, reason: String },
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

- Mirror py's two modes: `ARXMLParser::new(ParserOptions { warning: true })`
  collects warnings and continues; `warning: false` fails on the first error.
  Unknown tags produce an "unsupported element" warning, never a hard failure.
- The CLI (`src/bin/arxml-dump.rs`) is the only place allowed to print an
  error and `exit(1)` — no `panic!`.

## 8. Parser and writer patterns

Parser (`src/parser/`):

- `abstract_arxml_parser.rs` builds a `Node` DOM
  (`name`, `attrs: BTreeMap<String,String>`, `children`, `text`) from
  quick-xml events; element names are namespace-stripped (`{ns}TAG` → `TAG`).
- Helpers mirror py's names: `find`, `find_all`, `get_child_element_string`.
- Whitespace rule: capture text verbatim when `xml:space="preserve"` is
  present, trim otherwise. One reader method owns this rule.
- Each reader method corresponds to one py method (`read_admin_data` ↔
  `readAdminData`) taking `&mut Document` + the relevant ids — this keeps the
  P2–P4 mechanical port reviewable method-by-method.

Writer (`src/writer/`):

- quick-xml `Writer` with 2-space indentation; empty elements expanded
  (`<TAG></TAG>`, not self-closing) to match py output.
- One emitter method per py writer method; element order follows the spec
  `xml.sequenceOffset` tags already encoded in py's field order.
- Never re-derive values (checksums, timestamps) — write what the model holds.

## 9. Equality

`Id<T>` is only meaningful inside one `Document`, so:

- Do **not** derive `PartialEq` on arena-linked types.
- Implement `Document::assert_structurally_equal(&self, other: &Document)
  -> Result<(), String>`: walk both models, resolve ids within each document,
  compare scalars and `Vec<Id<_>>` element-wise in order, ignore `parent`.
- Pure-value types (primitives, enums) may derive `PartialEq`/`Eq` freely.

## 10. Testing

- Unit tests live in the same file (`#[cfg(test)] mod tests`), one per py test
  where a counterpart exists; every `get_`/`set_`/factory method gets an
  assertion, not just a "does it construct" call (the current
  `create_arpackage_test` asserts nothing).
- Integration: `tests/integration/roundtrip.rs` — parse → write → re-parse →
  `assert_structurally_equal`, plus targeted assertions (e.g. the SD text).
- Test fixtures are verbatim copies of py's
  `tests/integration_tests/test_files/*.arxml`; never regenerate them.
- Round-trip over the full 133-file corpus is the P2–P4 bar; keep the harness
  parameterised so adding files is a one-line change.

## 11. Converting from py-armodel (checklist)

When porting a Python class:

1. Find its spec `Package` row → derive the Rust module path (§2).
2. List its Python attributes and methods (use the parity checklist in the
   py docstrings — it states which methods have reader/writer support).
3. Write the struct: embedded base, private fields per §4/§6.
4. Port `getX`/`setX` → `get_x`/`set_x`; `None`-tolerant setters keep the
   `if value is not None` semantics (setter takes the value and no-ops on
   `None` where py does; otherwise make the field `Option` and allow
   clearing).
5. Wire reader/writer methods for any attribute the parity checklist marks
   `[x] reader` / `[x] writer`.
6. Port the py unit tests.
7. `cargo fmt`, `cargo clippy -D warnings`, `cargo test`.

## 12. Anti-patterns (do not do)

- ❌ Python-style trait hierarchies, `fn init()` traits, or `&self` factory
  methods (mutation needs `&mut`).
- ❌ `Vec<Struct>` self-ownership for tree nodes.
- ❌ `pub` fields on model structs — always accessors.
- ❌ `unwrap`/`expect`/`panic!` in library code.
- ❌ camelCase functions or Python attribute names in XML-facing code.
- ❌ Deriving `PartialEq` across `Id<T>` links or comparing `Id`s from two
  documents.
- ❌ Stringly-typed AUTOSAR enumerations.
- ❌ Files not reachable from `lib.rs`.
