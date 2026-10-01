//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::Identifiable
//!
//! Spec classes `Referrable`, `MultilanguageReferrable`, `Identifiable`
//! (P0 design §5), modelled as composition per `docs/code_guide.md` §3.

use super::ar_object::{ARObject, ElementRef};
use crate::m2::msr::asam_hdo::admin_data::AdminDataId;

/// spec class `Referrable` — `Referrable : ARObject`.
#[derive(Debug, Default)]
pub struct Referrable {
    base: ARObject,
    parent: Option<ElementRef>,
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

    pub fn get_parent(&self) -> Option<ElementRef> {
        self.parent
    }

    pub fn set_parent(&mut self, parent: Option<ElementRef>) -> &mut Self {
        self.parent = parent;
        self
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

/// spec class `Identifiable` — `Identifiable : MultilanguageReferrable`.
///
/// The remaining spec fields (desc, introduction, annotations, long_name)
/// arrive in P1 from the converter (P0 design §5).
#[derive(Debug, Default)]
pub struct Identifiable {
    base: MultilanguageReferrable,
    uuid: Option<String>,
    category: Option<String>,
    admin_data: Option<AdminDataId>,
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

    pub fn get_admin_data(&self) -> Option<AdminDataId> {
        self.admin_data
    }

    pub fn set_admin_data(&mut self, admin_data: AdminDataId) -> &mut Self {
        self.admin_data = Some(admin_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::msr::asam_hdo::admin_data::AdminData;

    #[test]
    fn referrable_accessors() {
        let mut referrable = Referrable::new();
        assert_eq!(referrable.get_short_name(), None);

        referrable.set_short_name("Pkg");
        assert_eq!(referrable.get_short_name(), Some("Pkg"));
        assert_eq!(referrable.base().get_checksum(), None);
        // parent is set through Document::add_ar_package (Task 8).
        assert!(referrable.get_parent().is_none());
    }

    #[test]
    fn multilanguage_referrable_forwards_short_name() {
        let mut referrable = MultilanguageReferrable::new();
        referrable.set_short_name("X");
        assert_eq!(referrable.get_short_name(), Some("X"));
        assert_eq!(referrable.base().get_short_name(), Some("X"));
    }

    #[test]
    fn identifiable_accessors() {
        let mut identifiable = Identifiable::new();
        assert_eq!(identifiable.get_uuid(), None);
        assert_eq!(identifiable.get_category(), None);
        assert!(identifiable.get_admin_data().is_none());

        let mut arena: slotmap::SlotMap<AdminDataId, AdminData> = slotmap::SlotMap::with_key();
        let admin_data_id = arena.insert(AdminData::new());

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
}
