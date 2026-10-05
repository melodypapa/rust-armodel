//! SW-BASE-TYPE readers (py readSwBaseType → readBaseTypeDirectDefinition).
//! Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::ByteOrderEnum;
use crate::m2::msr::asam_hdo::base_types::{BaseTypeDirectDefinition, SwBaseTypeId};

impl ARXMLReader {
    /// py `readSwBaseType`.
    pub(super) fn read_sw_base_type(
        &mut self,
        element: &Node,
        id: SwBaseTypeId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        // py readBaseTypeDirectDefinition(element, getBaseTypeDefinition()):
        // the definition exists once Task 0's extractor fix landed —
        // create it lazily only for models built without one.
        let definition_id = match document
            .sw_base_types
            .get(id)
            .and_then(|sw_base_type| sw_base_type.get_base_type_definition())
        {
            Some(definition_id) => definition_id,
            None => {
                let definition_id = document
                    .base_type_direct_definitions
                    .insert(BaseTypeDirectDefinition::new());
                if let Some(sw_base_type) = document.sw_base_types.get_mut(id) {
                    sw_base_type.set_base_type_definition(definition_id);
                }
                definition_id
            }
        };
        if let Some(sw_base_type) = document.sw_base_types.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                sw_base_type.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                sw_base_type.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                sw_base_type.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                sw_base_type.set_admin_data(admin_data);
            }
        }
        if let Some(definition) = document.base_type_direct_definitions.get_mut(definition_id) {
            self.read_base_type_direct_definition(element, definition)?;
        }
        Ok(())
    }

    /// py `readBaseTypeDirectDefinition`.
    fn read_base_type_direct_definition(
        &mut self,
        element: &Node,
        definition: &mut BaseTypeDirectDefinition,
    ) -> Result<(), ParseError> {
        if let Some(size) = get_child_element_string(element, "BASE-TYPE-SIZE") {
            definition.set_base_type_size(size);
        }
        if let Some(encoding) = get_child_element_string(element, "BASE-TYPE-ENCODING") {
            definition.set_base_type_encoding(encoding);
        }
        if let Some(alignment) = get_child_element_string(element, "MEM-ALIGNMENT") {
            definition.set_mem_alignment(alignment);
        }
        if let Some(order) = get_child_element_string(element, "BYTE-ORDER") {
            match ByteOrderEnum::try_from(order) {
                Ok(value) => {
                    definition.set_byte_order(value);
                }
                Err(_) => {
                    let message = format!("Unsupported BYTE-ORDER <{order}>");
                    self.not_implemented(message)?;
                }
            }
        }
        if let Some(native) = get_child_element_string(element, "NATIVE-DECLARATION") {
            definition.set_native_declaration(native);
        }
        Ok(())
    }
}
