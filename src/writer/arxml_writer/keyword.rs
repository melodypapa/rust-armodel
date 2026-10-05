//! KEYWORD / KEYWORD-SET emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::common_structure::standardization_template::keyword::{
    Keyword, KeywordSetId,
};

impl ARXMLWriter {
    /// py `writeKeyword` — chain, ABBR-NAME, CLASSIFICATIONS.
    fn write_keyword<W: Write>(
        &self,
        writer: &mut Writer<W>,
        keyword: &Keyword,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("KEYWORD");
        self.write_identifiable_attributes(
            &mut element,
            keyword.get_checksum(),
            keyword.get_timestamp(),
            keyword.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = keyword.get_short_name() {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_identifiable_parts(
            writer,
            IdentifiableParts {
                long_name: keyword.get_long_name(),
                desc: keyword.get_desc(),
                category: keyword.get_category(),
                introduction: keyword.get_introduction(),
                admin_data: keyword.get_admin_data(),
                sw_data_def_props: None,
            },
            document,
        )?;
        write_optional_text_element(writer, "ABBR-NAME", keyword.get_abbr_name())?;
        let classifications = keyword.get_classifications();
        if !classifications.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("CLASSIFICATIONS")))?;
            for classification in classifications {
                write_text_element(
                    writer,
                    "CLASSIFICATION",
                    BytesStart::new("CLASSIFICATION"),
                    Some(classification),
                )?;
            }
            writer.write_event(Event::End(BytesEnd::new("CLASSIFICATIONS")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("KEYWORD")))?;
        Ok(())
    }

    /// py `writeKeywordSet`.
    pub(super) fn write_keyword_set<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: KeywordSetId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(keyword_set) = document.keyword_sets.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("KEYWORD-SET");
        self.write_identifiable_attributes(
            &mut element,
            keyword_set.get_checksum(),
            keyword_set.get_timestamp(),
            keyword_set.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = keyword_set.get_short_name() {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_identifiable_parts(
            writer,
            IdentifiableParts {
                long_name: keyword_set.get_long_name(),
                desc: keyword_set.get_desc(),
                category: keyword_set.get_category(),
                introduction: keyword_set.get_introduction(),
                admin_data: keyword_set.get_admin_data(),
                sw_data_def_props: None,
            },
            document,
        )?;
        let keywords = keyword_set.get_keywords();
        if !keywords.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("KEYWORDS")))?;
            for keyword_id in keywords {
                if let Some(keyword) = document.keywords.get(*keyword_id) {
                    self.write_keyword(writer, keyword, document)?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("KEYWORDS")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("KEYWORD-SET")))?;
        Ok(())
    }
}
