//! Collection emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::generic_structure::general_template_classes::element_collection::{
    AutoCollectEnum, CollectionId,
};

impl ARXMLWriter {
    /// py `writeCollection`.
    pub(super) fn write_collection<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: CollectionId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(collection) = document.collections.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("COLLECTION");
        self.write_identifiable_attributes(
            &mut element,
            collection.get_checksum(),
            collection.get_timestamp(),
            collection.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;

        let short_name_element = BytesStart::new("SHORT-NAME");
        write_text_element(
            writer,
            "SHORT-NAME",
            short_name_element,
            collection.get_short_name(),
        )?;

        self.write_identifiable_parts(
            writer,
            IdentifiableParts {
                long_name: collection.get_long_name(),
                desc: collection.get_desc(),
                category: collection.get_category(),
                introduction: collection.get_introduction(),
                admin_data: collection.get_admin_data(),
                sw_data_def_props: None,
            },
            document,
        )?;

        // py writeCollection: AUTO-COLLECT, COLLECTION-SEMANTICS,
        // ELEMENT-ROLE, ELEMENT-REFS, SOURCE-ELEMENT-REFS.
        if let Some(auto_collect) = collection.get_auto_collect() {
            let token = match auto_collect {
                AutoCollectEnum::RefAll => "REF-ALL",
                AutoCollectEnum::RefNone => "REF-NONE",
                AutoCollectEnum::RefNonStandard => "REF-NON-STANDARD",
            };
            write_optional_text_element(writer, "AUTO-COLLECT", Some(token))?;
        }
        write_optional_text_element(
            writer,
            "COLLECTION-SEMANTICS",
            collection.get_collection_semantics(),
        )?;
        write_optional_text_element(writer, "ELEMENT-ROLE", collection.get_element_role())?;
        write_ref_type_list(
            writer,
            "ELEMENT-REFS",
            "ELEMENT-REF",
            collection.get_element_refs(),
            document,
        )?;
        write_ref_type_list(
            writer,
            "SOURCE-ELEMENT-REFS",
            "SOURCE-ELEMENT-REF",
            collection.get_source_element_refs(),
            document,
        )?;

        writer.write_event(Event::End(BytesEnd::new("COLLECTION")))?;
        Ok(())
    }
}
