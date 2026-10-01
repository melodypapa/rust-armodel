# P0 Walking Skeleton Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prove the full pipeline end to end on one real file — parse `AdminDataWhitespace.arxml` → typed model → write ARXML → re-parse → the two `Document`s compare structurally equal, with the SD value `"special   data"` surviving verbatim.

**Architecture:** Arena-based typed model (`Document` owns one `Arena<T>` per concrete class; nodes reference each other via `Id<T>` handles and a hand-written `ElementRef` enum). A quick-xml pull parser builds a local `Node` DOM, then hand-written reader methods mirror py-armodel's `arxml_parser.py` method-for-method; a quick-xml `Writer` with 2-space indentation mirrors `arxml_writer.py`. Spec: `docs/superpowers/specs/2026-10-01-rust-armodel-p0-walking-skeleton-design.md`; coding rules: `docs/code_guide.md` (authoritative).

**Tech Stack:** Rust stable, edition 2021, quick-xml 0.42, id-arena 2.3, thiserror 2, getopts 0.2.

**Reference implementation:** py-armodel at https://github.com/melodypapa/py-armodel/ (a local checkout, if present, sits at `../py-armodel`; py file/method references below name its repo-relative paths).

---

## Ground rules for every task (from docs/code_guide.md)

- No `unwrap`/`expect`/`panic!` in library code (`src/` outside `#[cfg(test)]` and outside `src/bin/`). Panics are fine in tests. The CLI may `eprintln!` + `process::exit(1)`.
- `cargo fmt` and `cargo clippy -- -D warnings` must be clean before every commit.
- Private fields everywhere; access via `get_`/`set_` (setters return `&mut Self`).
- Every commit must leave `cargo build` and `cargo test` green.
- Unit tests live in the same file under `#[cfg(test)] mod tests { use super::*; ... }`.

---

### Task 1: Add thiserror dependency

**Files:**
- Modify: `Cargo.toml`

- [x] **Step 1: Add thiserror to `[dependencies]`**

Edit `Cargo.toml` so the dependency block reads exactly:

```toml
[dependencies]
getopts = "0.2"
quick-xml = "0.42.0"
id-arena = "2.3.0"
thiserror = "2"
```

(The spec §9 step 1 also asks to remove unused `regex`/`libxml` references and
dead stubs — those are already gone from `Cargo.toml`, and the module tree is
already scaffolded. Nothing else to do here.)

- [x] **Step 2: Verify the build is green**

Run: `cargo build`
Expected: `Finished` — no errors.

- [x] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "build: add thiserror dependency for P0 error model"
```

---

### Task 2: ARObject base struct

**Files:**
- Modify: `src/m2/autosar_templates/generic_structure/general_template_classes/ar_object.rs`

- [x] **Step 1: Replace the stub with ARObject and its test**

Replace the file's stub comment with (the `ElementRef` enum is added here in
Task 5, once `ar_package::ARPackage` exists — this task only needs `ARObject`):

```rust
//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ArObject
//!
//! Spec class `ARObject` — abstract base of all AUTOSAR meta-classes
//! (AUTOSAR_FO_TPS_GenericStructureTemplate, Table 6.1).
//! `ElementRef` (the heterogeneous-link enum) is added in Task 5.

/// spec class `ARObject`
#[derive(Debug, Default)]
pub struct ARObject {
    checksum: Option<String>,
    timestamp: Option<String>,
}

impl ARObject {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }

    pub fn set_checksum(&mut self, value: impl Into<String>) -> &mut Self {
        self.checksum = Some(value.into());
        self
    }

    pub fn get_timestamp(&self) -> Option<&str> {
        self.timestamp.as_deref()
    }

    pub fn set_timestamp(&mut self, value: impl Into<String>) -> &mut Self {
        self.timestamp = Some(value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ar_object_accessors() {
        let mut ar_object = ARObject::new();
        assert_eq!(ar_object.get_checksum(), None);
        assert_eq!(ar_object.get_timestamp(), None);

        ar_object
            .set_checksum("abc")
            .set_timestamp("2026-10-01T12:00:00+01:00");

        assert_eq!(ar_object.get_checksum(), Some("abc"));
        assert_eq!(ar_object.get_timestamp(), Some("2026-10-01T12:00:00+01:00"));
    }
}
```

- [x] **Step 2: Run the test**

Run: `cargo test --lib ar_object`
Expected: PASS (1 test).

- [x] **Step 3: Commit**

```bash
git add src/m2/autosar_templates/generic_structure/general_template_classes/ar_object.rs
git commit -m "feat(m2): ARObject base struct with checksum/timestamp accessors"
```

---

### Task 3: Referrable and MultilanguageReferrable

**Files:**
- Modify: `src/m2/autosar_templates/generic_structure/general_template_classes/identifiable.rs`

- [x] **Step 1: Replace the stub with the two structs and tests**

```rust
//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::Identifiable
//!
//! Spec classes `Referrable`, `MultilanguageReferrable`, `Identifiable`
//! (P0 design §5), modelled as composition per `docs/code_guide.md` §3.

use super::ar_object::ARObject;

/// spec class `Referrable` — `Referrable : ARObject`.
///
/// The `parent: Option<ElementRef>` field is added in Task 5 together with
/// `ElementRef` (it needs `ar_package::ARPackage` to exist first).
#[derive(Debug, Default)]
pub struct Referrable {
    base: ARObject,
    short_name: Option<String>,
}

impl Referrable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.short_name = Some(value.into());
        self
    }
}

/// spec class `MultilanguageReferrable` — `MultilanguageReferrable : Referrable`.
/// No new fields in P0; `long_name` and friends arrive with the P1 converter.
#[derive(Debug, Default)]
pub struct MultilanguageReferrable {
    base: Referrable,
}

