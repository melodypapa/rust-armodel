//! ARObject/Identifiable attribute emitters shared by every domain writer
//! (py writeARObject / writeIdentifiable attribute parts). Part of the
//! arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;

impl ARXMLWriter {
    /// py `writeARObject` — the `S`/`T` attributes.
    pub(super) fn write_ar_object_attributes(
        &self,
        element: &mut BytesStart<'_>,
        ar_object: &ARObject,
    ) {
        if let Some(checksum) = ar_object.get_checksum() {
            element.push_attribute(("S", checksum));
        }
        if let Some(timestamp) = ar_object.get_timestamp() {
            element.push_attribute(("T", timestamp));
        }
    }

    /// S/T/UUID attributes in py's historical order (S, T, UUID) — shared by
    /// `write_ar_package` and every per-class emitter.
    pub(super) fn write_identifiable_attributes(
        &self,
        element: &mut BytesStart<'_>,
        checksum: Option<&str>,
        timestamp: Option<&str>,
        uuid: Option<&str>,
    ) {
        if let Some(checksum) = checksum {
            element.push_attribute(("S", checksum));
        }
        if let Some(timestamp) = timestamp {
            element.push_attribute(("T", timestamp));
        }
        if let Some(uuid) = uuid {
            element.push_attribute(("UUID", uuid));
        }
    }

    /// py `writeIdentifiable` tail: LONG-NAME, DESC, CATEGORY, INTRODUCTION,
    /// ADMIN-DATA — in exactly that element order.
    pub(super) fn write_identifiable_parts<W: Write>(
        &self,
        writer: &mut Writer<W>,
        parts: IdentifiableParts<'_>,
        document: &Document,
    ) -> Result<(), WriteError> {
        if let Some(long_name) = parts
            .long_name
            .and_then(|id| document.multilanguage_long_names.get(id))
        {
            self.set_multi_long_name(writer, long_name, document)?;
        }
        if let Some(desc) = parts
            .desc
            .and_then(|id| document.multi_language_overview_paragraphs.get(id))
        {
            self.set_multi_language_overview_paragraph(writer, desc, document)?;
        }
        if let Some(category) = parts.category {
            write_text_element(
                writer,
                "CATEGORY",
                BytesStart::new("CATEGORY"),
                Some(category),
            )?;
        }
        if let Some(introduction) = parts
            .introduction
            .and_then(|id| document.documentation_blocks.get(id))
        {
            self.write_documentation_block(writer, "INTRODUCTION", introduction, document)?;
        }
        if let Some(admin_data) = parts.admin_data.and_then(|id| document.admin_datas.get(id)) {
            self.write_admin_data(writer, admin_data, document)?;
        }
        Ok(())
    }

