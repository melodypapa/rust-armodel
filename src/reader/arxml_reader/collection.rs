//! Collection readers (py readCollection, the CollectableElement wrapper with
//! AUTO-COLLECT). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::generic_structure::general_template_classes::element_collection::{
    AutoCollectEnum, CollectionId,
};

impl ARXMLReader {
    /// py `readCollection`.
    pub(super) fn read_collection(
        &mut self,
        element: &Node,
        id: CollectionId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        // AUTO_COLLECT_XML_MAP: XML token → enum literal; unknown tokens are
        // a warning, matching py.
        let auto_collect = match find(element, "AUTO-COLLECT") {
            Some(node) => match node.text.as_deref().map(str::trim) {
                Some("REF-ALL") => Some(AutoCollectEnum::RefAll),
                Some("REF-NONE") => Some(AutoCollectEnum::RefNone),
                Some("REF-NON-STANDARD") => Some(AutoCollectEnum::RefNonStandard),
                other => {
                    let message = format!("Unsupported AUTO-COLLECT <{}>", other.unwrap_or(""));
                    self.not_implemented(message)?;
                    None
                }
            },
            None => None,
        };
        let collection_semantics =
            get_child_element_string(element, "COLLECTION-SEMANTICS").map(str::to_string);
        let element_role = get_child_element_string(element, "ELEMENT-ROLE").map(str::to_string);
        let element_refs = get_child_element_ref_type_list(element, "ELEMENT-REFS/ELEMENT-REF");
        let source_element_refs =
            get_child_element_ref_type_list(element, "SOURCE-ELEMENT-REFS/SOURCE-ELEMENT-REF");
        // RefType values live in the arena; the model links by id.
        let element_ref_ids: Vec<_> = element_refs
            .into_iter()
            .map(|r#ref| document.ref_types.insert(r#ref))
            .collect();
        let source_ref_ids: Vec<_> = source_element_refs
            .into_iter()
            .map(|r#ref| document.ref_types.insert(r#ref))
            .collect();

        if let Some(collection) = document.collections.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                collection.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                collection.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                collection.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                collection.set_admin_data(admin_data);
            }
            if let Some(auto_collect) = auto_collect {
                collection.set_auto_collect(auto_collect);
            }
            if let Some(semantics) = collection_semantics {
                collection.set_collection_semantics(semantics);
            }
            if let Some(role) = element_role {
                collection.set_element_role(role);
            }
            for ref_id in element_ref_ids {
                collection.push_element_ref(ref_id);
            }
            for ref_id in source_ref_ids {
                collection.push_source_element_ref(ref_id);
            }
        }
        Ok(())
    }
}