impl MultilanguageReferrable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &Referrable {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut Referrable {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.base.get_short_name()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.set_short_name(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn referrable_accessors() {
        let mut referrable = Referrable::new();
        assert_eq!(referrable.get_short_name(), None);

        referrable.set_short_name("Pkg");
        assert_eq!(referrable.get_short_name(), Some("Pkg"));
        assert_eq!(referrable.base().get_checksum(), None);
    }

    #[test]
    fn multilanguage_referrable_forwards_short_name() {
        let mut referrable = MultilanguageReferrable::new();
        referrable.set_short_name("X");
        assert_eq!(referrable.get_short_name(), Some("X"));
        assert_eq!(referrable.base().get_short_name(), Some("X"));
    }
}
```

- [x] **Step 2: Run the tests**

Run: `cargo test --lib identifiable`
Expected: PASS (2 tests).

- [x] **Step 3: Commit**

```bash
git add src/m2/autosar_templates/generic_structure/general_template_classes/identifiable.rs
git commit -m "feat(m2): Referrable and MultilanguageReferrable composition structs"
```

---

### Task 4: Identifiable

**Files:**
- Modify: `src/m2/autosar_templates/generic_structure/general_template_classes/identifiable.rs`

- [x] **Step 1: Add imports, the Identifiable struct, and a test**

Add to the file's import block at the top:

```rust
use crate::m2::msr::asam_hdo::admin_data::AdminData;
use id_arena::Id;
```

Append below `MultilanguageReferrable` (before `#[cfg(test)]`):

```rust
/// spec class `Identifiable` — `Identifiable : MultilanguageReferrable`.
///
/// The remaining spec fields (desc, introduction, annotations, long_name)
/// arrive in P1 from the converter (P0 design §5).
#[derive(Debug, Default)]
pub struct Identifiable {
    base: MultilanguageReferrable,
    uuid: Option<String>,
    category: Option<String>,
    admin_data: Option<Id<AdminData>>,
}

impl Identifiable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &MultilanguageReferrable {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut MultilanguageReferrable {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.base.get_short_name()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.set_short_name(value);
        self
    }

    pub fn get_uuid(&self) -> Option<&str> {
        self.uuid.as_deref()
    }

    pub fn set_uuid(&mut self, value: impl Into<String>) -> &mut Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn get_category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    pub fn set_category(&mut self, value: impl Into<String>) -> &mut Self {
        self.category = Some(value.into());
        self
    }

    pub fn get_admin_data(&self) -> Option<Id<AdminData>> {
        self.admin_data
    }

    pub fn set_admin_data(&mut self, admin_data: Id<AdminData>) -> &mut Self {
        self.admin_data = Some(admin_data);
        self
    }
}
```

`AdminData` does not exist yet — create its Task-5 stub in this same step so
the crate compiles (Task 7 fills in the content fields). New file content for
`src/m2/msr/asam_hdo/admin_data.rs`:

```rust
//! spec: M2::MSR::AsamHdo::AdminData
//!
//! Spec classes `AdminData`, `DocRevision`, `Modification` (P0 design §5).
//! The content fields (doc_revisions, language, sdgs, used_languages) and
//! `DocRevision` are added in Task 7.

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;

/// spec class `AdminData`
#[derive(Debug, Default)]
pub struct AdminData {
    base: ARObject,
}

impl AdminData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }
}
```

Add to `identifiable.rs`'s `mod tests`:

```rust
    #[test]
    fn identifiable_accessors() {
        let mut identifiable = Identifiable::new();
        assert_eq!(identifiable.get_uuid(), None);
        assert_eq!(identifiable.get_category(), None);
        assert!(identifiable.get_admin_data().is_none());

        let arena: id_arena::Arena<AdminData> = id_arena::Arena::new();
        let admin_data_id = arena.alloc(AdminData::new());

        identifiable
            .set_uuid("uuid-1")
            .set_category("STD")
            .set_short_name("Ident")
            .set_admin_data(admin_data_id);

        assert_eq!(identifiable.get_uuid(), Some("uuid-1"));
        assert_eq!(identifiable.get_category(), Some("STD"));
        assert_eq!(identifiable.get_short_name(), Some("Ident"));
        assert_eq!(identifiable.get_admin_data(), Some(admin_data_id));
    }
```

- [x] **Step 2: Run the tests**

Run: `cargo test --lib identifiable`
Expected: PASS (3 tests).

- [x] **Step 3: Commit**

```bash
git add src/m2/autosar_templates/generic_structure/general_template_classes/identifiable.rs src/m2/msr/asam_hdo/admin_data.rs
git commit -m "feat(m2): Identifiable with uuid/category/admin_data accessors"
```

---

### Task 5: Base-chain completion — CollectableElement, PackageableElement, ARElement, ARPackage, ReferenceBase, ElementRef

**Files:**
- Modify: `src/m2/autosar_templates/generic_structure/general_template_classes/ar_object.rs`
- Modify: `src/m2/autosar_templates/generic_structure/general_template_classes/identifiable.rs`
- Create: `src/m2/autosar_templates/generic_structure/general_template_classes/element_collection.rs` (fill the stub)
- Create: `src/m2/autosar_templates/generic_structure/general_template_classes/ar_package.rs` (fill the stub)

- [x] **Step 1: Add `ElementRef` and the `parent` field**

Append to `ar_object.rs` (after the `ARObject` impl, before `#[cfg(test)]`):

```rust
use id_arena::Id;

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackage;

/// Type-erased handle for heterogeneous links (`Referrable::parent`,
/// `ARPackage::elements`) — replaces py's untyped `parent: ARObject`.
///
/// P0 hand-writes the single variant `AdminDataWhitespace.arxml` can produce;
/// the P1 converter generates the full variant list.
#[derive(Debug, Clone, Copy)]
pub enum ElementRef {
    ARPackage(Id<ARPackage>),
}
```

(Rust allows the `use` items inside the file body; if you prefer, move both
`use` lines to the top of the file.)

In `identifiable.rs`, change the import and the `Referrable` struct:

```rust
use super::ar_object::{ARObject, ElementRef};
```

```rust
#[derive(Debug, Default)]
pub struct Referrable {
    base: ARObject,
    parent: Option<ElementRef>,
    short_name: Option<String>,
}
```

and add to `impl Referrable`:

```rust
    pub fn get_parent(&self) -> Option<ElementRef> {
        self.parent
    }

    pub fn set_parent(&mut self, parent: Option<ElementRef>) -> &mut Self {
        self.parent = parent;
        self
    }
```

Extend the `referrable_accessors` test with:

```rust
        // parent is set through Document::add_ar_package (Task 8).
        assert!(referrable.get_parent().is_none());
```

- [x] **Step 2: Fill element_collection.rs**

```rust
//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ElementCollection
//!
//! Spec classes `CollectableElement`, `Collection` (P0 design §5).

use super::identifiable::Identifiable;

/// spec class `CollectableElement` — `CollectableElement : Identifiable`.
/// No new fields in P0.
#[derive(Debug, Default)]
pub struct CollectableElement {
    base: Identifiable,
}

impl CollectableElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &Identifiable {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut Identifiable {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.base.get_short_name()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.set_short_name(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collectable_element_forwards_short_name() {
        let mut element = CollectableElement::new();
        element.set_short_name("Coll");
        assert_eq!(element.get_short_name(), Some("Coll"));
    }
}
```

- [x] **Step 3: Fill ar_package.rs**

```rust
//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ARPackage
//!
//! Spec classes `PackageableElement`, `ARElement`, `ARPackage`,
//! `ReferenceBase` (P0 design §5). Note the spec chain:
//! `ARPackage : CollectableElement` (py-armodel mirrors this — ARPackage does
//! NOT derive from ARElement).
//!
//! Children live in `Document`-owned arenas and are referenced by id —
//! never `Vec<ARPackage>` by value (`docs/code_guide.md` §4).

use id_arena::Id;

use super::ar_object::ElementRef;
use super::element_collection::CollectableElement;
use crate::m2::msr::asam_hdo::admin_data::AdminData;

/// spec class `PackageableElement` — `PackageableElement : CollectableElement`.
/// No new fields in P0.
#[derive(Debug, Default)]
pub struct PackageableElement {
    base: CollectableElement,
}

impl PackageableElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &CollectableElement {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut CollectableElement {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.base.get_short_name()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.set_short_name(value);
        self
    }
}

/// spec class `ARElement` — `ARElement : PackageableElement`.
/// No new fields in P0.
#[derive(Debug, Default)]
pub struct ARElement {
    base: PackageableElement,
}

impl ARElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &PackageableElement {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut PackageableElement {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.base.get_short_name()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.set_short_name(value);
        self
    }
}

/// spec class `ARPackage` — `ARPackage : CollectableElement`.
#[derive(Debug, Default)]
pub struct ARPackage {
    base: CollectableElement,
    elements: Vec<ElementRef>,
    ar_packages: Vec<Id<ARPackage>>,
    reference_bases: Vec<Id<ReferenceBase>>,
}

impl ARPackage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &CollectableElement {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut CollectableElement {
        &mut self.base
    }

    // — forwarded accessors (the py inheritance surface) —
    //
    // Chain: ARPackage → CollectableElement → Identifiable →
    // MultilanguageReferrable → Referrable → ARObject.

    pub fn get_checksum(&self) -> Option<&str> {
        self.base.base().base().base().base().get_checksum()
    }

    pub fn get_timestamp(&self) -> Option<&str> {
        self.base.base().base().base().base().get_timestamp()
    }

    pub fn get_parent(&self) -> Option<ElementRef> {
        self.base.base().base().base().get_parent()
    }

    pub fn set_parent(&mut self, parent: Option<ElementRef>) -> &mut Self {
        self.base.base_mut().base_mut().base_mut().base_mut().set_parent(parent);
        self
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.base.get_short_name()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.set_short_name(value);
        self
    }

    pub fn get_uuid(&self) -> Option<&str> {
        self.base.base().get_uuid()
    }

    pub fn set_uuid(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.base_mut().set_uuid(value);
        self
    }

    pub fn get_category(&self) -> Option<&str> {
        self.base.base().get_category()
    }

    pub fn set_category(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.base_mut().set_category(value);
        self
    }

    pub fn get_admin_data(&self) -> Option<Id<AdminData>> {
        self.base.base().get_admin_data()
    }

    pub fn set_admin_data(&mut self, admin_data: Id<AdminData>) -> &mut Self {
        self.base.base_mut().set_admin_data(admin_data);
        self
    }

    // — own fields —

    pub fn get_elements(&self) -> &[ElementRef] {
        &self.elements
    }

    pub(crate) fn push_element(&mut self, element: ElementRef) {
        self.elements.push(element);
    }

    pub fn get_ar_packages(&self) -> &[Id<ARPackage>] {
        &self.ar_packages
    }

    pub(crate) fn push_ar_package(&mut self, ar_package: Id<ARPackage>) {
        self.ar_packages.push(ar_package);
    }

    pub fn get_reference_bases(&self) -> &[Id<ReferenceBase>] {
        &self.reference_bases
    }
}

/// spec class `ReferenceBase` — `ReferenceBase : ARElement`.
/// P0 placeholder: `AdminDataWhitespace.arxml` has none; fields arrive in P1.
#[derive(Debug, Default)]
pub struct ReferenceBase {
    base: ARElement,
}

impl ReferenceBase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARElement {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARElement {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use id_arena::Arena;

    #[test]
    fn ar_package_accessors() {
        let mut package = ARPackage::new();
        package
            .set_short_name("Pkg")
            .set_uuid("u-1")
            .set_category("STD");

        assert_eq!(package.get_short_name(), Some("Pkg"));
        assert_eq!(package.get_uuid(), Some("u-1"));
        assert_eq!(package.get_category(), Some("STD"));
        assert_eq!(package.get_checksum(), None);
        assert_eq!(package.get_timestamp(), None);
        assert!(package.get_parent().is_none());
        assert!(package.get_elements().is_empty());
        assert!(package.get_ar_packages().is_empty());
        assert!(package.get_reference_bases().is_empty());
    }

    #[test]
    fn ar_package_links_use_ids() {
        let arena = Arena::new();
        let parent_id = arena.alloc(ARPackage::new());

        let mut child = ARPackage::new();
        child.set_parent(Some(ElementRef::ARPackage(parent_id)));

        assert!(matches!(child.get_parent(), Some(ElementRef::ARPackage(id)) if id == parent_id));
    }
}
```

- [x] **Step 4: Run all tests**

Run: `cargo test`
Expected: PASS (all tests so far).

- [x] **Step 5: Format, lint, commit**

```bash
cargo fmt
cargo clippy -- -D warnings
git add src/m2
git commit -m "feat(m2): complete base chain with ARPackage, ReferenceBase, ElementRef"
```

---

### Task 6: XmlSpace, LPlainText, MultiLanguagePlainText, Sd, Sdf

**Files:**
- Modify: `src/m2/msr/documentation/text_model/language_data_model.rs`
- Modify: `src/m2/msr/documentation/text_model/multilanguage_data.rs`
- Modify: `src/m2/msr/asam_hdo/special_data.rs`

- [x] **Step 1: Fill language_data_model.rs**

```rust
//! spec: M2::MSR::Documentation::TextModel::LanguageDataModel
//!
//! Spec classes `LPlainText`, `WhitespaceControlled`, `LanguageSpecific`
//! (P0 design §5). The spec marks `LPlainText` as an interface; per the locked
//! decision (P0 design §3) it is a concrete struct.

use std::fmt;

/// `xml:space` enumeration. Whitespace-fidelity carrier: text elements
/// (`SD`, `L-10`) keep this as an explicit field (`docs/code_guide.md` §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSpace {
    Preserve,
    Default,
}

impl TryFrom<&str> for XmlSpace {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "preserve" => Ok(XmlSpace::Preserve),
            "default" => Ok(XmlSpace::Default),
            _ => Err(()),
        }
    }
}

impl fmt::Display for XmlSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmlSpace::Preserve => write!(f, "preserve"),
            XmlSpace::Default => write!(f, "default"),
        }
    }
}

/// spec class `LPlainText` — the `<L-10 L="EN" xml:space="preserve">…</L-10>`
/// element. `l` comes from `LanguageSpecific`, `xml_space` from
/// `WhitespaceControlled`, `value` from the mixed-content base.
#[derive(Debug, Default)]
pub struct LPlainText {
    l: Option<String>,
    xml_space: Option<XmlSpace>,
    value: Option<String>,
}

impl LPlainText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_l(&self) -> Option<&str> {
        self.l.as_deref()
    }

    pub fn set_l(&mut self, value: impl Into<String>) -> &mut Self {
        self.l = Some(value.into());
        self
    }

    pub fn get_xml_space(&self) -> Option<XmlSpace> {
        self.xml_space
    }

    pub fn set_xml_space(&mut self, value: XmlSpace) -> &mut Self {
        self.xml_space = Some(value);
        self
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn set_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.value = Some(value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l_plain_text_accessors() {
        let mut l10 = LPlainText::new();
        assert_eq!(l10.get_l(), None);
        assert_eq!(l10.get_xml_space(), None);
        assert_eq!(l10.get_value(), None);

        l10.set_l("EN")
            .set_xml_space(XmlSpace::Preserve)
            .set_value("English");

        assert_eq!(l10.get_l(), Some("EN"));
        assert_eq!(l10.get_xml_space(), Some(XmlSpace::Preserve));
        assert_eq!(l10.get_value(), Some("English"));
    }

    #[test]
    fn xml_space_parses_and_formats() {
        assert_eq!(XmlSpace::try_from("preserve"), Ok(XmlSpace::Preserve));
        assert_eq!(XmlSpace::try_from("default"), Ok(XmlSpace::Default));
        assert!(XmlSpace::try_from("bogus").is_err());
        assert_eq!(XmlSpace::Preserve.to_string(), "preserve");
        assert_eq!(XmlSpace::Default.to_string(), "default");
    }
}
```

- [x] **Step 2: Fill multilanguage_data.rs**

```rust
//! spec: M2::MSR::Documentation::TextModel::MultilanguageData
//!
//! Spec classes `MultiLanguagePlainText`, `MultiLanguageOverviewParagraph`
//! (P0 design §5).

use id_arena::Id;

use super::language_data_model::LPlainText;

/// spec class `MultiLanguagePlainText` — the `<USED-LANGUAGES>` content.
/// P0 note: no `ARObject` base (spec §5 field list), so S/T attributes on
/// `<USED-LANGUAGES>` are not carried; the P1 converter restores them.
#[derive(Debug, Default)]
pub struct MultiLanguagePlainText {
    l10s: Vec<Id<LPlainText>>,
}

impl MultiLanguagePlainText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_l10s(&self) -> &[Id<LPlainText>] {
        &self.l10s
    }

    /// py `addL10`
    pub fn push_l10(&mut self, l10: Id<LPlainText>) {
        self.l10s.push(l10);
    }
}

/// spec class `MultiLanguageOverviewParagraph`.
/// P0 placeholder (referenced by `SdgCaption.desc`); fields arrive in P1.
#[derive(Debug, Default)]
pub struct MultiLanguageOverviewParagraph;

#[cfg(test)]
mod tests {
    use super::*;
    use id_arena::Arena;

    #[test]
    fn multi_language_plain_text_collects_l10s() {
        let mut paragraph = MultiLanguagePlainText::new();

        let mut arena: Arena<LPlainText> = Arena::new();
        let mut l10 = LPlainText::new();
        l10.set_l("EN");
        let id = arena.alloc(l10);

        paragraph.push_l10(id);
        assert_eq!(paragraph.get_l10s(), &[id]);
    }
}
```

- [x] **Step 3: Fill special_data.rs (Sd, Sdf — Sdg trio follows in Task 7)**

```rust
//! spec: M2::MSR::AsamHdo::SpecialData
//!
//! Spec classes `Sd`, `Sdf`, `Sdg`, `SdgCaption`, `SdgContents` (P0 design §5).
//! `Sd`/`Sdf` carry explicit `xml_space`/`value` fields — this is how
//! whitespace fidelity is preserved in the round trip.
//! `Sdg`, `SdgCaption` and `SdgContents` are added in Task 7.

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::msr::documentation::text_model::language_data_model::XmlSpace;

/// spec class `Sd` — `<SD GID="…" xml:space="preserve">text</SD>`.
#[derive(Debug, Default)]
pub struct Sd {
    base: ARObject,
    gid: Option<String>,
    value: Option<String>,
    xml_space: Option<XmlSpace>,
}

impl Sd {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }

    pub fn get_gid(&self) -> Option<&str> {
        self.gid.as_deref()
    }

    pub fn set_gid(&mut self, value: impl Into<String>) -> &mut Self {
        self.gid = Some(value.into());
        self
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn set_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.value = Some(value.into());
        self
    }

    pub fn get_xml_space(&self) -> Option<XmlSpace> {
        self.xml_space
    }

    pub fn set_xml_space(&mut self, value: XmlSpace) -> &mut Self {
        self.xml_space = Some(value);
        self
    }
}

/// spec class `Sdf`
#[derive(Debug, Default)]
pub struct Sdf {
    base: ARObject,
    gid: Option<String>,
    value: Option<String>,
}

impl Sdf {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }

    pub fn get_gid(&self) -> Option<&str> {
        self.gid.as_deref()
    }

    pub fn set_gid(&mut self, value: impl Into<String>) -> &mut Self {
        self.gid = Some(value.into());
        self
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn set_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.value = Some(value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sd_accessors() {
        let mut sd = Sd::new();
        assert_eq!(sd.get_gid(), None);
        sd.set_gid("purpose")
            .set_value("special   data")
            .set_xml_space(XmlSpace::Preserve);
        assert_eq!(sd.get_gid(), Some("purpose"));
        assert_eq!(sd.get_value(), Some("special   data"));
        assert_eq!(sd.get_xml_space(), Some(XmlSpace::Preserve));
    }

    #[test]
    fn sdf_accessors() {
        let mut sdf = Sdf::new();
        sdf.set_gid("g").set_value("42");
        assert_eq!(sdf.get_gid(), Some("g"));
        assert_eq!(sdf.get_value(), Some("42"));
    }
}
```

- [x] **Step 4: Run all tests, format, lint, commit**

```bash
cargo test
cargo fmt
cargo clippy -- -D warnings
git add src/m2
git commit -m "feat(m2): XmlSpace, LPlainText, MultiLanguagePlainText, Sd, Sdf"
```

Expected: all tests PASS.

---

### Task 7: SdgCaption, SdgContents, Sdg, AdminData completion, DocRevision

**Files:**
- Modify: `src/m2/msr/asam_hdo/special_data.rs`
- Modify: `src/m2/msr/asam_hdo/admin_data.rs`

- [x] **Step 1: Append SdgCaption, SdgContents, Sdg to special_data.rs**

Update the import block at the top of the file to:

```rust
use id_arena::Id;

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::autosar_templates::generic_structure::general_template_classes::identifiable::Referrable;
use crate::m2::msr::documentation::text_model::language_data_model::XmlSpace;
use crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguageOverviewParagraph;
```

Append below `Sdf` (before `#[cfg(test)]`):

```rust
/// spec class `SdgCaption` — `SdgCaption : MultilanguageReferrable` (P0 §5).
#[derive(Debug, Default)]
pub struct SdgCaption {
    base: Referrable,
    desc: Option<Id<MultiLanguageOverviewParagraph>>,
}

impl SdgCaption {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &Referrable {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut Referrable {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.base.get_short_name()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.set_short_name(value);
        self
    }

    pub fn get_desc(&self) -> Option<Id<MultiLanguageOverviewParagraph>> {
        self.desc
    }

    pub fn set_desc(&mut self, desc: Id<MultiLanguageOverviewParagraph>) -> &mut Self {
        self.desc = Some(desc);
        self
    }
}

/// spec class `SdgContents`. The spec marks it as an interface; modelled as a
/// concrete struct per §3.
#[derive(Debug, Default)]
pub struct SdgContents {
    sd: Vec<Id<Sd>>,
    sdf: Vec<Id<Sdf>>,
    sdg: Vec<Id<Sdg>>,
}

impl SdgContents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_sd(&self) -> &[Id<Sd>] {
        &self.sd
    }

    /// py `addSd`
    pub fn push_sd(&mut self, sd: Id<Sd>) {
        self.sd.push(sd);
    }

    pub fn get_sdf(&self) -> &[Id<Sdf>] {
        &self.sdf
    }

    /// py `addSdf`
    pub fn push_sdf(&mut self, sdf: Id<Sdf>) {
        self.sdf.push(sdf);
    }

    pub fn get_sdg(&self) -> &[Id<Sdg>] {
        &self.sdg
    }

    /// py `addSdg`
    pub fn push_sdg(&mut self, sdg: Id<Sdg>) {
        self.sdg.push(sdg);
    }

    /// py `getSdg`'s emptiness check: contents attach to an SDG only when
    /// non-empty.
    pub fn is_empty(&self) -> bool {
        self.sd.is_empty() && self.sdf.is_empty() && self.sdg.is_empty()
    }
}

/// spec class `Sdg`
#[derive(Debug, Default)]
pub struct Sdg {
    base: ARObject,
    gid: Option<String>,
    sdg_caption: Option<Id<SdgCaption>>,
    sdg_contents_type: Option<Id<SdgContents>>,
}

impl Sdg {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }

    pub fn get_gid(&self) -> Option<&str> {
        self.gid.as_deref()
    }

    pub fn set_gid(&mut self, value: impl Into<String>) -> &mut Self {
        self.gid = Some(value.into());
        self
    }

    pub fn get_sdg_caption(&self) -> Option<Id<SdgCaption>> {
        self.sdg_caption
    }

    pub fn set_sdg_caption(&mut self, caption: Id<SdgCaption>) -> &mut Self {
        self.sdg_caption = Some(caption);
        self
    }

    pub fn get_sdg_contents_type(&self) -> Option<Id<SdgContents>> {
        self.sdg_contents_type
    }

    pub fn set_sdg_contents_type(&mut self, contents: Id<SdgContents>) -> &mut Self {
        self.sdg_contents_type = Some(contents);
        self
    }
}
```

Extend the test module (with `use id_arena::Arena;` alongside `use super::*;`):

```rust
    #[test]
    fn sdg_with_contents_and_caption() {
        let mut sdg = Sdg::new();
        sdg.set_gid("demo");
        assert_eq!(sdg.get_gid(), Some("demo"));

        let mut caption = SdgCaption::new();
        caption.set_short_name("Caption");
        assert_eq!(caption.get_short_name(), Some("Caption"));
        assert!(caption.get_desc().is_none());

        let mut contents = SdgContents::new();
        assert!(contents.is_empty());

        let mut arena: Arena<Sd> = Arena::new();
        let mut sd = Sd::new();
        sd.set_gid("purpose");
        let sd_id = arena.alloc(sd);

        contents.push_sd(sd_id);
        assert!(!contents.is_empty());
        assert_eq!(contents.get_sd(), &[sd_id]);
    }
```

(The SDG-to-contents id wiring is covered by the parser tests in Task 10 and
the Document tests in Task 8 — at this task's level the structs are exercised
value-by-value.)

- [x] **Step 2: Complete admin_data.rs (content fields + DocRevision)**

Replace the Task-4 stub body with:

```rust
//! spec: M2::MSR::AsamHdo::AdminData
//!
//! Spec classes `AdminData`, `DocRevision`, `Modification` (P0 design §5).

use id_arena::Id;

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::msr::asam_hdo::special_data::Sdg;
use crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguagePlainText;

/// spec class `DocRevision`.
/// P0 placeholder: `AdminDataWhitespace.arxml` has none; fields arrive in P1
/// from the converter (P0 design §5).
#[derive(Debug, Default)]
pub struct DocRevision;

/// spec class `AdminData` — `AdminData : ARObject`.
#[derive(Debug, Default)]
pub struct AdminData {
    base: ARObject,
    doc_revisions: Vec<Id<DocRevision>>,
    language: Option<String>,
    sdgs: Vec<Id<Sdg>>,
    used_languages: Option<Id<MultiLanguagePlainText>>,
}

impl AdminData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }

    pub fn get_doc_revisions(&self) -> &[Id<DocRevision>] {
        &self.doc_revisions
    }

    /// py `addDocRevision`
    pub fn push_doc_revision(&mut self, revision: Id<DocRevision>) {
        self.doc_revisions.push(revision);
    }

    pub fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub fn set_language(&mut self, value: impl Into<String>) -> &mut Self {
        self.language = Some(value.into());
        self
    }

    pub fn get_sdgs(&self) -> &[Id<Sdg>] {
        &self.sdgs
    }

    /// py `addSdg`
    pub fn push_sdg(&mut self, sdg: Id<Sdg>) {
        self.sdgs.push(sdg);
    }

    /// py `getUsedLanguages` — a **single** `MultiLanguagePlainText`, not a list.
    pub fn get_used_languages(&self) -> Option<Id<MultiLanguagePlainText>> {
        self.used_languages
    }

    /// py `setUsedLanguages`
    pub fn set_used_languages(&mut self, used_languages: Id<MultiLanguagePlainText>) -> &mut Self {
        self.used_languages = Some(used_languages);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use id_arena::Arena;

    #[test]
    fn admin_data_accessors() {
        let mut admin_data = AdminData::new();
        assert_eq!(admin_data.get_language(), None);
        assert!(admin_data.get_sdgs().is_empty());
        assert!(admin_data.get_doc_revisions().is_empty());
        assert!(admin_data.get_used_languages().is_none());

        let mut arena: Arena<MultiLanguagePlainText> = Arena::new();
        let mlpt_id = arena.alloc(MultiLanguagePlainText::new());

        admin_data.set_language("EN").set_used_languages(mlpt_id);

        assert_eq!(admin_data.get_language(), Some("EN"));
        assert_eq!(admin_data.get_used_languages(), Some(mlpt_id));
    }
}
```

- [x] **Step 3: Run all tests, format, lint, commit**

```bash
cargo test
cargo fmt
cargo clippy -- -D warnings
git add src/m2
git commit -m "feat(m2): Sdg/SdgCaption/SdgContents, AdminData fields, DocRevision placeholder"
```

Expected: all tests PASS.

---

### Task 8: Document — arenas, factories, resolvers, structural equality

**Files:**
- Modify: `src/m2/autosar_templates/autosar_top_level_structure.rs`
- Modify: `src/lib.rs`

- [x] **Step 1: Implement Document**

Replace the stub comment with:

```rust
//! spec: M2::AUTOSARTemplates::AutosarTopLevelStructure
//!
//! Home of the spec class `AUTOSAR`. The Rust root type is `Document`
//! (P0 design §4): it owns one arena per concrete class, and all
//! cross-type mutation goes through it (`docs/code_guide.md` §4).

use id_arena::{Arena, Id};

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::{ARObject, ElementRef};
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::{ARPackage, ReferenceBase};
use crate::m2::msr::asam_hdo::admin_data::{AdminData, DocRevision};
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdf, Sdg, SdgCaption, SdgContents};
use crate::m2::msr::documentation::text_model::language_data_model::LPlainText;
use crate::m2::msr::documentation::text_model::multilanguage_data::{
    MultiLanguageOverviewParagraph, MultiLanguagePlainText,
};

/// Root type, in place of py-armodel's `AUTOSAR`.
///
/// The arena fields are `pub(crate)`: the parser and writer resolve ids
/// through them inside the crate; code outside the crate mutates only via the
/// factory methods and reads via the accessors/resolvers.
#[derive(Debug, Default)]
pub struct Document {
    // — arenas (one per concrete class) —
    pub(crate) ar_packages: Arena<ARPackage>,
    pub(crate) reference_bases: Arena<ReferenceBase>,
    pub(crate) admin_datas: Arena<AdminData>,
    pub(crate) doc_revisions: Arena<DocRevision>,
    pub(crate) multi_language_plain_texts: Arena<MultiLanguagePlainText>,
    pub(crate) multi_language_overview_paragraphs: Arena<MultiLanguageOverviewParagraph>,
    pub(crate) l_plain_texts: Arena<LPlainText>,
    pub(crate) sds: Arena<Sd>,
    pub(crate) sdfs: Arena<Sdf>,
    pub(crate) sdg_captions: Arena<SdgCaption>,
    pub(crate) sdg_contents: Arena<SdgContents>,
    pub(crate) sdgs: Arena<Sdg>,

    // — root fields (spec class `AUTOSAR`) —
    admin_data: Option<Id<AdminData>>,
    root_ar_packages: Vec<Id<ARPackage>>,
    schema_location: String,
    ar_release: String,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    // — root field accessors (py getAdminData/setAdminData/getARPackages) —

    pub fn get_admin_data(&self) -> Option<&AdminData> {
        self.admin_data.and_then(|id| self.admin_datas.get(id))
    }

    pub fn set_admin_data(&mut self, admin_data: Id<AdminData>) -> &mut Self {
        self.admin_data = Some(admin_data);
        self
    }

    pub fn get_ar_packages(&self) -> &[Id<ARPackage>] {
        &self.root_ar_packages
    }

    pub fn get_schema_location(&self) -> &str {
        &self.schema_location
    }

    pub fn set_schema_location(&mut self, value: impl Into<String>) -> &mut Self {
        self.schema_location = value.into();
        self
    }

    pub fn get_ar_release(&self) -> &str {
        &self.ar_release
    }

    pub fn set_ar_release(&mut self, value: impl Into<String>) -> &mut Self {
        self.ar_release = value.into();
        self
    }

    // — factories (py createARPackage / addElement) —

    /// Creates an `ARPackage` in the arena and links it: when `parent` is
    /// `Some`, the child's `parent` handle is set and the id is pushed into
    /// the parent's package list; when `None`, it becomes a root package.
    pub fn add_ar_package(&mut self, parent: Option<Id<ARPackage>>, short_name: &str) -> Id<ARPackage> {
        let mut package = ARPackage::new();
        package.set_short_name(short_name);
        if let Some(parent_id) = parent {
            package.set_parent(Some(ElementRef::ARPackage(parent_id)));
        }
        let id = self.ar_packages.alloc(package);
        match parent {
            Some(parent_id) => {
                if let Some(parent) = self.ar_packages.get_mut(parent_id) {
                    parent.push_ar_package(id);
                }
            }
            None => self.root_ar_packages.push(id),
        }
        id
    }

    /// py `addElement` — appends a heterogeneous element to a package.
    pub fn add_element(&mut self, package: Id<ARPackage>, element: ElementRef) {
        if let Some(package) = self.ar_packages.get_mut(package) {
            package.push_element(element);
        }
    }

    // — id resolvers (read access from outside the crate) —

    pub fn get_ar_package(&self, id: Id<ARPackage>) -> Option<&ARPackage> {
        self.ar_packages.get(id)
    }

    pub fn get_sdg(&self, id: Id<Sdg>) -> Option<&Sdg> {
        self.sdgs.get(id)
    }

    pub fn get_sdg_contents(&self, id: Id<SdgContents>) -> Option<&SdgContents> {
        self.sdg_contents.get(id)
    }

    pub fn get_sd(&self, id: Id<Sd>) -> Option<&Sd> {
        self.sds.get(id)
    }

    pub fn get_multi_language_plain_text(&self, id: Id<MultiLanguagePlainText>) -> Option<&MultiLanguagePlainText> {
        self.multi_language_plain_texts.get(id)
    }

    pub fn get_l_plain_text(&self, id: Id<LPlainText>) -> Option<&LPlainText> {
        self.l_plain_texts.get(id)
    }

    // — structural equality (P0 design §7) —

    /// Structural equality: walk both models, resolving `Id<T>` links within
    /// each `Document`. Mirrors py's `assert_models_equal`:
    /// exact type + field-by-field comparison, ignoring `parent`.
    pub fn assert_structurally_equal(&self, other: &Document) -> Result<(), String> {
        if self.schema_location != other.schema_location {
            return Err(format!(
                "schema_location mismatch: `{}` vs `{}`",
                self.schema_location, other.schema_location
            ));
        }
        if self.ar_release != other.ar_release {
            return Err(format!(
                "ar_release mismatch: `{}` vs `{}`",
                self.ar_release, other.ar_release
            ));
        }

        match (self.get_admin_data(), other.get_admin_data()) {
            (None, None) => {}
            (Some(admin_a), Some(admin_b)) => {
                self.compare_admin_data(other, admin_a, admin_b, "ADMIN-DATA")?;
            }
            _ => {
                return Err("admin_data mismatch: ADMIN-DATA present in one document only".to_string());
            }
        }

        if self.root_ar_packages.len() != other.root_ar_packages.len() {
            return Err(format!(
                "root_ar_packages length mismatch: {} vs {}",
                self.root_ar_packages.len(),
                other.root_ar_packages.len()
            ));
        }
        for (index, (a, b)) in self
            .root_ar_packages
            .iter()
            .zip(other.root_ar_packages.iter())
            .enumerate()
        {
            let package_a = self
                .ar_packages
                .get(*a)
                .ok_or_else(|| format!("root_ar_packages[{index}]: id not found in own arena"))?;
            let package_b = other
                .ar_packages
                .get(*b)
                .ok_or_else(|| format!("root_ar_packages[{index}]: id not found in other arena"))?;
            self.compare_ar_package(other, package_a, package_b, &format!("AR-PACKAGE[{index}]"))?;
        }
        Ok(())
    }

    fn compare_ar_object(a: &ARObject, b: &ARObject, path: &str) -> Result<(), String> {
        if a.get_checksum() != b.get_checksum() {
            return Err(format!("{path}: checksum mismatch"));
        }
        if a.get_timestamp() != b.get_timestamp() {
            return Err(format!("{path}: timestamp mismatch"));
        }
        Ok(())
    }

    fn compare_ar_package(&self, other: &Document, a: &ARPackage, b: &ARPackage, path: &str) -> Result<(), String> {
        // Chain to ARObject: ARPackage → CollectableElement → Identifiable →
        // MultilanguageReferrable → Referrable → ARObject.
        Self::compare_ar_object(
            a.base().base().base().base().base(),
            b.base().base().base().base().base(),
            path,
        )?;
        if a.get_short_name() != b.get_short_name() {
            return Err(format!("{path}: SHORT-NAME mismatch"));
        }
        if a.get_uuid() != b.get_uuid() {
            return Err(format!("{path}: UUID mismatch"));
        }
        if a.get_category() != b.get_category() {
            return Err(format!("{path}: CATEGORY mismatch"));
        }
        // `parent` is ignored, mirroring py's ignored_attrs = ["parent"]

        match (a.get_admin_data(), b.get_admin_data()) {
            (None, None) => {}
            (Some(admin_a), Some(admin_b)) => {
                let admin_a = self
                    .admin_datas
                    .get(admin_a)
                    .ok_or_else(|| format!("{path}: admin_data id not found in own arena"))?;
                let admin_b = other
                    .admin_datas
                    .get(admin_b)
                    .ok_or_else(|| format!("{path}: admin_data id not found in other arena"))?;
                self.compare_admin_data(other, admin_a, admin_b, &format!("{path}.ADMIN-DATA"))?;
            }
            _ => return Err(format!("{path}: admin_data present in one package only")),
        }

        let sub_packages_a = a.get_ar_packages();
        let sub_packages_b = b.get_ar_packages();
        if sub_packages_a.len() != sub_packages_b.len() {
            return Err(format!("{path}: nested AR-PACKAGES length mismatch"));
        }
        for (index, (sub_a, sub_b)) in sub_packages_a.iter().zip(sub_packages_b.iter()).enumerate() {
            let package_a = self
                .ar_packages
                .get(*sub_a)
                .ok_or_else(|| format!("{path}.AR-PACKAGES[{index}]: id not found in own arena"))?;
            let package_b = other
                .ar_packages
                .get(*sub_b)
                .ok_or_else(|| format!("{path}.AR-PACKAGES[{index}]: id not found in other arena"))?;
            self.compare_ar_package(other, package_a, package_b, &format!("{path}.AR-PACKAGES[{index}]"))?;
        }

        let elements_a = a.get_elements();
        let elements_b = b.get_elements();
        if elements_a.len() != elements_b.len() {
            return Err(format!("{path}: ELEMENTS length mismatch"));
        }
        for (index, (element_a, element_b)) in elements_a.iter().zip(elements_b.iter()).enumerate() {
            match (element_a, element_b) {
                (ElementRef::ARPackage(package_a), ElementRef::ARPackage(package_b)) => {
                    let package_a = self
                        .ar_packages
                        .get(*package_a)
                        .ok_or_else(|| format!("{path}.ELEMENTS[{index}]: id not found in own arena"))?;
                    let package_b = other
                        .ar_packages
                        .get(*package_b)
                        .ok_or_else(|| format!("{path}.ELEMENTS[{index}]: id not found in other arena"))?;
                    self.compare_ar_package(
                        other,
                        package_a,
                        package_b,
                        &format!("{path}.ELEMENTS[{index}]"),
                    )?;
                }
                _ => return Err(format!("{path}.ELEMENTS[{index}]: element kind mismatch")),
            }
        }

        // ReferenceBase is a P0 placeholder (no fields); list length is the
        // whole comparison until P1 fills it in.
        if a.get_reference_bases().len() != b.get_reference_bases().len() {
            return Err(format!("{path}: REFERENCE-BASES length mismatch"));
        }
        Ok(())
    }

    fn compare_admin_data(&self, other: &Document, a: &AdminData, b: &AdminData, path: &str) -> Result<(), String> {
        Self::compare_ar_object(a.base(), b.base(), path)?;
        if a.get_language() != b.get_language() {
            return Err(format!("{path}: LANGUAGE mismatch"));
        }

        match (a.get_used_languages(), b.get_used_languages()) {
            (None, None) => {}
            (Some(mlpt_a), Some(mlpt_b)) => {
                let mlpt_a = self
                    .multi_language_plain_texts
                    .get(mlpt_a)
                    .ok_or_else(|| format!("{path}: USED-LANGUAGES id not found in own arena"))?;
                let mlpt_b = other
                    .multi_language_plain_texts
                    .get(mlpt_b)
                    .ok_or_else(|| format!("{path}: USED-LANGUAGES id not found in other arena"))?;
                self.compare_multi_language_plain_text(
                    other,
                    mlpt_a,
                    mlpt_b,
                    &format!("{path}.USED-LANGUAGES"),
                )?;
            }
            _ => return Err(format!("{path}: USED-LANGUAGES present in one document only")),
        }

        let sdgs_a = a.get_sdgs();
        let sdgs_b = b.get_sdgs();
        if sdgs_a.len() != sdgs_b.len() {
            return Err(format!("{path}: SDGS length mismatch"));
        }
        for (index, (sdg_a, sdg_b)) in sdgs_a.iter().zip(sdgs_b.iter()).enumerate() {
            let sdg_a = self
                .sdgs
                .get(*sdg_a)
                .ok_or_else(|| format!("{path}.SDGS[{index}]: id not found in own arena"))?;
            let sdg_b = other
                .sdgs
                .get(*sdg_b)
                .ok_or_else(|| format!("{path}.SDGS[{index}]: id not found in other arena"))?;
            self.compare_sdg(other, sdg_a, sdg_b, &format!("{path}.SDGS[{index}]"))?;
        }

        // DocRevision is a P0 placeholder (no fields); list length is the
        // whole comparison until P1 fills it in.
        if a.get_doc_revisions().len() != b.get_doc_revisions().len() {
            return Err(format!("{path}: DOC-REVISIONS length mismatch"));
        }
        Ok(())
    }

    fn compare_multi_language_plain_text(
        &self,
        other: &Document,
        a: &MultiLanguagePlainText,
        b: &MultiLanguagePlainText,
        path: &str,
    ) -> Result<(), String> {
        let l10s_a = a.get_l10s();
        let l10s_b = b.get_l10s();
        if l10s_a.len() != l10s_b.len() {
            return Err(format!("{path}: L-10 length mismatch"));
        }
        for (index, (l10_a, l10_b)) in l10s_a.iter().zip(l10s_b.iter()).enumerate() {
            let l10_a = self
                .l_plain_texts
                .get(*l10_a)
                .ok_or_else(|| format!("{path}.L-10[{index}]: id not found in own arena"))?;
            let l10_b = other
                .l_plain_texts
                .get(*l10_b)
                .ok_or_else(|| format!("{path}.L-10[{index}]: id not found in other arena"))?;
            let l10_path = format!("{path}.L-10[{index}]");
            Self::compare_ar_object(l10_a.base(), l10_b.base(), &l10_path)?;
            if l10_a.get_l() != l10_b.get_l() {
                return Err(format!("{l10_path}: L attribute mismatch"));
            }
            if l10_a.get_xml_space() != l10_b.get_xml_space() {
                return Err(format!("{l10_path}: xml:space mismatch"));
            }
            if l10_a.get_value() != l10_b.get_value() {
                return Err(format!("{l10_path}: text mismatch"));
            }
        }
        Ok(())
    }

    fn compare_sdg(&self, other: &Document, a: &Sdg, b: &Sdg, path: &str) -> Result<(), String> {
        Self::compare_ar_object(a.base(), b.base(), path)?;
        if a.get_gid() != b.get_gid() {
            return Err(format!("{path}: GID mismatch"));
        }

        match (a.get_sdg_caption(), b.get_sdg_caption()) {
            (None, None) => {}
            (Some(caption_a), Some(caption_b)) => {
                let caption_a = self
                    .sdg_captions
                    .get(caption_a)
                    .ok_or_else(|| format!("{path}: SDG-CAPTION id not found in own arena"))?;
                let caption_b = other
                    .sdg_captions
                    .get(caption_b)
                    .ok_or_else(|| format!("{path}: SDG-CAPTION id not found in other arena"))?;
                let caption_path = format!("{path}.SDG-CAPTION");
                // SdgCaption base chain: Referrable → ARObject.
                Self::compare_ar_object(caption_a.base().base(), caption_b.base().base(), &caption_path)?;
                if caption_a.get_short_name() != caption_b.get_short_name() {
                    return Err(format!("{caption_path}: SHORT-NAME mismatch"));
                }
                // desc (MultiLanguageOverviewParagraph) is a P0 placeholder on
                // both sides; presence equality is the whole comparison.
                if caption_a.get_desc().is_some() != caption_b.get_desc().is_some() {
                    return Err(format!("{caption_path}: DESC present in one side only"));
                }
            }
            _ => return Err(format!("{path}: SDG-CAPTION present in one side only")),
        }

        match (a.get_sdg_contents_type(), b.get_sdg_contents_type()) {
            (None, None) => {}
            (Some(contents_a), Some(contents_b)) => {
                let contents_a = self
                    .sdg_contents
                    .get(contents_a)
                    .ok_or_else(|| format!("{path}: SDG contents id not found in own arena"))?;
                let contents_b = other
                    .sdg_contents
                    .get(contents_b)
                    .ok_or_else(|| format!("{path}: SDG contents id not found in other arena"))?;
                self.compare_sdg_contents(other, contents_a, contents_b, &format!("{path}.contents"))?;
            }
            _ => return Err(format!("{path}: SDG contents present in one side only")),
        }
        Ok(())
    }

    fn compare_sdg_contents(&self, other: &Document, a: &SdgContents, b: &SdgContents, path: &str) -> Result<(), String> {
        let sds_a = a.get_sd();
        let sds_b = b.get_sd();
        if sds_a.len() != sds_b.len() {
            return Err(format!("{path}: SD length mismatch"));
        }
        for (index, (sd_a, sd_b)) in sds_a.iter().zip(sds_b.iter()).enumerate() {
            let sd_a = self
                .sds
                .get(*sd_a)
                .ok_or_else(|| format!("{path}.SD[{index}]: id not found in own arena"))?;
            let sd_b = other
                .sds
                .get(*sd_b)
                .ok_or_else(|| format!("{path}.SD[{index}]: id not found in other arena"))?;
            let sd_path = format!("{path}.SD[{index}]");
            Self::compare_ar_object(sd_a.base(), sd_b.base(), &sd_path)?;
            if sd_a.get_gid() != sd_b.get_gid() {
                return Err(format!("{sd_path}: GID mismatch"));
            }
            if sd_a.get_xml_space() != sd_b.get_xml_space() {
                return Err(format!("{sd_path}: xml:space mismatch"));
            }
            if sd_a.get_value() != sd_b.get_value() {
                return Err(format!("{sd_path}: text mismatch"));
            }
        }

        let sdfs_a = a.get_sdf();
        let sdfs_b = b.get_sdf();
        if sdfs_a.len() != sdfs_b.len() {
            return Err(format!("{path}: SDF length mismatch"));
        }
        for (index, (sdf_a, sdf_b)) in sdfs_a.iter().zip(sdfs_b.iter()).enumerate() {
            let sdf_a = self
                .sdfs
                .get(*sdf_a)
                .ok_or_else(|| format!("{path}.SDF[{index}]: id not found in own arena"))?;
            let sdf_b = other
                .sdfs
                .get(*sdf_b)
                .ok_or_else(|| format!("{path}.SDF[{index}]: id not found in other arena"))?;
            let sdf_path = format!("{path}.SDF[{index}]");
            Self::compare_ar_object(sdf_a.base(), sdf_b.base(), &sdf_path)?;
            if sdf_a.get_gid() != sdf_b.get_gid() {
                return Err(format!("{sdf_path}: GID mismatch"));
            }
            if sdf_a.get_value() != sdf_b.get_value() {
                return Err(format!("{sdf_path}: text mismatch"));
            }
        }

        let sdgs_a = a.get_sdg();
        let sdgs_b = b.get_sdg();
        if sdgs_a.len() != sdgs_b.len() {
            return Err(format!("{path}: nested SDG length mismatch"));
        }
        for (index, (sdg_a, sdg_b)) in sdgs_a.iter().zip(sdgs_b.iter()).enumerate() {
            let sdg_a = self
                .sdgs
                .get(*sdg_a)
                .ok_or_else(|| format!("{path}.SDG[{index}]: id not found in own arena"))?;
            let sdg_b = other
                .sdgs
                .get(*sdg_b)
                .ok_or_else(|| format!("{path}.SDG[{index}]: id not found in other arena"))?;
            self.compare_sdg(other, sdg_a, sdg_b, &format!("{path}.SDG[{index}]"))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ar_package_links_parent_and_root() {
        let mut document = Document::new();
        let root = document.add_ar_package(None, "Root");
        let child = document.add_ar_package(Some(root), "Child");

        assert_eq!(document.get_ar_packages().len(), 1);

        let root_package = document.get_ar_package(root).unwrap();
        assert_eq!(root_package.get_short_name(), Some("Root"));
        assert_eq!(root_package.get_ar_packages(), &[child]);
        assert!(root_package.get_parent().is_none());

        let child_package = document.get_ar_package(child).unwrap();
        assert_eq!(child_package.get_short_name(), Some("Child"));
        assert!(matches!(child_package.get_parent(), Some(ElementRef::ARPackage(parent)) if parent == root));
    }

    #[test]
    fn add_element_registers_heterogeneous_child() {
        let mut document = Document::new();
        let parent = document.add_ar_package(None, "Parent");
        let child = document.add_ar_package(None, "Child");

        document.add_element(parent, ElementRef::ARPackage(child));

        assert_eq!(
            document.get_ar_package(parent).unwrap().get_elements(),
            &[ElementRef::ARPackage(child)]
        );
    }

    #[test]
    fn equal_documents_compare_equal() {
        let mut document_a = Document::new();
        document_a.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd");
        document_a.set_ar_release("R21-11");
        document_a.add_ar_package(None, "WhitespaceDemo");

        let mut document_b = Document::new();
        document_b.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd");
        document_b.set_ar_release("R21-11");
        document_b.add_ar_package(None, "WhitespaceDemo");

        document_a.assert_structurally_equal(&document_b).unwrap();
    }

    #[test]
    fn mismatched_short_names_are_reported() {
        let mut document_a = Document::new();
        document_a.add_ar_package(None, "A");

        let mut document_b = Document::new();
        document_b.add_ar_package(None, "B");

        let error = document_a.assert_structurally_equal(&document_b).unwrap_err();
        assert!(error.contains("SHORT-NAME mismatch"), "unexpected error: {error}");
    }
}
```

- [x] **Step 2: Re-export the public surface in lib.rs**

`src/lib.rs` becomes:

```rust
pub mod m2;
pub mod parser;
pub mod writer;

pub use m2::autosar_templates::autosar_top_level_structure::Document;
```

- [x] **Step 3: Run all tests**

Run: `cargo test`
Expected: PASS (all tasks' tests).

- [x] **Step 4: Format, lint, commit**

```bash
cargo fmt
cargo clippy -- -D warnings
git add src/m2/autosar_templates/autosar_top_level_structure.rs src/lib.rs
git commit -m "feat(m2): Document with arenas, factories, resolvers, structural equality"
```

---

### Task 9: Node DOM + abstract parser helpers

**Files:**
- Modify: `src/parser/abstract_arxml_parser.rs`

- [x] **Step 1: Implement Node, ParseError, the DOM builder, and find helpers**

Replace the stub comment with:

```rust
//! Node DOM + `find`/`find_all`/`get_child_element_string` helpers
//! (P0 design §6). Mirrors py's `abstract_arxml_parser.py`.

use std::collections::BTreeMap;
use std::io::BufRead;
use std::path::Path;

use quick_xml::escape::unescape;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use thiserror::Error;

/// Error model mirroring py's raise/notImplemented split
/// (`docs/code_guide.md` §7).
#[derive(Debug, Error)]
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

Continue with the node, whitespace rule, builder, and helpers:

```rust
/// A minimal DOM node. Attribute keys are stored as they appear in the
/// document (`GID`, `xml:space`, `xsi:schemaLocation` — prefix kept, matching
/// the wire format); element names are namespace-stripped local names,
/// matching py's `getPureTagName`.
#[derive(Debug, Default, Clone)]
pub struct Node {
    pub name: String,
    pub attrs: BTreeMap<String, String>,
    pub children: Vec<Node>,
    pub text: Option<String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Whitespace rule (P0 design §6): text is captured verbatim when the element
/// carries `xml:space="preserve"`, dropped when whitespace-only, and trimmed
/// otherwise. This is required for the `SD` value `"special   data"`.
fn apply_whitespace_rule(node: &mut Node) {
    if node.attrs.get("xml:space").map(String::as_str) == Some("preserve") {
        return;
    }
    if let Some(text) = node.text.as_mut() {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            node.text = None;
        } else {
            *text = trimmed.to_string();
        }
    }
}

fn start_to_node(start: &BytesStart<'_>) -> Result<Node, ParseError> {
    let mut node = Node::new(start.local_name().as_ref());
    for attr in start.attributes() {
        let attr = attr?;
        let value = attr.unescape_value()?.into_owned();
        node.attrs.insert(attr.key.into_inner().to_string(), value);
    }
    Ok(node)
}

fn unescape_text(raw: &str) -> Result<String, ParseError> {
    unescape(raw).map(|value| value.into_owned()).map_err(|error| ParseError::InvalidElement {
        element: String::new(),
        reason: format!("cannot unescape text: {error}"),
    })
}

/// Builds the `Node` DOM from a quick-xml reader.
pub fn build_dom_from_reader<R: BufRead>(reader: Reader<R>) -> Result<Node, ParseError> {
    let mut reader = reader;
    reader.config_mut().trim_text(false);

    let mut stack: Vec<Node> = Vec::new();
    let mut root: Option<Node> = None;
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf)? {
            Event::Start(start) => {
                stack.push(start_to_node(&start)?);
            }
            Event::Empty(start) => {
                let mut node = start_to_node(&start)?;
                apply_whitespace_rule(&mut node);
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                }
            }
            Event::Text(text) => {
                if let Some(current) = stack.last_mut() {
                    let raw = unescape_text(text.as_ref())?;
                    current.text.get_or_insert_with(String::new).push_str(&raw);
                }
            }
            Event::CData(cdata) => {
                if let Some(current) = stack.last_mut() {
                    current
                        .text
                        .get_or_insert_with(String::new)
                        .push_str(cdata.as_ref());
                }
            }
            Event::GeneralRef(reference) => {
                if let Some(current) = stack.last_mut() {
                    // quick-xml splits entity references out of text; resolve
                    // the predefined/numeric form via `unescape`.
                    let raw = format!("&{};", reference.as_ref());
                    let resolved = unescape_text(&raw)?;
                    current.text.get_or_insert_with(String::new).push_str(&resolved);
                }
            }
            Event::End(_) => {
                let mut node = stack.pop().ok_or_else(|| ParseError::InvalidElement {
                    element: String::new(),
                    reason: "unbalanced end tag".to_string(),
                })?;
                apply_whitespace_rule(&mut node);
                match stack.last_mut() {
                    Some(parent) => parent.children.push(node),
                    None => {
                        if root.is_some() {
                            return Err(ParseError::InvalidElement {
                                element: node.name,
                                reason: "multiple root elements".to_string(),
                            });
                        }
                        root = Some(node);
                    }
                }
            }
            Event::Eof => break,
            _ => {} // Decl, Comment, PI, DocType — ignored
        }
    }

