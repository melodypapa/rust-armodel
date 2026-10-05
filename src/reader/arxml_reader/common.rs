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
