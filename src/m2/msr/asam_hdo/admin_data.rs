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