    if let Some(unclosed) = stack.pop() {
        return Err(ParseError::InvalidElement {
            element: unclosed.name,
            reason: "unclosed element(s) at end of document".to_string(),
        });
    }

    root.ok_or_else(|| ParseError::InvalidElement {
        element: String::new(),
        reason: "document contains no elements".to_string(),
    })
}

/// Builds the `Node` DOM from a file.
pub fn build_dom(path: &Path) -> Result<Node, ParseError> {
    build_dom_from_reader(Reader::from_file(path)?)
}

/// py `AbstractARXMLParser.findall` — supports the `A/B`, `A/*` and `./A`
/// key forms; `*` matches any child.
pub fn find_all<'a>(parent: &'a Node, key: &str) -> Vec<&'a Node> {
    let mut current: Vec<&Node> = vec![parent];
    for segment in key.split('/') {
        let mut next: Vec<&Node> = Vec::new();
        for node in current {
            if segment == "." {
                next.push(node);
            } else if segment == "*" {
                next.extend(node.children.iter());
            } else {
                next.extend(node.children.iter().filter(|child| child.name == segment));
            }
        }
        current = next;
    }
    current
}

/// py `AbstractARXMLParser.find` — first match of `find_all`.
pub fn find<'a>(parent: &'a Node, key: &str) -> Option<&'a Node> {
    find_all(parent, key).into_iter().next()
}

