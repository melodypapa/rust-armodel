//! spec: M2::MSR::Documentation::TextModel::MultilanguageData
//!
//! Spec classes `MultiLanguagePlainText`, `MultiLanguageOverviewParagraph`
//! (P0 design §5).

use slotmap::new_key_type;

use super::language_data_model::LPlainTextId;

new_key_type! {
    /// Arena keys for the `MultilanguageData` classes.
    pub struct MultiLanguagePlainTextId;
    pub struct MultiLanguageOverviewParagraphId;
}

/// spec class `MultiLanguagePlainText` — the `<USED-LANGUAGES>` content.
/// P0 note: no `ARObject` base (spec §5 field list), so S/T attributes on
/// `<USED-LANGUAGES>` are not carried; the P1 converter restores them.
#[derive(Debug, Default)]
pub struct MultiLanguagePlainText {
    l10s: Vec<LPlainTextId>,
}

impl MultiLanguagePlainText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_l10s(&self) -> &[LPlainTextId] {
        &self.l10s
    }

    /// py `addL10`
    pub fn push_l10(&mut self, l10: LPlainTextId) {
        self.l10s.push(l10);
    }
}

/// spec class `MultiLanguageOverviewParagraph`.
/// P0 placeholder (referenced by `SdgCaption.desc`); fields arrive in P1.
#[derive(Debug, Default)]
pub struct MultiLanguageOverviewParagraph;

#[cfg(test)]
mod tests {
    use super::super::language_data_model::LPlainText;
    use super::*;
    use slotmap::SlotMap;

    #[test]
    fn multi_language_plain_text_collects_l10s() {
        let mut paragraph = MultiLanguagePlainText::new();

        let mut arena: SlotMap<LPlainTextId, LPlainText> = SlotMap::with_key();
        let mut l10 = LPlainText::new();
        l10.set_l("EN");
        let id = arena.insert(l10);

        paragraph.push_l10(id);
        assert_eq!(paragraph.get_l10s(), &[id]);
    }
}
