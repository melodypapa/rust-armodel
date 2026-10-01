//! spec: M2::MSR::AsamHdo::AdminData
//!
//! Spec classes `AdminData`, `DocRevision`, `Modification` (P0 design §5).

use slotmap::new_key_type;

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::msr::asam_hdo::special_data::SdgId;
use crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguagePlainTextId;

new_key_type! {
    /// Arena keys for the `AdminData` classes.
    pub struct AdminDataId;
    pub struct DocRevisionId;
}

/// spec class `DocRevision`.
/// P0 placeholder: `AdminDataWhitespace.arxml` has none; fields arrive in P1
/// from the converter (P0 design §5).
#[derive(Debug, Default)]
pub struct DocRevision;

/// spec class `AdminData` — `AdminData : ARObject`.
#[derive(Debug, Default)]
pub struct AdminData {
    base: ARObject,
    doc_revisions: Vec<DocRevisionId>,
    language: Option<String>,
    sdgs: Vec<SdgId>,
    used_languages: Option<MultiLanguagePlainTextId>,
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

    pub fn get_doc_revisions(&self) -> &[DocRevisionId] {
        &self.doc_revisions
    }

    /// py `addDocRevision`
    pub fn push_doc_revision(&mut self, revision: DocRevisionId) {
        self.doc_revisions.push(revision);
    }

    pub fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub fn set_language(&mut self, value: impl Into<String>) -> &mut Self {
        self.language = Some(value.into());
        self
    }

    pub fn get_sdgs(&self) -> &[SdgId] {
        &self.sdgs
    }

    /// py `addSdg`
    pub fn push_sdg(&mut self, sdg: SdgId) {
        self.sdgs.push(sdg);
    }

    /// py `getUsedLanguages` — a **single** `MultiLanguagePlainText`, not a list.
    pub fn get_used_languages(&self) -> Option<MultiLanguagePlainTextId> {
        self.used_languages
    }

    /// py `setUsedLanguages`
    pub fn set_used_languages(&mut self, used_languages: MultiLanguagePlainTextId) -> &mut Self {
        self.used_languages = Some(used_languages);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguagePlainText;
    use slotmap::SlotMap;

    #[test]
    fn admin_data_accessors() {
        let mut admin_data = AdminData::new();
        assert_eq!(admin_data.get_language(), None);
        assert!(admin_data.get_sdgs().is_empty());
        assert!(admin_data.get_doc_revisions().is_empty());
        assert!(admin_data.get_used_languages().is_none());

        let mut arena: SlotMap<MultiLanguagePlainTextId, MultiLanguagePlainText> =
            SlotMap::with_key();
        let mlpt_id = arena.insert(MultiLanguagePlainText::new());

        admin_data.set_language("EN").set_used_languages(mlpt_id);

        assert_eq!(admin_data.get_language(), Some("EN"));
        assert_eq!(admin_data.get_used_languages(), Some(mlpt_id));
    }
}
