//! SW-BASE-TYPE emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::base_types::SwBaseTypeId;

impl ARXMLWriter {
    /// py `writeSwBaseType`.
    pub(super) fn write_sw_base_type<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: SwBaseTypeId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(sw_base_type) = document.sw_base_types.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("SW-BASE-TYPE");
        self.write_identifiable_attributes(
            &mut element,
            sw_base_type.get_checksum(),
            sw_base_type.get_timestamp(),
            sw_base_type.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;

        let short_name_element = BytesStart::new("SHORT-NAME");
        write_text_element(
            writer,
            "SHORT-NAME",
            short_name_element,
            sw_base_type.get_short_name(),
        )?;

        self.write_identifiable_parts(
            writer,
            IdentifiableParts {
                long_name: sw_base_type.get_long_name(),
                desc: sw_base_type.get_desc(),
                category: sw_base_type.get_category(),
                introduction: sw_base_type.get_introduction(),
                admin_data: sw_base_type.get_admin_data(),
                sw_data_def_props: None,
            },
            document,
        )?;

        // py `setBaseTypeDirectDefinition`
        if let Some(definition) = sw_base_type
            .get_base_type_definition()
            .and_then(|definition_id| document.base_type_direct_definitions.get(definition_id))
        {
            write_optional_text_element(writer, "BASE-TYPE-SIZE", definition.get_base_type_size())?;
            write_optional_text_element(
                writer,
                "BASE-TYPE-ENCODING",
                definition.get_base_type_encoding(),
            )?;
            write_optional_text_element(writer, "MEM-ALIGNMENT", definition.get_mem_alignment())?;
            if let Some(byte_order) = definition.get_byte_order() {
                write_optional_text_element(writer, "BYTE-ORDER", Some(byte_order.as_str()))?;
            }
            write_optional_text_element(
                writer,
                "NATIVE-DECLARATION",
                definition.get_native_declaration(),
            )?;
        }

        writer.write_event(Event::End(BytesEnd::new("SW-BASE-TYPE")))?;
        Ok(())
    }
}