/// py `getChildElementOptionalStringValue`
pub fn get_child_element_string<'a>(parent: &'a Node, key: &str) -> Option<&'a str> {
    find(parent, key).and_then(|node| node.text.as_deref())
}

/// py `getShortName` — required by the spec; missing SHORT-NAME is an error.
pub fn get_short_name(element: &Node) -> Result<String, ParseError> {
    match get_child_element_string(element, "SHORT-NAME") {
        Some(name) => Ok(name.to_string()),
        None => Err(ParseError::InvalidElement {
            element: element.name.clone(),
            reason: "SHORT-NAME is required".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0">
  <SHORT-NAME>Demo</SHORT-NAME>
  <SD GID="purpose" xml:space="preserve">special   data</SD>
  <PLAIN>  padded  </PLAIN>
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>P1</SHORT-NAME>
    </AR-PACKAGE>
    <AR-PACKAGE>
      <SHORT-NAME>P2</SHORT-NAME>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    fn sample_dom() -> Node {
        build_dom_from_reader(Reader::from_str(SAMPLE)).unwrap()
    }

    #[test]
    fn dom_strips_namespaces_and_keeps_attributes() {
        let root = sample_dom();
        assert_eq!(root.name, "AUTOSAR");
        assert_eq!(
            root.attrs.get("xmlns").map(String::as_str),
            Some("http://autosar.org/schema/r4.0")
        );
    }

    #[test]
    fn whitespace_rule_is_applied_per_element() {
        let root = sample_dom();
        let sd = find(&root, "SD").unwrap();
        assert_eq!(sd.text.as_deref(), Some("special   data"));
        assert_eq!(
            sd.attrs.get("xml:space").map(String::as_str),
            Some("preserve")
        );

        let plain = find(&root, "PLAIN").unwrap();
        assert_eq!(plain.text.as_deref(), Some("padded"));

        let short_name = find(&root, "SHORT-NAME").unwrap();
        assert_eq!(short_name.text.as_deref(), Some("Demo"));
    }

    #[test]
    fn find_and_find_all_support_paths() {
        let root = sample_dom();
        let first = find(&root, "AR-PACKAGES/AR-PACKAGE/SHORT-NAME").unwrap();
        assert_eq!(first.text.as_deref(), Some("P1"));

        let packages = find_all(&root, "AR-PACKAGES/*");
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[0].name, "AR-PACKAGE");

        assert!(find(&root, "MISSING").is_none());
    }

    #[test]
    fn get_short_name_requires_element() {
        let root = sample_dom();
        assert_eq!(get_short_name(&root).unwrap(), "Demo");

        let error = get_short_name(find(&root, "PLAIN").unwrap()).unwrap_err();
        assert!(error.to_string().contains("SHORT-NAME is required"));
    }
}
```

- [x] **Step 2: Run the tests**

Run: `cargo test --lib abstract_arxml_parser`
Expected: PASS (4 tests).

- [x] **Step 3: Format, lint, commit**

```bash
cargo fmt
cargo clippy -- -D warnings
git add src/parser/abstract_arxml_parser.rs
git commit -m "feat(parser): Node DOM with xml:space-aware whitespace rule and find helpers"
```

---

### Task 10: ARXMLParser — load, read_admin_data, read_ar_packages

**Files:**
- Modify: `src/parser/arxml_parser.rs`
- Modify: `src/lib.rs`

- [x] **Step 1: Implement the parser**

Replace the stub comment with:

```rust
//! `ARXMLParser::load` with release detection, `read_admin_data`,
//! `read_ar_packages` (P0 design §6). Mirrors py's `arxml_parser.py`
//! method-for-method so the P2–P4 port stays reviewable.

use std::io::BufRead;
use std::path::Path;

use id_arena::Id;
use quick_xml::reader::Reader;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackage;
use crate::m2::msr::asam_hdo::admin_data::AdminData;
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdf, Sdg, SdgCaption, SdgContents};
use crate::m2::msr::documentation::text_model::language_data_model::{LPlainText, XmlSpace};
use crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguagePlainText;
use crate::parser::abstract_arxml_parser::{
    Node, ParseError, build_dom_from_reader, find, find_all, get_child_element_string, get_short_name,
};

/// py `ARXMLParser(options)` — `warning: true` (the default) collects warnings
/// and continues; `warning: false` fails on the first problem.
#[derive(Debug, Clone)]
pub struct ParserOptions {
    pub warning: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self { warning: true }
    }
}

