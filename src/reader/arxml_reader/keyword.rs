//! KEYWORD / KEYWORD-SET readers. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::common_structure::standardization_template::keyword::{
    Keyword, KeywordId, KeywordSetId,
};

impl ARXMLReader {
    /// py `readKeyword` — payload + ABBR-NAME + CLASSIFICATIONS texts.
    fn read_keyword(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<KeywordId, ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let abbr_name = get_child_element_string(element, "ABBR-NAME").map(str::to_string);
        // py readKeywordClassifications
        let mut classifications = Vec::new();
        for class_node in find_all(element, "CLASSIFICATIONS/CLASSIFICATION") {
            if let Some(text) = &class_node.text {
                classifications.push(text.as_str().to_string());
            }
        }
        let short_name = get_short_name(element)?;
        let mut keyword = Keyword::new();
        keyword.set_short_name(short_name);
        if let Some(checksum) = element.attrs.get("S") {
            keyword.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = element.attrs.get("T") {
            keyword.set_timestamp(timestamp.as_str());
        }
        if let Some(uuid) = element.attrs.get("UUID") {
            keyword.set_uuid(uuid);
        }
        if let Some(category) = get_child_element_string(element, "CATEGORY") {
            keyword.set_category(category);
        }
        if let Some(long_name) = payload.long_name {
            keyword.set_long_name(long_name);
        }
        if let Some(desc) = payload.desc {
            keyword.set_desc(desc);
        }
        if let Some(introduction) = payload.introduction {
            keyword.set_introduction(introduction);
        }
        if let Some(admin_data) = payload.admin_data {
            keyword.set_admin_data(admin_data);
        }
        if let Some(abbr_name) = abbr_name {
            keyword.set_abbr_name(abbr_name);
        }
        for classification in classifications {
            keyword.push_classification(classification);
        }
        Ok(document.keywords.insert(keyword))
    }

    /// py `readKeywordSet` — Identifiable chain + KEYWORDS children.
    pub(super) fn read_keyword_set(
        &mut self,
        element: &Node,
        id: KeywordSetId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let mut keywords = Vec::new();
        for keyword_node in find_all(element, "KEYWORDS/KEYWORD") {
            keywords.push(self.read_keyword(keyword_node, document)?);
        }
        if let Some(keyword_set) = document.keyword_sets.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                keyword_set.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                keyword_set.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                keyword_set.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                keyword_set.set_admin_data(admin_data);
            }
            for keyword in keywords {
                keyword_set.push_keyword(keyword);
            }
        }
        Ok(())
    }
}