    /// py `writeARElement` — the ARElement chain level: ARElement adds no own
    /// payload, so this emits the `writeIdentifiable` chain — attributes S, T,
    /// UUID (historical order), then children SHORT-NAME, LONG-NAME, DESC,
    /// CATEGORY, INTRODUCTION, ADMIN-DATA.
    pub(super) fn write_ar_element<W: Write>(
        &self,
        writer: &mut Writer<W>,
        element: BytesStart<'_>,
        attrs: IdentifiableAttrs<'_>,
        parts: IdentifiableParts<'_>,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = element;
        self.write_identifiable_attributes(
            &mut element,
            attrs.checksum,
            attrs.timestamp,
            attrs.uuid,
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = attrs.short_name {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_identifiable_parts(writer, parts, document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::msr::asam_hdo::admin_data::AdminData;

    /// py `writeARObject` — S (checksum) then T (timestamp), read from the
    /// getters; an empty element that carries attributes self-closes.
    /// (Relocated from PR #18's arxml_writer/mod.rs tests by the domain-split
    /// merge — see docs/plan/sync-todo/Group1.md ARObject row.)
    #[test]
    fn ar_object_s_t_attributes_emit_in_py_order() {
        let mut document = Document::new();
        document.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00052.xsd");

        let mut admin_data = AdminData::new();
        admin_data
            .set_checksum("c1")
            .set_timestamp("2024-11-26T21:25:17+08:00");
        let admin_data_id = document.admin_datas.insert(admin_data);
        document.set_admin_data(admin_data_id);

        let package_id = document.add_ar_package(None, "Pkg");
        document
            .ar_packages
            .get_mut(package_id)
            .unwrap()
            .set_checksum("c2")
            .set_timestamp("2024-11-27T08:00:00+08:00");

        let output = tempfile::NamedTempFile::new().unwrap();
        // validate = false: this is an emission-order test and must not
        // depend on schema compilation (R23-11 compiles fine since the shared
        // xml.xsd landed in schemas/R23-11/, but this test is not about that).
        ARXMLWriter::with_options(WriterOptions {
            unescape_entities: false,
            validate: false,
        })
        .save(output.path(), &document)
        .unwrap();
        let text = std::fs::read_to_string(output.path()).unwrap();

        assert!(
            text.contains("<ADMIN-DATA S=\"c1\" T=\"2024-11-26T21:25:17+08:00\"/>"),
            "{text}"
        );
        assert!(
            text.contains("<AR-PACKAGE S=\"c2\" T=\"2024-11-27T08:00:00+08:00\">"),
            "{text}"
        );
    }

    /// py `writeARElement` — the ARElement chain level emits py's
    /// `writeIdentifiable` chain: attributes S, T, UUID (historical order),
    /// then children SHORT-NAME, LONG-NAME, DESC, CATEGORY, INTRODUCTION,
    /// ADMIN-DATA. Absent chain parts emit nothing (empty-wrapper rule).
    #[test]
    fn ar_element_level_emits_identifiable_chain() {
        let mut document = Document::new();
        let mut admin_data = AdminData::new();
        admin_data.set_language("EN");
        let admin_data_id = document.admin_datas.insert(admin_data);

        let writer = ARXMLWriter::with_options(WriterOptions {
            unescape_entities: false,
            validate: false,
        });
        let mut buffer = Vec::new();
        let mut serializer = quick_xml::Writer::new(&mut buffer);
        writer
            .write_ar_element(
                &mut serializer,
                BytesStart::new("KEYWORD-SET"),
                IdentifiableAttrs {
                    checksum: Some("c1"),
                    timestamp: Some("t1"),
                    uuid: Some("u1"),
                    short_name: Some("Ks"),
                },
                IdentifiableParts {
                    long_name: None,
                    desc: None,
                    category: Some("doNotExport"),
                    introduction: None,
                    admin_data: Some(admin_data_id),
                    sw_data_def_props: None,
                },
                &document,
            )
            .unwrap();

        let text = String::from_utf8(buffer).unwrap();
        assert_eq!(
            text,
            "<KEYWORD-SET S=\"c1\" T=\"t1\" UUID=\"u1\">\
             <SHORT-NAME>Ks</SHORT-NAME>\
             <CATEGORY>doNotExport</CATEGORY>\
             <ADMIN-DATA><LANGUAGE>EN</LANGUAGE></ADMIN-DATA>"
        );
    }

    /// py `writeARElement` — the empty-wrapper case: chain parts left unset
    /// emit no wrapper elements at all.
    #[test]
    fn ar_element_level_omits_absent_chain_parts() {
        let document = Document::new();

        let writer = ARXMLWriter::with_options(WriterOptions {
            unescape_entities: false,
            validate: false,
        });
        let mut buffer = Vec::new();
        let mut serializer = quick_xml::Writer::new(&mut buffer);
        writer
            .write_ar_element(
                &mut serializer,
                BytesStart::new("KEYWORD-SET"),
                IdentifiableAttrs {
                    checksum: None,
                    timestamp: None,
                    uuid: None,
                    short_name: Some("Ks"),
                },
                IdentifiableParts {
                    long_name: None,
                    desc: None,
                    category: None,
                    introduction: None,
                    admin_data: None,
                    sw_data_def_props: None,
                },
                &document,
            )
            .unwrap();

        let text = String::from_utf8(buffer).unwrap();
        assert_eq!(text, "<KEYWORD-SET><SHORT-NAME>Ks</SHORT-NAME>");
    }
}