/// py's default options (`warning: true`).
pub fn default_options() -> ParserOptions {
    ParserOptions::default()
}

/// py `ARXMLParser`
#[derive(Debug)]
pub struct ARXMLParser {
    options: ParserOptions,
    warnings: Vec<String>,
}

impl Default for ARXMLParser {
    fn default() -> Self {
        Self::new(ParserOptions::default())
    }
}

impl ARXMLParser {
    pub fn new(options: ParserOptions) -> Self {
        Self {
            options,
            warnings: Vec::new(),
        }
    }

    /// Warnings collected so far (only populated in `warning: true` mode).
    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }

    /// py `raiseError` — log-and-continue in warning mode, hard error otherwise.
    fn raise_error(&mut self, message: String) -> Result<(), ParseError> {
        if self.options.warning {
            self.warnings.push(message);
            Ok(())
        } else {
            Err(ParseError::InvalidElement {
                element: "ARXML".to_string(),
                reason: message,
            })
        }
    }

    /// py `notImplemented`
    fn not_implemented(&mut self, message: String) -> Result<(), ParseError> {
        self.raise_error(message)
    }

    /// py `load`
    pub fn load(&mut self, path: &Path, document: &mut Document) -> Result<(), ParseError> {
        self.load_from_reader(Reader::from_file(path)?, document)
    }

    fn load_from_reader<R: BufRead>(
        &mut self,
        reader: Reader<R>,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let root = build_dom_from_reader(reader)?;
        if root.name != "AUTOSAR" {
            return Err(ParseError::UnexpectedRoot(root.name.clone()));
        }

        // py detectNamespace + getAUTOSARInfo: P0 records the schema
        // location; the XSD file name maps to a release via py's
        // `xsd_to_version_mapping` (conftest.py), defaulting to R23-11.
        if let Some(location) = root.attrs.get("xsi:schemaLocation") {
            document.set_schema_location(location.clone());
        }
        let xsd = document
            .get_schema_location()
            .split_whitespace()
            .nth(1)
            .unwrap_or("");
        document.set_ar_release(xsd_to_version(xsd));

        // py load: document.setAdminData(self.getAdminData(root, "ADMIN-DATA"))
        if let Some(admin_data_node) = find(&root, "ADMIN-DATA") {
            let admin_data = self.read_admin_data(admin_data_node, document)?;
            document.set_admin_data(admin_data);
        }
        // FILE-INFO-COMMENT and INTRODUCTION arrive in P1 (absent in this file).

        // py load: self.readARPackages(root, document)
        self.read_ar_packages(&root, None, document)?;
        Ok(())
    }

    /// py `readARObject` — reads the `S` (checksum) and `T` (timestamp)
    /// attributes.
    fn read_ar_object(&mut self, element: &Node, ar_object: &mut ARObject) {
        if let Some(checksum) = element.attrs.get("S") {
            ar_object.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = element.attrs.get("T") {
            ar_object.set_timestamp(timestamp.as_str());
        }
    }

    /// py `readWhitespaceControlled` — reads the element's own `xml:space`.
    fn read_xml_space(&mut self, element: &Node) -> Option<XmlSpace> {
        match element.attrs.get("xml:space").map(String::as_str) {
            Some(value) => match XmlSpace::try_from(value) {
                Ok(xml_space) => Some(xml_space),
                Err(_) => {
                    let message =
                        format!("Unsupported xml:space value <{value}> on <{}>", element.name);
                    self.not_implemented(message)?;
                    None
                }
            },
            None => None,
        }
    }

    /// py `getAdminData` — returns the allocated `AdminData` id.
    fn read_admin_data(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Id<AdminData>, ParseError> {
        let mut admin_data = AdminData::new();
        self.read_ar_object(element, admin_data.base_mut());

        if let Some(language) = get_child_element_string(element, "LANGUAGE") {
            admin_data.set_language(language);
        }

        // py getMultiLanguagePlainText — USED-LANGUAGES is a single
        // MultiLanguagePlainText collecting the L-10 entries. (py's
        // readARObject on the USED-LANGUAGES element is dropped in P0:
        // MultiLanguagePlainText has no ARObject base here.)
        if let Some(used_languages_node) = find(element, "USED-LANGUAGES") {
            let mut paragraph = MultiLanguagePlainText::new();
            for l10_node in find_all(used_languages_node, "L-10") {
                let mut l10 = LPlainText::new();
                self.read_ar_object(l10_node, l10.base_mut());
                if let Some(l) = l10_node.attrs.get("L") {
                    l10.set_l(l.as_str());
                }
                if let Some(xml_space) = self.read_xml_space(l10_node) {
                    l10.set_xml_space(xml_space);
                }
                // py readLanguageSpecific: setValue(element.text) — the text
                // itself; the DOM already applied the whitespace rule.
                if let Some(value) = &l10_node.text {
                    l10.set_value(value.as_str());
                }
                let l10_id = document.l_plain_texts.alloc(l10);
                paragraph.push_l10(l10_id);
            }
            let paragraph_id = document.multi_language_plain_texts.alloc(paragraph);
            admin_data.set_used_languages(paragraph_id);
        }

        // py readAdminDataSdgs
        if let Some(sdgs_node) = find(element, "SDGS") {
            for child in find_all(sdgs_node, "*") {
                if child.name == "SDG" {
                    let sdg_id = self.read_sdg(child, document)?;
                    admin_data.push_sdg(sdg_id);
                } else {
                    self.not_implemented(format!("Unsupported SDG <{}>", child.name))?;
                }
            }
        }

        // py readAdminDataDocRevisions — DocRevision is a P0 placeholder;
        // flag DOC-REVISIONS content instead of silently dropping it.
        if let Some(doc_revisions_node) = find(element, "DOC-REVISIONS") {
            if !find_all(doc_revisions_node, "*").is_empty() {
                self.not_implemented("DOC-REVISIONS are not supported in P0".to_string())?;
            }
        }

        Ok(document.admin_datas.alloc(admin_data))
    }

    /// py `getSdg` — reads one `SDG` element (recursively) and returns its id.
    fn read_sdg(&mut self, element: &Node, document: &mut Document) -> Result<Id<Sdg>, ParseError> {
        let mut sdg = Sdg::new();
        self.read_ar_object(element, sdg.base_mut());
        if let Some(gid) = element.attrs.get("GID") {
            sdg.set_gid(gid.as_str());
        }

        // py readSdgCaption — SDG-CAPTION with a required SHORT-NAME.
        if let Some(caption_node) = find(element, "SDG-CAPTION") {
            let mut caption = SdgCaption::new();
            self.read_ar_object(caption_node, caption.base_mut().base_mut());
            let caption_short_name = get_short_name(caption_node)?;
            caption.set_short_name(caption_short_name);
            if find(caption_node, "DESC").is_some() {
                self.not_implemented("SDG-CAPTION/DESC is not supported in P0".to_string())?;
            }
            let caption_id = document.sdg_captions.alloc(caption);
            sdg.set_sdg_caption(caption_id);
        }

        // py readSd / readSdf / nested getSdg, collected into an SdgContents
        // that is attached only when non-empty.
        let mut contents = SdgContents::new();

        for sd_node in find_all(element, "SD") {
            let mut sd = Sd::new();
            self.read_ar_object(sd_node, sd.base_mut());
            if let Some(gid) = sd_node.attrs.get("GID") {
                sd.set_gid(gid.as_str());
            }
            if let Some(xml_space) = self.read_xml_space(sd_node) {
                sd.set_xml_space(xml_space);
            }
            if let Some(value) = &sd_node.text {
                sd.set_value(value.as_str());
            }
            let sd_id = document.sds.alloc(sd);
            contents.push_sd(sd_id);
        }

        for sdf_node in find_all(element, "SDF") {
            let mut sdf = Sdf::new();
            self.read_ar_object(sdf_node, sdf.base_mut());
            if let Some(gid) = sdf_node.attrs.get("GID") {
                sdf.set_gid(gid.as_str());
            }
            if let Some(value) = &sdf_node.text {
                sdf.set_value(value.as_str());
            }
            let sdf_id = document.sdfs.alloc(sdf);
            contents.push_sdf(sdf_id);
        }

        for sdg_node in find_all(element, "SDG") {
            let nested_id = self.read_sdg(sdg_node, document)?;
            contents.push_sdg(nested_id);
        }

        if !contents.is_empty() {
            let contents_id = document.sdg_contents.alloc(contents);
            sdg.set_sdg_contents_type(contents_id);
        }

        Ok(document.sdgs.alloc(sdg))
    }

    /// py `readARPackages` — recurses into nested AR-PACKAGES. `parent` is
    /// `None` at the document root.
    fn read_ar_packages(
        &mut self,
        element: &Node,
        parent: Option<Id<ARPackage>>,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        if let Some(packages_node) = find(element, "AR-PACKAGES") {
            for child in find_all(packages_node, "*") {
                if child.name == "AR-PACKAGE" {
                    self.read_ar_package(child, parent, document)?;
                } else {
                    self.not_implemented(format!("Unsupported ARPackage <{}>", child.name))?;
                }
            }
        }
        Ok(())
    }

    /// py `readARPackage` — allocates the package first (so children can link
    /// to it), then fills its own fields in one scoped arena borrow, then
    /// recurses.
    fn read_ar_package(
        &mut self,
        element: &Node,
        parent: Option<Id<ARPackage>>,
        document: &mut Document,
    ) -> Result<Id<ARPackage>, ParseError> {
        let short_name = get_short_name(element)?;
        let id = document.add_ar_package(parent, &short_name);

        // Scalar pieces are read before the arena borrow so the fill below is
        // a single scoped block (`docs/code_guide.md` §4).
        let admin_data = match find(element, "ADMIN-DATA") {
            Some(admin_data_node) => Some(self.read_admin_data(admin_data_node, document)?),
            None => None,
        };
        let uuid = element.attrs.get("UUID").cloned();
        let category = get_child_element_string(element, "CATEGORY").map(str::to_string);

        if let Some(package) = document.ar_packages.get_mut(id) {
            if let Some(admin_data) = admin_data {
                package.set_admin_data(admin_data);
            }
            if let Some(uuid) = uuid {
                package.set_uuid(uuid);
            }
            if let Some(category) = category {
                package.set_category(category);
            }
        }

        // py readARPackageElements / readReferenceBases — no element kinds are
        // dispatched in P0; flag unexpected content instead of dropping it.
        if let Some(elements_node) = find(element, "ELEMENTS") {
            if !find_all(elements_node, "*").is_empty() {
                self.not_implemented("AR-PACKAGE/ELEMENTS are not supported in P0".to_string())?;
            }
        }
        if let Some(reference_bases_node) = find(element, "REFERENCE-BASES") {
            if !find_all(reference_bases_node, "*").is_empty() {
                self.not_implemented("REFERENCE-BASES are not supported in P0".to_string())?;
            }
        }

        self.read_ar_packages(element, Some(id), document)?;
        Ok(id)
    }
}

/// py `xsd_to_version_mapping` (tests/integration_tests/conftest.py) —
/// XSD file name to release, defaulting to R23-11 (P0 design §6).
fn xsd_to_version(xsd: &str) -> &'static str {
    match xsd {
        "autosar.xsd" => "3.2.3",
        "AUTOSAR_4-0-3.xsd" => "4.0.3",
        "AUTOSAR_4-1-0.xsd" => "4.1.0",
        "AUTOSAR_4-1-1.xsd" => "4.1.1",
        "AUTOSAR_4-1-2.xsd" => "4.1.2",
        "AUTOSAR_4-1-3.xsd" => "4.1.3",
        "AUTOSAR_4-2-1.xsd" => "4.2.1",
        "AUTOSAR_4-2-2.xsd" => "4.2.2",
        "AUTOSAR_00043.xsd" => "4.3.0",
        "AUTOSAR_00044.xsd" => "4.3.1",
        "AUTOSAR_00046.xsd" => "4.4.0",
        "AUTOSAR_00048.xsd" => "R19-11",
        "AUTOSAR_00049.xsd" => "R20-11",
        "AUTOSAR_00050.xsd" => "R21-11",
        "AUTOSAR_00051.xsd" => "R22-11",
        "AUTOSAR_00052.xsd" => "R23-11",
        "AUTOSAR_00053.xsd" => "R24-11",
        _ => "R23-11",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ElementRef;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Outer</SHORT-NAME>
      <AR-PACKAGES>
        <AR-PACKAGE>
          <SHORT-NAME>Inner</SHORT-NAME>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    fn parse_sample() -> Document {
        let mut document = Document::new();
        ARXMLParser::new(default_options())
            .load_from_reader(Reader::from_str(SAMPLE), &mut document)
            .unwrap();
        document
    }

    #[test]
    fn load_sets_schema_location_and_release() {
        let document = parse_sample();
        assert_eq!(
            document.get_schema_location(),
            "http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd"
        );
        assert_eq!(document.get_ar_release(), "R21-11");
    }

    #[test]
    fn load_rejects_wrong_root() {
        let mut document = Document::new();
        let result = ARXMLParser::new(default_options()).load_from_reader(
            Reader::from_str("<?xml version=\"1.0\"?><WRONG/>"),
            &mut document,
        );
        assert!(matches!(result, Err(ParseError::UnexpectedRoot(_))));
    }

    #[test]
    fn admin_data_round_trips_through_the_model() {
        let document = parse_sample();

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

    #[test]
    fn packages_nest_through_the_document_factory() {
        let document = parse_sample();
        assert_eq!(document.get_ar_packages().len(), 1);

        let outer_id = document.get_ar_packages()[0];
        let outer = document.get_ar_package(outer_id).unwrap();
        assert_eq!(outer.get_short_name(), Some("Outer"));
        assert!(outer.get_parent().is_none());

        let inner_id = outer.get_ar_packages()[0];
        let inner = document.get_ar_package(inner_id).unwrap();
        assert_eq!(inner.get_short_name(), Some("Inner"));
        assert!(matches!(inner.get_parent(), Some(ElementRef::ARPackage(parent)) if parent == outer_id));
    }
}
```

Update `src/lib.rs`:

```rust
pub mod m2;
pub mod parser;
pub mod writer;

pub use m2::autosar_templates::autosar_top_level_structure::Document;
pub use parser::abstract_arxml_parser::ParseError;
pub use parser::arxml_parser::{ARXMLParser, ParserOptions, default_options};
```

- [x] **Step 2: Run the tests**

Run: `cargo test --lib arxml_parser`
Expected: PASS (4 tests).

- [x] **Step 3: Format, lint, commit**

```bash
cargo fmt
cargo clippy -- -D warnings
git add src/parser/arxml_parser.rs src/lib.rs
git commit -m "feat(parser): ARXMLParser with release detection, admin data and package reading"
```

---

### Task 11: ARXMLWriter — save with namespaces, ADMIN-DATA, AR-PACKAGES

**Files:**
- Modify: `src/writer/abstract_arxml_writer.rs`
- Modify: `src/writer/arxml_writer.rs`
- Modify: `src/lib.rs`

- [x] **Step 1: WriteError + the shared text-element emitter**

Replace the stub comment in `abstract_arxml_writer.rs` with:

```rust
//! quick-xml `Writer` + element emitters shared by `arxml_writer.rs`
//! (P0 design §9 step 5). Mirrors py's `abstract_arxml_writer.py`.

use std::io::{self, Write};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use thiserror::Error;

/// Error model for the writer (mirrors `ParseError`).
#[derive(Debug, Error)]
pub enum WriteError {
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Emits `<name attrs>text</name>` on one line; when `text` is `None` the
/// element is still expanded (`<name></name>`, not self-closing) to match
/// py's `short_empty_elements=False`. The empty inline `Text` event is what
/// keeps the end tag on the same line under `new_with_indent`.
pub(crate) fn write_text_element<W: Write>(
    writer: &mut Writer<W>,
    name: &str,
    element: BytesStart<'_>,
    text: Option<&str>,
) -> Result<(), WriteError> {
    writer.write_event(Event::Start(element))?;
    match text {
        Some(value) => writer.write_event(Event::Text(BytesText::new(value)))?,
        None => writer.write_event(Event::Text(BytesText::from_escaped("")))?,
    }
    writer.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_element_expands_empty_elements_inline() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
        let element = BytesStart::new("LANGUAGE");
        write_text_element(&mut writer, "LANGUAGE", element, None).unwrap();
        writer.write_event(Event::Eof).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), "<LANGUAGE></LANGUAGE>");
    }

    #[test]
    fn text_element_escapes_and_writes_text() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
        let element = BytesStart::new("SHORT-NAME");
        write_text_element(&mut writer, "SHORT-NAME", element, Some("a<b")).unwrap();
        writer.write_event(Event::Eof).unwrap();
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "<SHORT-NAME>a&lt;b</SHORT-NAME>"
        );
    }
}
```

- [x] **Step 2: Implement ARXMLWriter::save in arxml_writer.rs**

Replace the stub comment with:

```rust
//! `ARXMLWriter::save` emitting the XML declaration, `AUTOSAR` with
//! namespaces, `ADMIN-DATA`, `AR-PACKAGES` (P0 design §9 step 5).
//! Mirrors py's `arxml_writer.py`.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use id_arena::Id;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::writer::Writer;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackage;
use crate::m2::msr::asam_hdo::admin_data::AdminData;
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdf, Sdg, SdgContents};
use crate::m2::msr::documentation::text_model::language_data_model::LPlainText;
use crate::writer::abstract_arxml_writer::{WriteError, write_text_element};

