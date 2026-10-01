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
