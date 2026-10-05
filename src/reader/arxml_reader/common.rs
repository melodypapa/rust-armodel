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
}