const DEFAULT_NAMESPACE: &str = "http://autosar.org/schema/r4.0";
const XSI_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";

/// py `ARXMLWriter`
#[derive(Debug, Default)]
pub struct ARXMLWriter;

impl ARXMLWriter {
    pub fn new() -> Self {
        Self
    }

    /// py `save` — 2-space indentation; element order follows py
    /// (`ADMIN-DATA`, then `AR-PACKAGES`; FILE-INFO-COMMENT/INTRODUCTION are
    /// P1).
    pub fn save(&self, path: &Path, document: &Document) -> Result<(), WriteError> {
        let file = BufWriter::new(File::create(path)?);
        let mut writer = Writer::new_with_indent(file, b' ', 2);

        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

        // py save(): namespace and schema location come from the document,
        // with the same fallback defaults.
        let schema_location = if document.get_schema_location().is_empty() {
            format!("{DEFAULT_NAMESPACE} AUTOSAR_4-0-3.xsd")
        } else {
            document.get_schema_location().to_string()
        };
        let namespace = schema_location.split(' ').next().unwrap_or(DEFAULT_NAMESPACE);

        let mut root = BytesStart::new("AUTOSAR");
        root.push_attribute(("xmlns", namespace));
        root.push_attribute(("xmlns:xsi", XSI_NAMESPACE));
        root.push_attribute(("xsi:schemaLocation", schema_location.as_str()));
        writer.write_event(Event::Start(root))?;

        if let Some(admin_data) = document.get_admin_data() {
            self.write_admin_data(&mut writer, admin_data, document)?;
        }

        self.write_ar_packages(&mut writer, document.get_ar_packages(), document)?;

        writer.write_event(Event::End(BytesEnd::new("AUTOSAR")))?;
        Ok(())
    }

