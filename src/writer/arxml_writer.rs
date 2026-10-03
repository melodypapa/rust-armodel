//! `ARXMLWriter::save` emitting the XML declaration, `AUTOSAR` with
//! namespaces, `ADMIN-DATA`, `AR-PACKAGES` (P0 design §9 step 5).
//! Mirrors py's `arxml_writer.py`.

use std::io::Write;
use std::path::Path;

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::writer::Writer;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ElementRef;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::{
    ARPackage, ARPackageId,
};
use crate::m2::element_registry;
use crate::m2::msr::asam_hdo::admin_data::{AdminData, AdminDataId};
use crate::m2::msr::asam_hdo::base_types::SwBaseTypeId;
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdg};
use crate::m2::msr::documentation::text_model::block_elements::{
    DocumentationBlock, DocumentationBlockId,
};
use crate::m2::msr::documentation::text_model::multilanguage_data::{
    MultiLanguageOverviewParagraph, MultiLanguageOverviewParagraphId, MultilanguageLongName,
    MultilanguageLongNameId,
};
use crate::writer::abstract_arxml_writer::{
    write_optional_text_element, write_text_element, WriteError,
};

/// The per-class `Identifiable` payload pieces a family emitter hands to
/// `write_identifiable_parts` (ids resolve through the `Document` arenas).
struct IdentifiableParts<'a> {
    long_name: Option<MultilanguageLongNameId>,
    desc: Option<MultiLanguageOverviewParagraphId>,
    category: Option<&'a str>,
    introduction: Option<DocumentationBlockId>,
    admin_data: Option<AdminDataId>,
}

const DEFAULT_NAMESPACE: &str = "http://autosar.org/schema/r4.0";
const XSI_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";

/// py `ARXMLWriter(options)` — `unescape_entities` mirrors py's `patch_xml`
/// post-pass; `warning`/`version` exist in py's option table but do not
/// affect its serialization.
#[derive(Debug, Clone, Default)]
pub struct WriterOptions {
    pub unescape_entities: bool,
}

/// py `ARXMLWriter`
#[derive(Debug, Default)]
pub struct ARXMLWriter {
    options: WriterOptions,
}

