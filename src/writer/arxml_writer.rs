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
use crate::m2::autosar_templates::generic_structure::general_template_classes::element_collection::{
    AutoCollectEnum, CollectionId,
};
use crate::m2::autosar_templates::generic_structure::life_cycles::{
    LifeCycleInfoId, LifeCycleInfoSetId, LifeCyclePeriod,
};
use crate::m2::element_registry;
use crate::m2::msr::asam_hdo::admin_data::{AdminData, AdminDataId};
use crate::m2::msr::asam_hdo::constraints::global_constraints::{
    DataConstrId, DataConstrRule, InternalConstrs, PhysConstrs, ScaleConstr,
};
use crate::m2::msr::asam_hdo::computation_method::{
    Compu, CompuConst, CompuConstContentRef, CompuMethodId, CompuScaleContentsRef,
};
use crate::m2::msr::asam_hdo::base_types::SwBaseTypeId;
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdg};
use crate::m2::msr::asam_hdo::units::{PhysicalDimensionId, UnitId};
use crate::m2::msr::documentation::text_model::block_elements::{
    DocumentationBlock, DocumentationBlockId,
};
use crate::m2::msr::documentation::text_model::multilanguage_data::{
    MultiLanguageOverviewParagraph, MultiLanguageOverviewParagraphId, MultilanguageLongName,
    MultilanguageLongNameId,
};
use crate::writer::abstract_arxml_writer::{
    WriteError,
    write_limit_element,
    write_optional_boolean_element,
    write_optional_ref_type,
    write_optional_text_element,
    write_ref_type_list,
    write_text_element,
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

        // py writeReferenceBases — REFERENCE-BASES comes before ELEMENTS.
        let reference_bases = package.get_reference_bases();
        if !reference_bases.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("REFERENCE-BASES")))?;
            for base_id in reference_bases {
                if let Some(base) = document.reference_bases.get(*base_id) {
                    writer.write_event(Event::Start(BytesStart::new("REFERENCE-BASE")))?;
                    write_optional_text_element(writer, "SHORT-LABEL", base.get_short_label())?;
                    write_optional_boolean_element(writer, "IS-DEFAULT", base.get_is_default())?;
                    write_optional_boolean_element(writer, "IS-GLOBAL", base.get_is_global())?;
                    write_optional_boolean_element(
                        writer,
                        "BASE-IS-THIS-PACKAGE",
                        base.get_base_is_this_package(),
                    )?;
                    write_ref_type_list(
                        writer,
                        "GLOBAL-IN-PACKAGE-REFS",
                        "GLOBAL-IN-PACKAGE-REF",
                        base.get_global_in_package_refs(),
                        document,
                    )?;
                    let global_elements = base.get_global_elements();
                    if !global_elements.is_empty() {
                        writer.write_event(Event::Start(BytesStart::new("GLOBAL-ELEMENTS")))?;
                        for element_text in global_elements {
                            write_text_element(
                                writer,
                                "GLOBAL-ELEMENT",
                                BytesStart::new("GLOBAL-ELEMENT"),
                                Some(element_text),
                            )?;
                        }
                        writer.write_event(Event::End(BytesEnd::new("GLOBAL-ELEMENTS")))?;
                    }
                    write_optional_ref_type(
                        writer,
                        "PACKAGE-REF",
                        base.get_package_ref()
                            .and_then(|ref_id| document.ref_types.get(ref_id)),
                    )?;
                    writer.write_event(Event::End(BytesEnd::new("REFERENCE-BASE")))?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("REFERENCE-BASES")))?;
        }

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
        if long_name.get_l4().is_empty() {
            write_text_element(writer, "LONG-NAME", element, None)?;
            return Ok(());
        }
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
        // py/minidom renders a childless wrapper expanded-inline
        // (`<DESC></DESC>`) — which is exactly write_text_element's
        // empty-without-attributes form.
        if paragraph.get_l2().is_empty() {
            write_text_element(writer, "DESC", element, None)?;
            return Ok(());
        }
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

    /// py `writeCompuMethod`.
    fn write_compu_method<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: CompuMethodId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(compu_method) = document.compu_methods.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("COMPU-METHOD");
        self.write_identifiable_attributes(
            &mut element,
            compu_method.get_checksum(),
            compu_method.get_timestamp(),
            compu_method.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = compu_method.get_short_name() {
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
                long_name: compu_method.get_long_name(),
                desc: compu_method.get_desc(),
                category: compu_method.get_category(),
                introduction: compu_method.get_introduction(),
                admin_data: compu_method.get_admin_data(),
            },
            document,
        )?;
        write_optional_text_element(writer, "DISPLAY-FORMAT", compu_method.get_display_format())?;
        write_optional_ref_type(
            writer,
            "UNIT-REF",
            compu_method
                .get_unit_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        if let Some(internal) = compu_method
            .get_compu_internal_to_phys()
            .and_then(|c| document.compus.get(c))
        {
            self.set_compu(writer, "COMPU-INTERNAL-TO-PHYS", internal, document)?;
        }
        if let Some(phys) = compu_method
            .get_compu_phys_to_internal()
            .and_then(|c| document.compus.get(c))
        {
            self.set_compu(writer, "COMPU-PHYS-TO-INTERNAL", phys, document)?;
        }
        writer.write_event(Event::End(BytesEnd::new("COMPU-METHOD")))?;
        Ok(())
    }

    /// py `setCompu` — S/T attrs, COMPU-SCALES, COMPU-DEFAULT-VALUE.
    fn set_compu<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        compu: &Compu,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new(key);
        self.write_ar_object_attributes(&mut element, compu.base());
        writer.write_event(Event::Start(element))?;
        if let Some(crate::m2::msr::asam_hdo::computation_method::CompuContentRef::CompuScales(
            scales_id,
        )) = compu.get_compu_content()
        {
            if let Some(scales) = document.compu_scales_arena.get(scales_id) {
                self.set_compu_scales(writer, scales, document)?;
            }
        }
        if let Some(default) = compu
            .get_compu_default_value()
            .and_then(|c| document.compu_consts.get(c))
        {
            self.set_compu_const(writer, "COMPU-DEFAULT-VALUE", default, document)?;
        }
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    /// py `setCompuScales` — wrapper (no attrs) + per-scale bodies.
    fn set_compu_scales<W: Write>(
        &self,
        writer: &mut Writer<W>,
        scales: &crate::m2::msr::asam_hdo::computation_method::CompuScales,
        document: &Document,
    ) -> Result<(), WriteError> {
        writer.write_event(Event::Start(BytesStart::new("COMPU-SCALES")))?;
        for scale_id in scales.get_compu_scales() {
            if let Some(scale) = document.compu_scales.get(*scale_id) {
                self.write_compu_scale(writer, "COMPU-SCALE", scale, document)?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("COMPU-SCALES")))?;
        Ok(())
    }

    /// py `writeCompuScale` — fixed child order per the py body.
    fn write_compu_scale<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        scale: &crate::m2::msr::asam_hdo::computation_method::CompuScale,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new(key);
        self.write_ar_object_attributes(&mut element, scale.base());
        writer.write_event(Event::Start(element))?;
        write_optional_text_element(writer, "A2L-DISPLAY-TEXT", scale.get_a2l_display_text())?;
        if let Some(inverse) = scale
            .get_compu_inverse_value()
            .and_then(|c| document.compu_consts.get(c))
        {
            self.set_compu_const(writer, "COMPU-INVERSE-VALUE", inverse, document)?;
        }
        write_optional_text_element(writer, "SHORT-LABEL", scale.get_short_label())?;
        write_optional_text_element(writer, "SYMBOL", scale.get_symbol())?;
        if let Some(desc) = scale
            .get_desc()
            .and_then(|id| document.multi_language_overview_paragraphs.get(id))
        {
            self.set_multi_language_overview_paragraph(writer, desc, document)?;
        }
        write_optional_text_element(writer, "MASK", scale.get_mask())?;
        if let Some(limit) = scale.get_lower_limit().and_then(|l| document.limits.get(l)) {
            write_limit_element(writer, "LOWER-LIMIT", limit)?;
        }
        if let Some(limit) = scale.get_upper_limit().and_then(|l| document.limits.get(l)) {
            write_limit_element(writer, "UPPER-LIMIT", limit)?;
        }
        match scale.get_compu_scale_contents() {
            Some(CompuScaleContentsRef::CompuScaleConstantContents(id)) => {
                if let Some(constant) = document.compu_scale_constant_contents.get(id) {
                    // py writeCompuScaleConstantContents: bare COMPU-CONST +
                    // VT literal child (no ARObject attrs on either).
                    writer.write_event(Event::Start(BytesStart::new("COMPU-CONST")))?;
                    if let Some(const_id) = constant.get_compu_const() {
                        if let Some(const_) = document.compu_consts.get(const_id) {
                            if let Some(CompuConstContentRef::CompuConstTextContent(text_id)) =
                                const_.get_compu_const_content_type()
                            {
                                if let Some(text) = document.compu_const_text_contents.get(text_id)
                                {
                                    write_optional_text_element(writer, "VT", text.get_vt())?;
                                }
                            }
                        }
                    }
                    writer.write_event(Event::End(BytesEnd::new("COMPU-CONST")))?;
                }
            }
            Some(CompuScaleContentsRef::CompuScaleRationalFormula(id)) => {
                if let Some(formula) = document.compu_scale_rational_formulas.get(id) {
                    writer.write_event(Event::Start(BytesStart::new("COMPU-RATIONAL-COEFFS")))?;
                    if let Some(coeffs) = formula
                        .get_compu_rational_coeffs()
                        .and_then(|c| document.compu_rational_coeffs.get(c))
                    {
                        if let Some(numerator) = coeffs
                            .get_compu_numerator()
                            .and_then(|n| document.compu_nominator_denominators.get(n))
                        {
                            self.write_nominator_denominator(writer, "COMPU-NUMERATOR", numerator)?;
                        }
                        if let Some(denominator) = coeffs
                            .get_compu_denominator()
                            .and_then(|d| document.compu_nominator_denominators.get(d))
                        {
                            self.write_nominator_denominator(
                                writer,
                                "COMPU-DENOMINATOR",
                                denominator,
                            )?;
                        }
                    }
                    writer.write_event(Event::End(BytesEnd::new("COMPU-RATIONAL-COEFFS")))?;
                }
            }
            None => {}
        }
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    /// py `writeCompuNominatorDenominator` — V children in list order.
    fn write_nominator_denominator<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        nom_denom: &crate::m2::msr::asam_hdo::computation_method::CompuNominatorDenominator,
    ) -> Result<(), WriteError> {
        writer.write_event(Event::Start(BytesStart::new(key)))?;
        for v in nom_denom.get_v() {
            write_text_element(writer, "V", BytesStart::new("V"), Some(v))?;
        }
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    /// py `setCompuConst` — S/T attrs + content child (VT/V/VF).
    fn set_compu_const<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        compu_const: &CompuConst,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new(key);
        self.write_ar_object_attributes(&mut element, compu_const.base());
        writer.write_event(Event::Start(element))?;
        match compu_const.get_compu_const_content_type() {
            Some(CompuConstContentRef::CompuConstTextContent(id)) => {
                if let Some(text) = document.compu_const_text_contents.get(id) {
                    write_optional_text_element(writer, "VT", text.get_vt())?;
                }
            }
            Some(CompuConstContentRef::CompuConstNumericContent(id)) => {
                if let Some(numeric) = document.compu_const_numeric_contents.get(id) {
                    write_optional_text_element(writer, "V", numeric.get_v())?;
                }
            }
            Some(CompuConstContentRef::CompuConstFormulaContent(id)) => {
                if let Some(formula) = document.compu_const_formula_contents.get(id) {
                    write_optional_text_element(writer, "VF", formula.get_vf())?;
                }
            }
            None => {}
        }
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    /// py `writeDataConstr`.
    fn write_data_constr<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: DataConstrId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(data_constr) = document.data_constrs.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("DATA-CONSTR");
        self.write_identifiable_attributes(
            &mut element,
            data_constr.get_checksum(),
            data_constr.get_timestamp(),
            data_constr.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = data_constr.get_short_name() {
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
                long_name: data_constr.get_long_name(),
                desc: data_constr.get_desc(),
                category: data_constr.get_category(),
                introduction: data_constr.get_introduction(),
                admin_data: data_constr.get_admin_data(),
            },
            document,
        )?;
        // py writeDataConstrRules — wrapper only when non-empty
        let rules = data_constr.get_data_constr_rule();
        if !rules.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("DATA-CONSTR-RULES")))?;
            for rule_id in rules {
                if let Some(rule) = document.data_constr_rules.get(*rule_id) {
                    self.write_data_constr_rule(writer, rule, document)?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("DATA-CONSTR-RULES")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("DATA-CONSTR")))?;
        Ok(())
    }

    /// py `writeDataConstrRules` per-rule body — CONSTR-LEVEL, then
    /// PHYS-CONSTRS, then INTERNAL-CONSTRS (writer order differs from the
    /// reader's).
    fn write_data_constr_rule<W: Write>(
        &self,
        writer: &mut Writer<W>,
        rule: &DataConstrRule,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("DATA-CONSTR-RULE");
        self.write_ar_object_attributes(&mut element, rule.base());
        writer.write_event(Event::Start(element))?;
        write_optional_text_element(writer, "CONSTR-LEVEL", rule.get_constr_level())?;
        if let Some(constrs) = rule
            .get_phys_constrs()
            .and_then(|c| document.phys_constrs.get(c))
        {
            self.write_phys_constrs(writer, constrs, document)?;
        }
        if let Some(constrs) = rule
            .get_internal_constrs()
            .and_then(|c| document.internal_constrs.get(c))
        {
            self.write_internal_constrs(writer, constrs, document)?;
        }
        writer.write_event(Event::End(BytesEnd::new("DATA-CONSTR-RULE")))?;
        Ok(())
    }

    /// py `setInternalConstrs`.
    fn write_internal_constrs<W: Write>(
        &self,
        writer: &mut Writer<W>,
        constrs: &InternalConstrs,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("INTERNAL-CONSTRS");
        self.write_ar_object_attributes(&mut element, constrs.base());
        writer.write_event(Event::Start(element))?;
        if let Some(limit) = constrs
            .get_lower_limit()
            .and_then(|l| document.limits.get(l))
        {
            write_limit_element(writer, "LOWER-LIMIT", limit)?;
        }
        if let Some(limit) = constrs
            .get_upper_limit()
            .and_then(|l| document.limits.get(l))
        {
            write_limit_element(writer, "UPPER-LIMIT", limit)?;
        }
        self.write_scale_constrs(
            writer,
            "SCALE-CONSTRS",
            &constrs.get_scale_constrs(),
            document,
        )?;
        write_optional_text_element(writer, "MAX-GRADIENT", constrs.get_max_gradient())?;
        write_optional_text_element(writer, "MAX-DIFF", constrs.get_max_diff())?;
        if let Some(monotony) = constrs.get_monotony() {
            write_text_element(
                writer,
                "MONOTONY",
                BytesStart::new("MONOTONY"),
                Some(monotony.as_str()),
            )?;
        }
        writer.write_event(Event::End(BytesEnd::new("INTERNAL-CONSTRS")))?;
        Ok(())
    }

    /// py `setPhysConstrs`.
    fn write_phys_constrs<W: Write>(
        &self,
        writer: &mut Writer<W>,
        constrs: &PhysConstrs,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("PHYS-CONSTRS");
        self.write_ar_object_attributes(&mut element, constrs.base());
        writer.write_event(Event::Start(element))?;
        if let Some(limit) = constrs
            .get_lower_limit()
            .and_then(|l| document.limits.get(l))
        {
            write_limit_element(writer, "LOWER-LIMIT", limit)?;
        }
        if let Some(limit) = constrs
            .get_upper_limit()
            .and_then(|l| document.limits.get(l))
        {
            write_limit_element(writer, "UPPER-LIMIT", limit)?;
        }
        self.write_scale_constrs(
            writer,
            "SCALE-CONSTRS",
            &constrs.get_scale_constrs(),
            document,
        )?;
        write_optional_text_element(writer, "MAX-GRADIENT", constrs.get_max_gradient())?;
        write_optional_text_element(writer, "MAX-DIFF", constrs.get_max_diff())?;
        if let Some(monotony) = constrs.get_monotony() {
            write_text_element(
                writer,
                "MONOTONY",
                BytesStart::new("MONOTONY"),
                Some(monotony.as_str()),
            )?;
        }
        write_optional_ref_type(
            writer,
            "UNIT-REF",
            constrs
                .get_unit_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        writer.write_event(Event::End(BytesEnd::new("PHYS-CONSTRS")))?;
        Ok(())
    }

    /// py's SCALE-CONSTRS wrapper + `setScaleConstr` per item.
    fn write_scale_constrs<W: Write>(
        &self,
        writer: &mut Writer<W>,
        wrapper: &str,
        scale_constrs: &[crate::m2::msr::asam_hdo::constraints::global_constraints::ScaleConstrId],
        document: &Document,
    ) -> Result<(), WriteError> {
        if scale_constrs.is_empty() {
            return Ok(());
        }
        writer.write_event(Event::Start(BytesStart::new(wrapper)))?;
        for sc_id in scale_constrs {
            if let Some(scale_constr) = document.scale_constrs.get(*sc_id) {
                let mut element = BytesStart::new("SCALE-CONSTR");
                self.write_ar_object_attributes(&mut element, scale_constr.base());
                if let Some(validity) = scale_constr.get_validity() {
                    element.push_attribute(("VALIDITY", validity.as_str()));
                }
                writer.write_event(Event::Start(element))?;
                write_optional_text_element(writer, "SHORT-LABEL", scale_constr.get_short_label())?;
                if let Some(desc) = scale_constr
                    .get_desc()
                    .and_then(|id| document.multi_language_overview_paragraphs.get(id))
                {
                    self.set_multi_language_overview_paragraph(writer, desc, document)?;
                }
                if let Some(limit) = scale_constr
                    .get_lower_limit()
                    .and_then(|l| document.limits.get(l))
                {
                    write_limit_element(writer, "LOWER-LIMIT", limit)?;
                }
                if let Some(limit) = scale_constr
                    .get_upper_limit()
                    .and_then(|l| document.limits.get(l))
                {
                    write_limit_element(writer, "UPPER-LIMIT", limit)?;
                }
                writer.write_event(Event::End(BytesEnd::new("SCALE-CONSTR")))?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new(wrapper)))?;
        Ok(())
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
        match element_ref {
            ElementRef::CompuMethod(id) => {
                return self.write_compu_method(writer, id, document);
            }
            ElementRef::DataConstr(id) => {
                return self.write_data_constr(writer, id, document);
            }
            ElementRef::SwBaseType(id) => return self.write_sw_base_type(writer, id, document),
            ElementRef::Collection(id) => return self.write_collection(writer, id, document),
            ElementRef::LifeCycleInfoSet(id) => {
                return self.write_life_cycle_info_set(writer, id, document)
            }
            ElementRef::PhysicalDimension(id) => {
                return self.write_physical_dimension(writer, id, document)
            }
            ElementRef::Unit(id) => return self.write_unit(writer, id, document),
            _ => {}
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

    /// py `writeCollection`.
    fn write_collection<W: Write>(
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

    /// py `setLifeCyclePeriod` — py does not run `writeARObject` here, so
    /// only the three value children are emitted.
    fn write_life_cycle_period<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        period: &LifeCyclePeriod,
    ) -> Result<(), WriteError> {
        writer.write_event(Event::Start(BytesStart::new(key)))?;
        write_optional_text_element(writer, "DATE", period.get_date())?;
        write_optional_text_element(
            writer,
            "AR-RELEASE-VERSION",
            period.get_ar_release_version(),
        )?;
        write_optional_text_element(writer, "PRODUCT-RELEASE", period.get_product_release())?;
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    /// py `writeLifeCycleInfo` — one `LIFE-CYCLE-INFO` child of the
    /// `LIFE-CYCLE-INFOS` wrapper.
    fn write_life_cycle_info<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: LifeCycleInfoId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(info) = document.life_cycle_infos.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("LIFE-CYCLE-INFO");
        self.write_ar_object_attributes(&mut element, info.base());
        writer.write_event(Event::Start(element))?;

        write_optional_ref_type(
            writer,
            "LC-OBJECT-REF",
            info.get_lc_object_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        write_optional_ref_type(
            writer,
            "LC-STATE-REF",
            info.get_lc_state_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        if let Some(period_id) = info.get_period_begin() {
            if let Some(period) = document.life_cycle_periods.get(period_id) {
                self.write_life_cycle_period(writer, "PERIOD-BEGIN", period)?;
            }
        }
        if let Some(period_id) = info.get_period_end() {
            if let Some(period) = document.life_cycle_periods.get(period_id) {
                self.write_life_cycle_period(writer, "PERIOD-END", period)?;
            }
        }
        if let Some(remark) = info
            .get_remark()
            .and_then(|remark_id| document.documentation_blocks.get(remark_id))
        {
            self.write_documentation_block(writer, "REMARK", remark, document)?;
        }
        write_ref_type_list(
            writer,
            "USE-INSTEAD-REFS",
            "USE-INSTEAD-REF",
            info.get_use_instead_refs(),
            document,
        )?;

        writer.write_event(Event::End(BytesEnd::new("LIFE-CYCLE-INFO")))?;
        Ok(())
    }

    /// py `writeLifeCycleInfoSet`.
    fn write_life_cycle_info_set<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: LifeCycleInfoSetId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(info_set) = document.life_cycle_info_sets.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("LIFE-CYCLE-INFO-SET");
        self.write_identifiable_attributes(
            &mut element,
            info_set.get_checksum(),
            info_set.get_timestamp(),
            info_set.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;

        let short_name_element = BytesStart::new("SHORT-NAME");
        write_text_element(
            writer,
            "SHORT-NAME",
            short_name_element,
            info_set.get_short_name(),
        )?;

        self.write_identifiable_parts(
            writer,
            IdentifiableParts {
                long_name: info_set.get_long_name(),
                desc: info_set.get_desc(),
                category: info_set.get_category(),
                introduction: info_set.get_introduction(),
                admin_data: info_set.get_admin_data(),
            },
            document,
        )?;

        write_optional_ref_type(
            writer,
            "DEFAULT-LC-STATE-REF",
            info_set
                .get_default_lc_state_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        if let Some(period_id) = info_set.get_default_period_begin() {
            if let Some(period) = document.life_cycle_periods.get(period_id) {
                self.write_life_cycle_period(writer, "DEFAULT-PERIOD-BEGIN", period)?;
            }
        }
        if let Some(period_id) = info_set.get_default_period_end() {
            if let Some(period) = document.life_cycle_periods.get(period_id) {
                self.write_life_cycle_period(writer, "DEFAULT-PERIOD-END", period)?;
            }
        }
        // py writeLifeCycleInfoSetLifeCycleInfos — wrapper only when non-empty.
        let infos = info_set.get_life_cycle_infos();
        if !infos.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("LIFE-CYCLE-INFOS")))?;
            for info_id in infos {
                self.write_life_cycle_info(writer, *info_id, document)?;
            }
            writer.write_event(Event::End(BytesEnd::new("LIFE-CYCLE-INFOS")))?;
        }
        write_optional_ref_type(
            writer,
            "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF",
            info_set
                .get_used_life_cycle_state_definition_group_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;

        writer.write_event(Event::End(BytesEnd::new("LIFE-CYCLE-INFO-SET")))?;
        Ok(())
    }

    /// py `writePhysicalDimension` — the seven numerical children in fixed
    /// order after the Identifiable parts.
    fn write_physical_dimension<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: PhysicalDimensionId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(dimension) = document.physical_dimensions.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("PHYSICAL-DIMENSION");
        self.write_identifiable_attributes(
            &mut element,
            dimension.get_checksum(),
            dimension.get_timestamp(),
            dimension.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;

        let short_name_element = BytesStart::new("SHORT-NAME");
        write_text_element(
            writer,
            "SHORT-NAME",
            short_name_element,
            dimension.get_short_name(),
        )?;

        self.write_identifiable_parts(
            writer,
            IdentifiableParts {
                long_name: dimension.get_long_name(),
                desc: dimension.get_desc(),
                category: dimension.get_category(),
                introduction: dimension.get_introduction(),
                admin_data: dimension.get_admin_data(),
            },
            document,
        )?;

        write_optional_text_element(writer, "LENGTH-EXP", dimension.get_length_exp())?;
        write_optional_text_element(
            writer,
            "LUMINOUS-INTENSITY-EXP",
            dimension.get_luminous_intensity_exp(),
        )?;
        write_optional_text_element(writer, "MASS-EXP", dimension.get_mass_exp())?;
        write_optional_text_element(writer, "MOLAR-AMOUNT-EXP", dimension.get_molar_amount_exp())?;
        write_optional_text_element(writer, "TEMPERATURE-EXP", dimension.get_temperature_exp())?;
        write_optional_text_element(writer, "TIME-EXP", dimension.get_time_exp())?;
        write_optional_text_element(writer, "CURRENT-EXP", dimension.get_current_exp())?;

        writer.write_event(Event::End(BytesEnd::new("PHYSICAL-DIMENSION")))?;
        Ok(())
    }

    /// py `writeUnit` — DISPLAY-NAME wrapper, then factor/offset and the
    /// physical-dimension ref after the Identifiable parts.
    fn write_unit<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: UnitId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(unit) = document.units.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("UNIT");
        self.write_identifiable_attributes(
            &mut element,
            unit.get_checksum(),
            unit.get_timestamp(),
            unit.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;

        let short_name_element = BytesStart::new("SHORT-NAME");
        write_text_element(
            writer,
            "SHORT-NAME",
            short_name_element,
            unit.get_short_name(),
        )?;

        self.write_identifiable_parts(
            writer,
            IdentifiableParts {
                long_name: unit.get_long_name(),
                desc: unit.get_desc(),
                category: unit.get_category(),
                introduction: unit.get_introduction(),
                admin_data: unit.get_admin_data(),
            },
            document,
        )?;

        // py setSingleLanguageUnitNames — the mixed string is the element
        // text; SUP/SUB attrs only when set (never in this batch).
        if let Some(names) = unit
            .get_display_name()
            .and_then(|names_id| document.single_language_unit_names.get(names_id))
        {
            write_text_element(
                writer,
                "DISPLAY-NAME",
                BytesStart::new("DISPLAY-NAME"),
                names.base().atp_mixed_string().get_mixed_string(),
            )?;
        }
        write_optional_text_element(writer, "FACTOR-SI-TO-UNIT", unit.get_factor_si_to_unit())?;
        write_optional_text_element(writer, "OFFSET-SI-TO-UNIT", unit.get_offset_si_to_unit())?;
        write_optional_ref_type(
            writer,
            "PHYSICAL-DIMENSION-REF",
            unit.get_physical_dimension_ref()
                .and_then(|ref_id| document.ref_types.get(ref_id)),
        )?;

        writer.write_event(Event::End(BytesEnd::new("UNIT")))?;
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