    /// py `writeARObject` — the `S`/`T` attributes.
    fn write_ar_object_attributes(&self, element: &mut BytesStart<'_>, ar_object: &ARObject) {
        if let Some(checksum) = ar_object.get_checksum() {
            element.push_attribute(("S", checksum));
        }
        if let Some(timestamp) = ar_object.get_timestamp() {
            element.push_attribute(("T", timestamp));
        }
    }

    /// py `setAdminData`
    fn write_admin_data<W: Write>(
        &self,
        writer: &mut Writer<W>,
        admin_data: &AdminData,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("ADMIN-DATA");
        self.write_ar_object_attributes(&mut element, admin_data.base());
        writer.write_event(Event::Start(element))?;

        if let Some(language) = admin_data.get_language() {
            let language_element = BytesStart::new("LANGUAGE");
            write_text_element(writer, "LANGUAGE", language_element, Some(language))?;
        }

        // py setMultiLanguagePlainText
        if let Some(paragraph) = admin_data
            .get_used_languages()
            .and_then(|id| document.multi_language_plain_texts.get(id))
        {
            writer.write_event(Event::Start(BytesStart::new("USED-LANGUAGES")))?;
            for l10_id in paragraph.get_l10s() {
                if let Some(l10) = document.l_plain_texts.get(*l10_id) {
                    let mut l10_element = BytesStart::new("L-10");
                    if let Some(l) = l10.get_l() {
                        l10_element.push_attribute(("L", l));
                    }
                    if let Some(xml_space) = l10.get_xml_space() {
                        let xml_space = xml_space.to_string();
                        l10_element.push_attribute(("xml:space", xml_space.as_str()));
                    }
                    write_text_element(writer, "L-10", l10_element, l10.get_value())?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("USED-LANGUAGES")))?;
        }

        // py writeAdminDataSdgs
        let sdgs = admin_data.get_sdgs();
        if !sdgs.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("SDGS")))?;
            for sdg_id in sdgs {
                if let Some(sdg) = document.sdgs.get(*sdg_id) {
                    self.write_sdg(writer, sdg, document)?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("SDGS")))?;
        }

        // py writeAdminDataDocRevisions — DocRevision is a P0 placeholder and
        // `doc_revisions` is always empty here; emitted from P1.

        writer.write_event(Event::End(BytesEnd::new("ADMIN-DATA")))?;
        Ok(())
    }

    /// py `setSdg`
    fn write_sdg<W: Write>(
        &self,
        writer: &mut Writer<W>,
        sdg: &Sdg,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("SDG");
        self.write_ar_object_attributes(&mut element, sdg.base());
        if let Some(gid) = sdg.get_gid() {
            element.push_attribute(("GID", gid));
        }
        writer.write_event(Event::Start(element))?;

        // py writeSdgCaption
        if let Some(caption) = sdg
            .get_sdg_caption()
            .and_then(|id| document.sdg_captions.get(id))
        {
            writer.write_event(Event::Start(BytesStart::new("SDG-CAPTION")))?;
            if let Some(short_name) = caption.get_short_name() {
                let short_name_element = BytesStart::new("SHORT-NAME");
                write_text_element(writer, "SHORT-NAME", short_name_element, Some(short_name))?;
            }
            // caption.get_desc() → MultiLanguageOverviewParagraph is a P0
            // placeholder; nothing to emit yet.
            writer.write_event(Event::End(BytesEnd::new("SDG-CAPTION")))?;
        }

        // py writeSds / writeSdfs / nested setSdg
        if let Some(contents) = sdg
            .get_sdg_contents_type()
            .and_then(|id| document.sdg_contents.get(id))
        {
            for sd_id in contents.get_sd() {
                if let Some(sd) = document.sds.get(*sd_id) {
                    self.write_sd(writer, sd)?;
                }
            }
            for sdf_id in contents.get_sdf() {
                if let Some(sdf) = document.sdfs.get(*sdf_id) {
                    let mut sdf_element = BytesStart::new("SDF");
                    self.write_ar_object_attributes(&mut sdf_element, sdf.base());
                    if let Some(gid) = sdf.get_gid() {
                        sdf_element.push_attribute(("GID", gid));
                    }
                    write_text_element(writer, "SDF", sdf_element, sdf.get_value())?;
                }
            }
            for nested_id in contents.get_sdg() {
                if let Some(nested) = document.sdgs.get(*nested_id) {
                    self.write_sdg(writer, nested, document)?;
                }
            }
        }

        writer.write_event(Event::End(BytesEnd::new("SDG")))?;
        Ok(())
    }

    /// py `writeSds` per-SD body
    fn write_sd<W: Write>(&self, writer: &mut Writer<W>, sd: &Sd) -> Result<(), WriteError> {
        let mut element = BytesStart::new("SD");
        self.write_ar_object_attributes(&mut element, sd.base());
        if let Some(gid) = sd.get_gid() {
            element.push_attribute(("GID", gid));
        }
        if let Some(xml_space) = sd.get_xml_space() {
            let xml_space = xml_space.to_string();
            element.push_attribute(("xml:space", xml_space.as_str()));
        }
        write_text_element(writer, "SD", element, sd.get_value())?;
        Ok(())
    }

    /// py `writeARPackages`
    fn write_ar_packages<W: Write>(
        &self,
        writer: &mut Writer<W>,
        packages: &[Id<ARPackage>],
        document: &Document,
    ) -> Result<(), WriteError> {
        if packages.is_empty() {
            return Ok(());
        }
        writer.write_event(Event::Start(BytesStart::new("AR-PACKAGES")))?;
        for package_id in packages {
            if let Some(package) = document.ar_packages.get(*package_id) {
                self.write_ar_package(writer, package, document)?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("AR-PACKAGES")))?;
        Ok(())
    }

    /// py `writeARPackage`
    fn write_ar_package<W: Write>(
        &self,
        writer: &mut Writer<W>,
        package: &ARPackage,
        document: &Document,
    ) -> Result<(), WriteError> {
        // py writeIdentifiable attribute order on the AR-PACKAGE tag: S, T, UUID.
        let mut element = BytesStart::new("AR-PACKAGE");
        if let Some(checksum) = package.get_checksum() {
            element.push_attribute(("S", checksum));
        }
        if let Some(timestamp) = package.get_timestamp() {
            element.push_attribute(("T", timestamp));
        }
        if let Some(uuid) = package.get_uuid() {
            element.push_attribute(("UUID", uuid));
        }
        writer.write_event(Event::Start(element))?;

        if let Some(short_name) = package.get_short_name() {
            let short_name_element = BytesStart::new("SHORT-NAME");
            write_text_element(writer, "SHORT-NAME", short_name_element, Some(short_name))?;
        }

        // py writeReferenceBases / writeARPackageElements — nothing to emit in
        // P0 (both lists are empty for this file).

        // py writeARPackages (nested packages last)
        self.write_ar_packages(writer, package.get_ar_packages(), document)?;

        writer.write_event(Event::End(BytesEnd::new("AR-PACKAGE")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::msr::asam_hdo::special_data::SdgContents;
    use crate::m2::msr::documentation::text_model::language_data_model::XmlSpace;

    /// Builds the fixture model through the Document API and serializes it.
    fn fixture_document() -> Document {
        let mut document = Document::new();
        document.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd");
        document.set_ar_release("R21-11");

        let mut l10 = LPlainText::new();
        l10.set_l("EN")
            .set_xml_space(XmlSpace::Preserve)
            .set_value("English");
        let l10_id = document.l_plain_texts.alloc(l10);

        let mut paragraph = crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguagePlainText::new();
        paragraph.push_l10(l10_id);
        let paragraph_id = document.multi_language_plain_texts.alloc(paragraph);

        let mut sd = Sd::new();
        sd.set_gid("purpose")
            .set_xml_space(XmlSpace::Preserve)
            .set_value("special   data");
        let sd_id = document.sds.alloc(sd);

        let mut contents = SdgContents::new();
        contents.push_sd(sd_id);
        let contents_id = document.sdg_contents.alloc(contents);

        let mut sdg = Sdg::new();
        sdg.set_gid("demo").set_sdg_contents_type(contents_id);
        let sdg_id = document.sdgs.alloc(sdg);

        let mut admin_data = AdminData::new();
        admin_data.set_used_languages(paragraph_id);
        admin_data.push_sdg(sdg_id);
        let admin_data_id = document.admin_datas.alloc(admin_data);
        document.set_admin_data(admin_data_id);

        document.add_ar_package(None, "WhitespaceDemo");
        document
    }

    #[test]
    fn save_emits_namespaced_root_and_preserved_whitespace() {
        let document = fixture_document();
        let output = std::env::temp_dir().join(format!("armodel_writer_test_{}.arxml", std::process::id()));
        ARXMLWriter::new().save(&output, &document).unwrap();

        let text = std::fs::read_to_string(&output).unwrap();
        let _ = std::fs::remove_file(&output);

        assert!(text.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(text.contains(
            "<AUTOSAR xmlns=\"http://autosar.org/schema/r4.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd\">"
        ));
        assert!(text.contains("<L-10 L=\"EN\" xml:space=\"preserve\">English</L-10>"));
        assert!(text.contains("<SD GID=\"purpose\" xml:space=\"preserve\">special   data</SD>"));
        assert!(text.contains("<SHORT-NAME>WhitespaceDemo</SHORT-NAME>"));
    }
}
```

Update `src/lib.rs`:

```rust
pub mod m2;
pub mod parser;
pub mod writer;

pub use m2::autosar_templates::autosar_top_level_structure::Document;
pub use parser::abstract_arxml_parser::ParseError;
pub use parser::arxml_parser::{ARXMLParser, ParserOptions, default_options};
pub use writer::abstract_arxml_writer::WriteError;
pub use writer::arxml_writer::ARXMLWriter;
```

- [x] **Step 3: Run the tests**

Run: `cargo test --lib arxml_writer`
Expected: PASS (3 tests: 2 in abstract, 1 here).

- [x] **Step 4: Format, lint, commit**

```bash
cargo fmt
cargo clippy -- -D warnings
git add src/writer src/lib.rs
git commit -m "feat(writer): ARXMLWriter::save with namespaces, ADMIN-DATA and AR-PACKAGES"
```

---

### Task 12: Round-trip integration test

**Files:**
- Create: `tests/integration/test_files/AdminDataWhitespace.arxml` (verbatim copy from py-armodel — never regenerate)
- Create: `tests/integration/roundtrip.rs`

- [x] **Step 1: Copy the fixture verbatim**

```bash
mkdir -p tests/integration/test_files
cp ../py-armodel/tests/integration_tests/test_files/AdminDataWhitespace.arxml tests/integration/test_files/
cmp tests/integration/test_files/AdminDataWhitespace.arxml ../py-armodel/tests/integration_tests/test_files/AdminDataWhitespace.arxml && echo identical
```

Expected: `identical` (no output from `cmp`). If `../py-armodel` is absent,
fetch the file from
https://github.com/melodypapa/py-armodel/blob/main/tests/integration_tests/test_files/AdminDataWhitespace.arxml —
it must be byte-identical (UTF-8, LF endings, trailing newline):

```xml
<?xml version="1.0" encoding="UTF-8"?>
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
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>WhitespaceDemo</SHORT-NAME>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>
```

- [x] **Step 2: Write the round-trip test**

`tests/integration/roundtrip.rs`:

```rust
//! P0 acceptance (P0 design §2, §8):
//! parse → write → re-parse → assert_structurally_equal, plus the
//! whitespace-preserving SD text assertion.

use std::path::Path;

use armodel::m2::msr::documentation::text_model::language_data_model::XmlSpace;
use armodel::parser::arxml_parser::{ARXMLParser, default_options};
use armodel::writer::arxml_writer::ARXMLWriter;
use armodel::Document;

#[test]
fn admin_data_whitespace_roundtrip() {
    let source = Path::new("tests/integration/test_files/AdminDataWhitespace.arxml");

    // 1. parse into a Document
    let mut document = Document::new();
    ARXMLParser::new(default_options())
        .load(source, &mut document)
        .unwrap();

    // release detection: AUTOSAR_00050.xsd → R21-11
    assert_eq!(document.get_ar_release(), "R21-11");

    // 2. write to a temp file
    let output = std::env::temp_dir().join("armodel_roundtrip_AdminDataWhitespace.arxml");
    ARXMLWriter::new().save(&output, &document).unwrap();

    // 3. re-parse
    let mut reparsed = Document::new();
    ARXMLParser::new(default_options())
        .load(&output, &mut reparsed)
        .unwrap();

    // 4. structural equality (model equality only — no written-text diff in P0)
    document.assert_structurally_equal(&reparsed).unwrap();

    // 5. the whitespace-preserving field round-trips exactly
    let admin_data = document.get_admin_data().unwrap();
    let sdg = document.get_sdg(admin_data.get_sdgs()[0]).unwrap();
    let contents = document
        .get_sdg_contents(sdg.get_sdg_contents_type().unwrap())
        .unwrap();
    let sd = document.get_sd(contents.get_sd()[0]).unwrap();
    assert_eq!(sd.get_value(), Some("special   data"));
    assert_eq!(sd.get_xml_space(), Some(XmlSpace::Preserve));

    // and the used-languages entry too
    let paragraph = document
        .get_multi_language_plain_text(admin_data.get_used_languages().unwrap())
        .unwrap();
    let l10 = document.get_l_plain_text(paragraph.get_l10s()[0]).unwrap();
    assert_eq!(l10.get_value(), Some("English"));

    let _ = std::fs::remove_file(&output);
}
```

Note: cargo runs integration tests with the package root as CWD, so the
relative fixture path works.

- [x] **Step 3: Run the integration test**

Run: `cargo test --test roundtrip -- --nocapture`
Expected: PASS (1 test).

- [x] **Step 4: Run everything, format, lint, commit**

```bash
cargo test
cargo fmt
cargo clippy -- -D warnings
git add tests/integration
git commit -m "test: P0 round-trip over AdminDataWhitespace.arxml"
```

---

### Task 13: arxml-dump binary + final verification sweep

**Files:**
- Modify: `src/bin/arxml-dump.rs`

- [x] **Step 1: Wire the binary to the new API**

Replace `src/bin/arxml-dump.rs` with:

```rust
use std::env;
use std::path::Path;
use std::process;

use getopts::Options;

use armodel::Document;
use armodel::parser::arxml_parser::{ARXMLParser, default_options};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {program} [options]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "arxml", "Set arxml file name", "NAME");
    opts.optflag("h", "help", "Show this help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{f}");
            process::exit(1);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    let Some(path) = matches.opt_str("a") else {
        print_usage(&program, &opts);
        return;
    };

    let mut document = Document::new();
    if let Err(error) = ARXMLParser::new(default_options()).load(Path::new(&path), &mut document) {
        eprintln!("Failed to parse {path}: {error}");
        process::exit(1);
    }

    println!("AR release: {}", document.get_ar_release());
    for package_id in document.get_ar_packages() {
        match document.get_ar_package(*package_id) {
            Some(package) => {
                let name = package.get_short_name().unwrap_or("<no SHORT-NAME>");
                println!("AR-PACKAGE {name}");
            }
            None => println!("AR-PACKAGE <unresolved>"),
        }
    }
}
```

(The P0 design says the binary's behaviour is "unchanged" — it stays a dump
utility with the same `-a`/`-h` options; loading and printing top-level
package names is the minimal compile-against-the-API validation, and errors
now print + `exit(1)` instead of panicking, per `docs/code_guide.md` §7.)

- [x] **Step 2: Manual CLI check**

```bash
cargo run --bin arxml-dump -- -a tests/integration/test_files/AdminDataWhitespace.arxml
```

Expected stdout:

```
AR release: R21-11
AR-PACKAGE WhitespaceDemo
```

Also check the failure path:

```bash
cargo run --bin arxml-dump -- -a /nonexistent.arxml; echo "exit=$?"
```

Expected: `Failed to parse …` on stderr and `exit=1`.

- [x] **Step 3: Full verification sweep**

```bash
cargo build
cargo test
cargo fmt -- --check
cargo clippy -- -D warnings
```

Expected: all green.

- [x] **Step 4: Commit**

```bash
git add src/bin/arxml-dump.rs
git commit -m "feat(bin): arxml-dump loads ARXML through the new Document API"
```

---

## Acceptance checklist (maps to spec §2)

- [x] `cargo test --test roundtrip` passes: parse → write → re-parse → `assert_structurally_equal`.
- [x] SD text assertion holds: `Some("special   data")` with `xml:space == Preserve`.
- [x] `cargo build` and `cargo test` clean (Travis's gates).
- [x] `cargo fmt --check` and `cargo clippy -- -D warnings` clean.
- [x] No `unwrap`/`expect`/`panic!` in library code (allowed in `#[cfg(test)]` and `src/bin/`).
- [x] No `PartialEq` derived on arena-linked model types; equality only via `Document::assert_structurally_equal`.
- [x] Fixture is a byte-identical copy of py-armodel's `AdminDataWhitespace.arxml`.

## Deliberate P0 simplifications (documented divergences)

1. **MultiLanguagePlainText has no ARObject base in P0** (spec §5 field list);
   py reads S/T attributes on `<USED-LANGUAGES>` — dropped, restored in P1 by
   the converter.
2. **SD/SDF text follows the P0 whitespace rule** (trim unless
   `xml:space="preserve"`), while py stores raw text whenever it is non-blank.
   Identical on the fixture; normalizes uglier inputs.
3. **DocRevision, MultiLanguageOverviewParagraph, Modification** are empty
   placeholders; the parser warns on DOC-REVISIONS content instead of parsing.
4. **Writer options** (py's `warning`/`version`/`unescape_entities`) are not
   ported; `ARXMLWriter::new()` uses fixed defaults.
5. **Legacy R3.x namespaces/wrappers** (TOP-LEVEL-PACKAGES/SUB-PACKAGES) are
   not handled; P0 targets the R4.x fixture only.
