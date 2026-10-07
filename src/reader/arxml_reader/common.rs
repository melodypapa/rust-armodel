//! ARObject-level helpers shared by every domain reader (py's abstract-level
//! readARObject / readIdentifiable chain). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

impl ARXMLReader {
    /// py `readARObject` — reads the `S` (checksum) and `T` (timestamp)
    /// attributes.
    pub(super) fn read_ar_object(&mut self, element: &Node, ar_object: &mut ARObject) {
        if let Some(checksum) = element.attrs.get("S") {
            ar_object.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = element.attrs.get("T") {
            ar_object.set_timestamp(timestamp.as_str());
        }
    }

    /// py `readWhitespaceControlled` — reads the element's own `xml:space`.
    pub(super) fn read_xml_space(
        &mut self,
        element: &Node,
    ) -> Result<Option<XmlSpace>, ParseError> {
        match element.attrs.get("xml:space").map(String::as_str) {
            Some(value) => match XmlSpace::try_from(value) {
                Ok(xml_space) => Ok(Some(xml_space)),
                Err(_) => {
                    let message = format!(
                        "Unsupported xml:space value <{value}> on <{}>",
                        element.name
                    );
                    self.not_implemented(message)?;
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }

    /// py `readIdentifiable` — everything under the MultilanguageReferrable
    /// chain except SHORT-NAME/UUID/CATEGORY (read by the allocation path)
    /// and annotations/variation points (no pinned fixture carries them;
    /// tracked on the port checklist).
    pub(super) fn read_identifiable_payload(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<IdentifiablePayload, ParseError> {
        Ok(IdentifiablePayload {
            long_name: self.get_multilanguage_long_name(element, document)?,
            desc: self.get_multi_language_overview_paragraph(element, document)?,
            introduction: self.get_documentation_block(element, document)?,
            admin_data: match find(element, "ADMIN-DATA") {
                Some(node) => Some(self.read_admin_data(node, document)?),
                None => None,
            },
        })
    }
    /// py `readARElement` — the ARElement chain level: ARElement adds no own
    /// payload, so this passes the Identifiable payload through.
    pub(super) fn read_ar_element(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<IdentifiablePayload, ParseError> {
        self.read_identifiable_payload(element, document)
    }
}
/// The `Identifiable`-owned XML payload read by `read_identifiable_payload`
/// (py `readIdentifiable` minus SHORT-NAME/UUID/CATEGORY, which the
/// allocation path already consumed, and minus annotations — no pinned
/// fixture carries `ANNOTATIONS`; tracked on the port checklist).
pub(super) struct IdentifiablePayload {
    pub(super) long_name: Option<MultilanguageLongNameId>,
    pub(super) desc: Option<MultiLanguageOverviewParagraphId>,
    pub(super) introduction: Option<DocumentationBlockId>,
    pub(super) admin_data: Option<AdminDataId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::autosar_templates::common_structure::standardization_template::keyword::KeywordSet;

    /// py `readARObject` — the S (checksum) / T (timestamp) attributes must
    /// arrive on every path that builds an ARObject-derived object: the
    /// document-level ADMIN-DATA (non-package-element reader), the AR-PACKAGE
    /// itself, and package elements (allocation loop).
    /// (Relocated from PR #18's arxml_reader/mod.rs tests by the domain-split
    /// merge — see docs/plan/sync-todo/Group1.md ARObject row.)
    const AR_OBJECT_S_T_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00052.xsd">
  <ADMIN-DATA S="c1" T="2024-11-26T21:25:17+08:00"/>
  <AR-PACKAGES>
    <AR-PACKAGE S="c2" T="2024-11-27T08:00:00+08:00">
      <SHORT-NAME>Pkg</SHORT-NAME>
      <ELEMENTS>
        <SW-BASE-TYPE S="c3" T="2024-11-28T09:30:00+08:00">
          <SHORT-NAME>T</SHORT-NAME>
        </SW-BASE-TYPE>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    #[test]
    fn ar_object_s_t_attributes_parse_on_every_read_path() {
        let mut document = Document::new();
        ARXMLReader::new(default_options())
            .load_from_reader(Reader::from_str(AR_OBJECT_S_T_SAMPLE), &mut document)
            .unwrap();

        let admin_data = document.get_admin_data().unwrap();
        assert_eq!(admin_data.get_checksum(), Some("c1"));
        assert_eq!(
            admin_data.get_timestamp(),
            Some("2024-11-26T21:25:17+08:00")
        );

        let pkg = document
            .get_ar_package(document.get_ar_packages()[0])
            .unwrap();
        assert_eq!(pkg.get_checksum(), Some("c2"));
        assert_eq!(pkg.get_timestamp(), Some("2024-11-27T08:00:00+08:00"));

        let ElementRef::SwBaseType(id) = pkg.get_elements()[0] else {
            panic!("expected SwBaseType element");
        };
        let base_type = document.get_sw_base_type(id).unwrap();
        assert_eq!(base_type.get_checksum(), Some("c3"));
        assert_eq!(base_type.get_timestamp(), Some("2024-11-28T09:30:00+08:00"));
    }

    /// py `readARElement` — the ARElement chain level passes the Identifiable
    /// payload through: LONG-NAME, DESC, INTRODUCTION, ADMIN-DATA resolve to
    /// populated arenas. Exercised directly on a KEYWORD-SET fragment (py
    /// readKeywordSet is the ported-domain caller of readARElement; the
    /// KEYWORD-SET end-to-end path itself is covered by the graduated
    /// KeywordSet_Blueprint fixture).
    #[test]
    fn ar_element_level_reads_identifiable_payload() {
        const KEYWORD_SET_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<KEYWORD-SET>
  <SHORT-NAME>Ks</SHORT-NAME>
  <LONG-NAME>
    <L-4 L="EN">Keyword Set Long Name</L-4>
  </LONG-NAME>
  <DESC>
    <L-2 L="EN">Keyword set description</L-2>
  </DESC>
  <INTRODUCTION>
    <P>
      <L-1 L="EN">Introduction paragraph</L-1>
    </P>
  </INTRODUCTION>
  <ADMIN-DATA>
    <LANGUAGE>EN</LANGUAGE>
  </ADMIN-DATA>
</KEYWORD-SET>"#;

        let node = build_dom_from_reader(Reader::from_str(KEYWORD_SET_SAMPLE)).unwrap();

        let mut document = Document::new();
        document.keyword_sets.insert(KeywordSet::new());
        let payload = ARXMLReader::new(default_options())
            .read_ar_element(&node, &mut document)
            .unwrap();

        let long_name = document
            .get_multilanguage_long_name(payload.long_name.unwrap())
            .unwrap();
        let l4 = document.get_l_long_name(long_name.get_l4()[0]).unwrap();
        assert_eq!(l4.get_value(), Some("Keyword Set Long Name"));
        assert_eq!(l4.get_l(), Some("EN"));

        let desc = document
            .get_multi_language_overview_paragraph(payload.desc.unwrap())
            .unwrap();
        let l2 = document.get_l_overview_paragraph(desc.get_l2()[0]).unwrap();
        assert_eq!(l2.get_value(), Some("Keyword set description"));

        let block = document
            .get_documentation_block(payload.introduction.unwrap())
            .unwrap();
        let paragraph = document
            .get_multi_language_paragraph(block.get_ps()[0])
            .unwrap();
        let l1 = document.get_l_paragraph(paragraph.get_l1()[0]).unwrap();
        assert_eq!(l1.get_value(), Some("Introduction paragraph"));

        let admin_data = document
            .admin_datas
            .get(payload.admin_data.unwrap())
            .unwrap();
        assert_eq!(admin_data.get_language(), Some("EN"));
    }
}