impl ARXMLWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(options: WriterOptions) -> Self {
        Self { options }
    }

    /// py `save` — 2-space indentation; element order follows py
    /// (`ADMIN-DATA`, then `AR-PACKAGES`; FILE-INFO-COMMENT/INTRODUCTION are
    /// P1).
    pub fn save(&self, path: &Path, document: &Document) -> Result<(), WriteError> {
        // py saveToFile serializes to a string, runs patch_xml, then writes
        // the file; buffering here gives the post-pass the same reach.
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);

        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

        // py save(): namespace and schema location come from the document,
        // with the same fallback defaults.
        let schema_location = if document.get_schema_location().is_empty() {
            format!("{DEFAULT_NAMESPACE} AUTOSAR_4-0-3.xsd")
        } else {
            document.get_schema_location().to_string()
        };
        let namespace = schema_location
            .split(' ')
            .next()
            .unwrap_or(DEFAULT_NAMESPACE);

        let mut root = BytesStart::new("AUTOSAR");
        root.push_attribute(("xmlns", namespace));
        root.push_attribute(("xmlns:xsi", XSI_NAMESPACE));
        root.push_attribute(("xsi:schemaLocation", schema_location.as_str()));
        writer.write_event(Event::Start(root))?;

        if let Some(admin_data) = document.get_admin_data() {
            self.write_admin_data(&mut writer, admin_data, document)?;
        }

        self.write_ar_packages(&mut writer, document.get_ar_packages(), document)?;

        writer.write_event(Event::End(BytesEnd::new("AUTOSAR")))?;
        // py saveToFile's minidom toprettyxml terminates the document with a
        // final newline after the root element
        buffer.push(b'\n');

        // py patch_xml — the self-closing-tag expansion (`<tag/>` →
        // `<tag></tag>`) is a no-op by construction here: the indent writer
        // already expands empty elements, so only the entity unescape applies.
        if self.options.unescape_entities {
            let text = String::from_utf8(buffer).map_err(|_| {
                WriteError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "serialized ARXML is not UTF-8",
                ))
            })?;
            let text = text.replace("&quot;", "\"").replace("&apos;", "'");
            buffer = text.into_bytes();
        }

        std::fs::write(path, buffer)?;
        Ok(())
    }

    /// py `writeARObject` — the `S`/`T` attributes.
    fn write_ar_object_attributes(&self, element: &mut BytesStart<'_>, ar_object: &ARObject) {
        if let Some(checksum) = ar_object.get_checksum() {
            element.push_attribute(("S", checksum));
        }
        if let Some(timestamp) = ar_object.get_timestamp() {
            element.push_attribute(("T", timestamp));
        }
    }

    /// py `setAdminData`
    fn write_admin_data<W: Write>(
        &self,
        writer: &mut Writer<W>,
        admin_data: &AdminData,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("ADMIN-DATA");
        self.write_ar_object_attributes(&mut element, admin_data.base());
        writer.write_event(Event::Start(element))?;

        if let Some(language) = admin_data.get_language() {
            let language_element = BytesStart::new("LANGUAGE");
            write_text_element(writer, "LANGUAGE", language_element, Some(language))?;
        }

        // py setMultiLanguagePlainText
        if let Some(paragraph) = admin_data
            .get_used_languages()
            .and_then(|id| document.multi_language_plain_texts.get(id))
        {
            writer.write_event(Event::Start(BytesStart::new("USED-LANGUAGES")))?;
            for l10_id in paragraph.get_l10s() {
                if let Some(l10) = document.l_plain_texts.get(*l10_id) {
                    let mut l10_element = BytesStart::new("L-10");
                    if let Some(l) = l10.get_l() {
                        l10_element.push_attribute(("L", l));
                    }
                    if let Some(xml_space) = l10.get_xml_space() {
                        let xml_space = xml_space.to_string();
                        l10_element.push_attribute(("xml:space", xml_space.as_str()));
                    }
                    write_text_element(writer, "L-10", l10_element, l10.get_value())?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("USED-LANGUAGES")))?;
        }

        // py writeAdminDataSdgs
        let sdgs = admin_data.get_sdgs();
        if !sdgs.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("SDGS")))?;
            for sdg_id in sdgs {
                if let Some(sdg) = document.sdgs.get(*sdg_id) {
                    self.write_sdg(writer, sdg, document)?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("SDGS")))?;
        }

        // py writeAdminDataDocRevisions — DocRevision is a P0 placeholder and
        // `doc_revisions` is always empty here; emitted from P1.

        writer.write_event(Event::End(BytesEnd::new("ADMIN-DATA")))?;
        Ok(())
    }

    /// py `setSdg`
    fn write_sdg<W: Write>(
        &self,
        writer: &mut Writer<W>,
        sdg: &Sdg,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("SDG");
        self.write_ar_object_attributes(&mut element, sdg.base());
        if let Some(gid) = sdg.get_gid() {
            element.push_attribute(("GID", gid));
        }
        writer.write_event(Event::Start(element))?;

        // py writeSdgCaption
        if let Some(caption) = sdg
            .get_sdg_caption()
            .and_then(|id| document.sdg_captions.get(id))
        {
            writer.write_event(Event::Start(BytesStart::new("SDG-CAPTION")))?;
            if let Some(short_name) = caption.get_short_name() {
                let short_name_element = BytesStart::new("SHORT-NAME");
                write_text_element(writer, "SHORT-NAME", short_name_element, Some(short_name))?;
            }
            // caption.get_desc() → MultiLanguageOverviewParagraph is a P0
            // placeholder; nothing to emit yet.
            writer.write_event(Event::End(BytesEnd::new("SDG-CAPTION")))?;
        }

        // py writeSds / writeSdfs / nested setSdg
        if let Some(contents) = sdg
            .get_sdg_contents_type()
            .and_then(|id| document.sdg_contents.get(id))
        {
            for sd_id in contents.get_sd() {
                if let Some(sd) = document.sds.get(*sd_id) {
                    self.write_sd(writer, sd)?;
                }
            }
            for sdf_id in contents.get_sdf() {
                if let Some(sdf) = document.sdfs.get(*sdf_id) {
                    let mut sdf_element = BytesStart::new("SDF");
                    self.write_ar_object_attributes(&mut sdf_element, sdf.base());
                    if let Some(gid) = sdf.get_gid() {
                        sdf_element.push_attribute(("GID", gid));
                    }
                    write_text_element(writer, "SDF", sdf_element, sdf.get_value())?;
                }
            }
            for nested_id in contents.get_sdg() {
                if let Some(nested) = document.sdgs.get(*nested_id) {
                    self.write_sdg(writer, nested, document)?;
                }
            }
        }

        writer.write_event(Event::End(BytesEnd::new("SDG")))?;
        Ok(())
    }

    /// py `writeSds` per-SD body
    fn write_sd<W: Write>(&self, writer: &mut Writer<W>, sd: &Sd) -> Result<(), WriteError> {
        let mut element = BytesStart::new("SD");
        self.write_ar_object_attributes(&mut element, sd.base());
        if let Some(gid) = sd.get_gid() {
            element.push_attribute(("GID", gid));
        }
        if let Some(xml_space) = sd.get_xml_space() {
            let xml_space = xml_space.to_string();
            element.push_attribute(("xml:space", xml_space.as_str()));
        }
        write_text_element(writer, "SD", element, sd.get_value())?;
        Ok(())
    }

    /// py `writeARPackages`
    fn write_ar_packages<W: Write>(
        &self,
        writer: &mut Writer<W>,
        packages: &[ARPackageId],
        document: &Document,
    ) -> Result<(), WriteError> {
        if packages.is_empty() {
            return Ok(());
        }
        writer.write_event(Event::Start(BytesStart::new("AR-PACKAGES")))?;
        for package_id in packages {
            if let Some(package) = document.ar_packages.get(*package_id) {
                self.write_ar_package(writer, package, document)?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("AR-PACKAGES")))?;
        Ok(())
    }

    /// py `writeARPackage`
    fn write_ar_package<W: Write>(
        &self,
        writer: &mut Writer<W>,
        package: &ARPackage,
        document: &Document,
    ) -> Result<(), WriteError> {
        // py writeIdentifiable attribute order on the AR-PACKAGE tag: S, T, UUID.
        let mut element = BytesStart::new("AR-PACKAGE");
        if let Some(checksum) = package.get_checksum() {
            element.push_attribute(("S", checksum));
        }
        if let Some(timestamp) = package.get_timestamp() {
            element.push_attribute(("T", timestamp));
        }
        if let Some(uuid) = package.get_uuid() {
            element.push_attribute(("UUID", uuid));
        }
        writer.write_event(Event::Start(element))?;

        if let Some(short_name) = package.get_short_name() {
            let short_name_element = BytesStart::new("SHORT-NAME");
            write_text_element(writer, "SHORT-NAME", short_name_element, Some(short_name))?;
        }

        // py writeIdentifiable emission order after SHORT-NAME:
        // LONG-NAME, DESC, CATEGORY, INTRODUCTION, ADMIN-DATA.
        self.write_identifiable_parts(
            writer,
            // ARPackage forwards only a subset; hop to the bases that own
            // long_name (MultilanguageReferrable) and desc/introduction
            // (Identifiable).
            IdentifiableParts {
                long_name: package.base().base().base().get_long_name(),
                desc: package.base().base().get_desc(),
                category: package.get_category(),
                introduction: package.base().base().get_introduction(),
                admin_data: package.get_admin_data(),
            },
            document,
        )?;

        // py writeARPackageElements — one <TAG> per element carrying the
        // common Identifiable parts; per-class payload is the P2–P4 port.
        let elements = package.get_elements();
        if !elements.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("ELEMENTS")))?;
            for element_ref in elements {
                self.write_ar_package_element(writer, *element_ref, document)?;
            }
            writer.write_event(Event::End(BytesEnd::new("ELEMENTS")))?;
        }

        // py writeReferenceBases — nothing to emit until Task 5 ports it.

        // py writeARPackages (nested packages last)
        self.write_ar_packages(writer, package.get_ar_packages(), document)?;

        writer.write_event(Event::End(BytesEnd::new("AR-PACKAGE")))?;
        Ok(())
    }

    /// S/T/UUID attributes in py's historical order (S, T, UUID) — shared by
    /// `write_ar_package` and every per-class emitter.
    fn write_identifiable_attributes(
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
    fn write_identifiable_parts<W: Write>(
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

    /// py `setMultiLongName` / `setLLongName` — L attr, text, then S/T
    /// (writeMixedContentForLongName runs after the text is set).
    fn set_multi_long_name<W: Write>(
        &self,
        writer: &mut Writer<W>,
        long_name: &MultilanguageLongName,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("LONG-NAME");
        self.write_ar_object_attributes(&mut element, long_name.base());
        writer.write_event(Event::Start(element))?;
        for l4_id in long_name.get_l4() {
            if let Some(l4) = document.l_long_names.get(*l4_id) {
                let mut l4_element = BytesStart::new("L-4");
                if let Some(l) = l4.get_l() {
                    l4_element.push_attribute(("L", l));
                }
                if let Some(sup) = l4.get_sup() {
                    l4_element.push_attribute(("SUP", sup));
                }
                if let Some(sub) = l4.get_sub() {
                    l4_element.push_attribute(("SUB", sub));
                }
                self.write_ar_object_attributes(&mut l4_element, l4.base().base());
                write_text_element(writer, "L-4", l4_element, l4.get_value())?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("LONG-NAME")))?;
        Ok(())
    }

    /// py `setMultiLanguageOverviewParagraph` / `setLOverviewParagraph` —
    /// S/T then L on each L-2 (setLanguageSpecific writes ARObject first).
    fn set_multi_language_overview_paragraph<W: Write>(
        &self,
        writer: &mut Writer<W>,
        paragraph: &MultiLanguageOverviewParagraph,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("DESC");
        self.write_ar_object_attributes(&mut element, paragraph.base());
        writer.write_event(Event::Start(element))?;
        for l2_id in paragraph.get_l2() {
            if let Some(l2) = document.l_overview_paragraphs.get(*l2_id) {
                let mut l2_element = BytesStart::new("L-2");
                self.write_ar_object_attributes(&mut l2_element, l2.base().base());
                if let Some(l) = l2.get_l() {
                    l2_element.push_attribute(("L", l));
                }
                if let Some(blueprint) = l2.get_blueprint_value() {
                    l2_element.push_attribute(("BLUEPRINT-VALUE", blueprint));
                }
                write_text_element(writer, "L-2", l2_element, l2.get_value())?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("DESC")))?;
        Ok(())
    }

    /// py `writeDocumentationBlock` / `writeDocumentationBlockContent` —
    /// P paragraphs and LISTs; the other block kinds do not occur in the
    /// pinned fixtures (tracked on the port checklist).
    fn write_documentation_block<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        block: &DocumentationBlock,
        document: &Document,
    ) -> Result<(), WriteError> {
        writer.write_event(Event::Start(BytesStart::new(key)))?;
        self.write_documentation_block_content(writer, block, document)?;
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    fn write_documentation_block_content<W: Write>(
        &self,
        writer: &mut Writer<W>,
        block: &DocumentationBlock,
        document: &Document,
    ) -> Result<(), WriteError> {
        // py setMultiLanguageParagraphs(element, "P", …)
        for paragraph_id in block.get_ps() {
            if let Some(paragraph) = document.multi_language_paragraphs.get(*paragraph_id) {
                let mut p_element = BytesStart::new("P");
                if let Some(help) = paragraph.get_help_entry() {
                    p_element.push_attribute(("HELP-ENTRY", help));
                }
                self.write_paginateable_attrs(&mut p_element, paragraph.base());
                writer.write_event(Event::Start(p_element))?;
                // py writeLParagraphs — S/T, text, then L (attributes in
                // py's dict-insertion order; the text is the element text).
                for l1_id in paragraph.get_l1() {
                    if let Some(l1) = document.l_paragraphs.get(*l1_id) {
                        let mut l1_element = BytesStart::new("L-1");
                        self.write_ar_object_attributes(&mut l1_element, l1.base().base());
                        if let Some(l) = l1.get_l() {
                            l1_element.push_attribute(("L", l));
                        }
                        write_text_element(writer, "L-1", l1_element, l1.get_value())?;
                    }
                }
                writer.write_event(Event::End(BytesEnd::new("P")))?;
            }
        }
        // py setListElement(element, "LIST", …)
        for list_id in block.get_lists() {
            if let Some(list) = document.ar_lists.get(*list_id) {
                let mut list_element = BytesStart::new("LIST");
                self.write_paginateable_attrs(&mut list_element, list.base());
                if let Some(list_type) = list.get_type() {
                    // py: type.getValue().upper()
                    let value = list_type.as_str().to_uppercase();
                    list_element.push_attribute(("TYPE", value.as_str()));
                }
                writer.write_event(Event::Start(list_element))?;
                for item_id in list.get_items() {
                    if let Some(item) = document.items.get(*item_id) {
                        let mut item_element = BytesStart::new("ITEM");
                        self.write_paginateable_attrs(&mut item_element, item.base());
                        writer.write_event(Event::Start(item_element))?;
                        if let Some(contents) = item
                            .get_item_contents()
                            .and_then(|id| document.documentation_blocks.get(id))
                        {
                            self.write_documentation_block_content(writer, contents, document)?;
                        }
                        writer.write_event(Event::End(BytesEnd::new("ITEM")))?;
                    }
                }
                writer.write_event(Event::End(BytesEnd::new("LIST")))?;
            }
        }
        Ok(())
    }

    /// py `writePaginateable` — BREAK / KEEP-WITH-PREVIOUS attributes.
    fn write_paginateable_attrs(
        &self,
        element: &mut BytesStart<'_>,
        paginateable: &crate::m2::msr::documentation::block_elements::pagination_and_view::Paginateable,
    ) {
        if let Some(chapter_break) = paginateable.get_chapter_break() {
            element.push_attribute(("BREAK", chapter_break.as_str()));
        }
        if let Some(keep) = paginateable.get_keep_with_previous() {
            element.push_attribute(("KEEP-WITH-PREVIOUS", keep.as_str()));
        }
    }

    /// py `writeARPackageElement`'s isinstance chain. Grows one arm per
    /// ported family; the wildcard keeps unported families on the P0 shape
    /// (common Identifiable parts only).
    fn write_ar_package_element<W: Write>(
        &self,
        writer: &mut Writer<W>,
        element_ref: ElementRef,
        document: &Document,
    ) -> Result<(), WriteError> {
        if let ElementRef::SwBaseType(id) = element_ref {
            return self.write_sw_base_type(writer, id, document);
        }
        let tag = element_registry::element_tag(&element_ref);
        let mut element = BytesStart::new(tag);
        self.write_identifiable_attributes(
            &mut element,
            element_registry::element_checksum(document, &element_ref),
            element_registry::element_timestamp(document, &element_ref),
            element_registry::element_uuid(document, &element_ref),
        );
        writer.write_event(Event::Start(element))?;
        let short_name = element_registry::element_short_name(document, &element_ref);
        let short_name_element = BytesStart::new("SHORT-NAME");
        write_text_element(writer, "SHORT-NAME", short_name_element, short_name)?;
        if let Some(category) = element_registry::element_category(document, &element_ref) {
            let category_element = BytesStart::new("CATEGORY");
            write_text_element(writer, "CATEGORY", category_element, Some(category))?;
        }
        // LongName/Desc/Introduction/AdminData per family land with Tasks 4-9;
        // the registry's category setters own everything the P0 model tracks.
        writer.write_event(Event::End(BytesEnd::new(tag)))?;
        Ok(())
    }

    /// py `writeSwBaseType`.
    fn write_sw_base_type<W: Write>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::msr::asam_hdo::special_data::SdgContents;
    use crate::m2::msr::documentation::text_model::language_data_model::{LPlainText, XmlSpace};
    use crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguagePlainText;

    /// Builds the fixture model through the Document API and serializes it.
    fn fixture_document() -> Document {
        let mut document = Document::new();
        document.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd");
        document.set_ar_release("R21-11");

        let mut l10 = LPlainText::new();
        l10.set_l("EN")
            .set_xml_space(XmlSpace::Preserve)
            .set_value("English");
        let l10_id = document.l_plain_texts.insert(l10);

        let mut paragraph = MultiLanguagePlainText::new();
        paragraph.push_l10(l10_id);
        let paragraph_id = document.multi_language_plain_texts.insert(paragraph);

        let mut sd = Sd::new();
        sd.set_gid("purpose")
            .set_xml_space(XmlSpace::Preserve)
            .set_value("special   data");
        let sd_id = document.sds.insert(sd);

        let mut contents = SdgContents::new();
        contents.push_sd(sd_id);
        let contents_id = document.sdg_contents.insert(contents);

        let mut sdg = Sdg::new();
        sdg.set_gid("demo").set_sdg_contents_type(contents_id);
        let sdg_id = document.sdgs.insert(sdg);

        let mut admin_data = AdminData::new();
        admin_data.set_used_languages(paragraph_id);
        admin_data.push_sdg(sdg_id);
        let admin_data_id = document.admin_datas.insert(admin_data);
        document.set_admin_data(admin_data_id);

        document.add_ar_package(None, "WhitespaceDemo");
        document
    }

    #[test]
    fn save_emits_namespaced_root_and_preserved_whitespace() {
        let document = fixture_document();
        let output = tempfile::NamedTempFile::new().unwrap();
        ARXMLWriter::new().save(output.path(), &document).unwrap();

        let text = std::fs::read_to_string(output.path()).unwrap();

        assert!(text.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(text.contains(
            "<AUTOSAR xmlns=\"http://autosar.org/schema/r4.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd\">"
        ));
        assert!(text.contains("<L-10 L=\"EN\" xml:space=\"preserve\">English</L-10>"));
        assert!(text.contains("<SD GID=\"purpose\" xml:space=\"preserve\">special   data</SD>"));
        assert!(text.contains("<SHORT-NAME>WhitespaceDemo</SHORT-NAME>"));
    }

    /// py: ElementTree escapes quotes in attribute values and `--unescape-entities`
    /// (patch_xml) unescapes exactly `&quot;`/`&apos;` in the final text.
    #[test]
    fn unescape_entities_option_unescapes_attribute_quotes() {
        let mut document = Document::new();
        document.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd");
        let package_id = document.add_ar_package(None, "Pkg");
        document
            .ar_packages
            .get_mut(package_id)
            .unwrap()
            .set_uuid("a\"b'c");

        let escaped = tempfile::NamedTempFile::new().unwrap();
        ARXMLWriter::new().save(escaped.path(), &document).unwrap();
        let text = std::fs::read_to_string(escaped.path()).unwrap();
        assert!(text.contains("UUID=\"a&quot;b&apos;c\""), "{text}");

        let unescaped = tempfile::NamedTempFile::new().unwrap();
        ARXMLWriter::with_options(WriterOptions {
            unescape_entities: true,
        })
        .save(unescaped.path(), &document)
        .unwrap();
        let text = std::fs::read_to_string(unescaped.path()).unwrap();
        assert!(text.contains("UUID=\"a\"b'c\""), "{text}");
    }
}
