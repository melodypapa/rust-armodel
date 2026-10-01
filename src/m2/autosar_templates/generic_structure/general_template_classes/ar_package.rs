//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ARPackage
//!
//! Spec classes `PackageableElement`, `ARElement`, `ARPackage`,
//! `ReferenceBase` (P0 design §5). Note the spec chain:
//! `ARPackage : CollectableElement` (py-armodel mirrors this — ARPackage does
//! NOT derive from ARElement).
//!
//! Children live in `Document`-owned arenas and are referenced by id —
//! never `Vec<ARPackage>` by value (`docs/code_guide.md` §4).

use slotmap::new_key_type;

use super::ar_object::ElementRef;
use super::element_collection::CollectableElement;
use crate::m2::msr::asam_hdo::admin_data::AdminDataId;

new_key_type! {
    /// Arena keys for the `ARPackage` classes.
    pub struct ARPackageId;
    pub struct ReferenceBaseId;
}

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
    ar_packages: Vec<ARPackageId>,
    reference_bases: Vec<ReferenceBaseId>,
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
        // self.base is CollectableElement; three hops reach Referrable.
        self.base
            .base_mut()
            .base_mut()
            .base_mut()
            .set_parent(parent);
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

    pub fn get_admin_data(&self) -> Option<AdminDataId> {
        self.base.base().get_admin_data()
    }

    pub fn set_admin_data(&mut self, admin_data: AdminDataId) -> &mut Self {
        self.base.base_mut().set_admin_data(admin_data);
        self
    }

    // — own fields —

    pub fn get_elements(&self) -> &[ElementRef] {
        &self.elements
    }

    // Called from Document's factories (Task 8); until then only tests use
    // them, which the lib build does not see.
    #[allow(dead_code)]
    pub(crate) fn push_element(&mut self, element: ElementRef) {
        self.elements.push(element);
    }

    pub fn get_ar_packages(&self) -> &[ARPackageId] {
        &self.ar_packages
    }

    #[allow(dead_code)]
    pub(crate) fn push_ar_package(&mut self, ar_package: ARPackageId) {
        self.ar_packages.push(ar_package);
    }

    pub fn get_reference_bases(&self) -> &[ReferenceBaseId] {
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
    use slotmap::SlotMap;

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
        let mut arena: SlotMap<ARPackageId, ARPackage> = SlotMap::with_key();
        let parent_id = arena.insert(ARPackage::new());

        let mut child = ARPackage::new();
        child.set_parent(Some(ElementRef::ARPackage(parent_id)));

        // The push methods are what Document's factories call (Task 8);
        // exercise them at the struct level here.
        let mut package = ARPackage::new();
        package.push_ar_package(parent_id);
        package.push_element(ElementRef::ARPackage(parent_id));
        assert_eq!(package.get_ar_packages(), &[parent_id]);
        assert_eq!(package.get_elements(), &[ElementRef::ARPackage(parent_id)]);

        assert!(matches!(child.get_parent(), Some(ElementRef::ARPackage(id)) if id == parent_id));
    }
}
