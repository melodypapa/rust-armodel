//! `ARXMLParser::load` with release detection, `read_admin_data`,
//! `read_ar_packages` (P0 design §6). Mirrors py's `arxml_parser.py`
//! method-for-method so the P2–P4 port stays reviewable.

use std::io::BufRead;
use std::path::Path;

use quick_xml::reader::Reader;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::{
    ARObject, ElementRef,
};
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackageId;
use crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::ByteOrderEnum;
use crate::m2::element_registry::{
    element_factory_for_tag, element_set_category, element_set_checksum, element_set_timestamp,
    element_set_uuid,
};
use crate::m2::msr::asam_hdo::admin_data::{AdminData, AdminDataId};
use crate::m2::msr::asam_hdo::base_types::BaseTypeDirectDefinition;
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdf, Sdg, SdgCaption, SdgContents, SdgId};
use crate::m2::msr::documentation::block_elements::list_elements::{ARList, Item, ListEnum};
use crate::m2::msr::documentation::block_elements::pagination_and_view::{
    ChapterEnumBreak, KeepWithPreviousEnum, Paginateable,
};
use crate::m2::msr::documentation::text_model::block_elements::{
    DocumentationBlock, DocumentationBlockId,
};
use crate::m2::msr::documentation::text_model::language_data_model::{
    LLongName, LOverviewParagraph, LParagraph, LPlainText, XmlSpace,
};
use crate::m2::msr::documentation::text_model::multilanguage_data::{
    MultiLanguageOverviewParagraph, MultiLanguageOverviewParagraphId, MultiLanguageParagraph,
    MultiLanguagePlainText, MultilanguageLongName, MultilanguageLongNameId,
};
use crate::parser::abstract_arxml_parser::{
    build_dom_from_reader, find, find_all, get_child_element_string, get_short_name, Node,
    ParseError,
};

/// The `Identifiable`-owned XML payload read by `read_identifiable_payload`
/// (py `readIdentifiable` minus SHORT-NAME/UUID/CATEGORY, which the
/// allocation path already consumed, and minus annotations — no pinned
/// fixture carries `ANNOTATIONS`; tracked on the port checklist).
struct IdentifiablePayload {
    long_name: Option<MultilanguageLongNameId>,
    desc: Option<MultiLanguageOverviewParagraphId>,
    introduction: Option<DocumentationBlockId>,
    admin_data: Option<AdminDataId>,
}

/// py `ARXMLParser(options)` — `warning: true` (the default) collects warnings
/// and continues; `warning: false` fails on the first problem.
#[derive(Debug, Clone)]
pub struct ParserOptions {
    pub warning: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self { warning: true }
    }
}

/// py's default options (`warning: true`).
pub fn default_options() -> ParserOptions {
    ParserOptions::default()
}

/// py `ARXMLParser`
#[derive(Debug)]
pub struct ARXMLParser {
    options: ParserOptions,
    warnings: Vec<String>,
}

impl Default for ARXMLParser {
    fn default() -> Self {
        Self::new(ParserOptions::default())
    }
}

impl ARXMLParser {
    pub fn new(options: ParserOptions) -> Self {
        Self {
            options,
            warnings: Vec::new(),
        }
    }

    /// Warnings collected so far (only populated in `warning: true` mode).
    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }

    /// py `raiseError` — log-and-continue in warning mode, hard error otherwise.
    fn raise_error(&mut self, message: String) -> Result<(), ParseError> {
        if self.options.warning {
            self.warnings.push(message);
            Ok(())
        } else {
            Err(ParseError::InvalidElement {
                element: "ARXML".to_string(),
                reason: message,
            })
        }
    }

    /// py `notImplemented`
    fn not_implemented(&mut self, message: String) -> Result<(), ParseError> {
        self.raise_error(message)
    }

    /// py `load`
    pub fn load(&mut self, path: &Path, document: &mut Document) -> Result<(), ParseError> {
        self.load_from_reader(Reader::from_file(path)?, document)
    }

    fn load_from_reader<R: BufRead>(
        &mut self,
        reader: Reader<R>,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let root = build_dom_from_reader(reader)?;
        if root.name != "AUTOSAR" {
            return Err(ParseError::UnexpectedRoot(root.name.clone()));
        }

        // py detectNamespace + getAUTOSARInfo: P0 records the schema
        // location; the XSD file name maps to a release via py's
        // `xsd_to_version_mapping` (conftest.py), defaulting to R23-11.
        if let Some(location) = root.attrs.get("xsi:schemaLocation") {
            document.set_schema_location(location.clone());
        }
        let xsd = document
            .get_schema_location()
            .split_whitespace()
            .nth(1)
            .unwrap_or("");
        document.set_ar_release(xsd_to_version(xsd));

        // py load: document.setAdminData(self.getAdminData(root, "ADMIN-DATA"))
        if let Some(admin_data_node) = find(&root, "ADMIN-DATA") {
            let admin_data = self.read_admin_data(admin_data_node, document)?;
            document.set_admin_data(admin_data);
        }
        // FILE-INFO-COMMENT and INTRODUCTION arrive in P1 (absent in this file).

        // py load: self.readARPackages(root, document)
        self.read_ar_packages(&root, None, document)?;
        Ok(())
    }

    /// py `readARObject` — reads the `S` (checksum) and `T` (timestamp)
    /// attributes.
    fn read_ar_object(&mut self, element: &Node, ar_object: &mut ARObject) {
        if let Some(checksum) = element.attrs.get("S") {
            ar_object.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = element.attrs.get("T") {
            ar_object.set_timestamp(timestamp.as_str());
        }
    }

    /// py `readWhitespaceControlled` — reads the element's own `xml:space`.
    fn read_xml_space(&mut self, element: &Node) -> Result<Option<XmlSpace>, ParseError> {
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

    /// py `getAdminData` — returns the allocated `AdminData` id.
    fn read_admin_data(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<AdminDataId, ParseError> {
        let mut admin_data = AdminData::new();
        self.read_ar_object(element, admin_data.base_mut());

        if let Some(language) = get_child_element_string(element, "LANGUAGE") {
            admin_data.set_language(language);
        }

        // py getMultiLanguagePlainText — USED-LANGUAGES is a single
        // MultiLanguagePlainText collecting the L-10 entries. (py's
        // readARObject on the USED-LANGUAGES element is dropped in P0:
        // MultiLanguagePlainText has no ARObject base here.)
        if let Some(used_languages_node) = find(element, "USED-LANGUAGES") {
            let mut paragraph = MultiLanguagePlainText::new();
            for l10_node in find_all(used_languages_node, "L-10") {
                let mut l10 = LPlainText::new();
                if let Some(l) = l10_node.attrs.get("L") {
                    l10.set_l(l.as_str());
                }
                if let Some(xml_space) = self.read_xml_space(l10_node)? {
                    l10.set_xml_space(xml_space);
                }
                // py readLanguageSpecific: setValue(element.text) — the text
                // itself; the DOM already applied the whitespace rule.
                if let Some(value) = &l10_node.text {
                    l10.set_value(value.as_str());
                }
                let l10_id = document.l_plain_texts.insert(l10);
                paragraph.push_l10(l10_id);
            }
            let paragraph_id = document.multi_language_plain_texts.insert(paragraph);
            admin_data.set_used_languages(paragraph_id);
        }

        // py readAdminDataSdgs
        if let Some(sdgs_node) = find(element, "SDGS") {
            for child in find_all(sdgs_node, "*") {
                if child.name == "SDG" {
                    let sdg_id = self.read_sdg(child, document)?;
                    admin_data.push_sdg(sdg_id);
                } else {
                    self.not_implemented(format!("Unsupported SDG <{}>", child.name))?;
                }
            }
        }

        // py readAdminDataDocRevisions — DocRevision is a P0 placeholder;
        // flag DOC-REVISIONS content instead of silently dropping it.
        if let Some(doc_revisions_node) = find(element, "DOC-REVISIONS") {
            if !find_all(doc_revisions_node, "*").is_empty() {
                self.not_implemented("DOC-REVISIONS are not supported in P0".to_string())?;
            }
        }

        Ok(document.admin_datas.insert(admin_data))
    }

    /// py `getSdg` — reads one `SDG` element (recursively) and returns its id.
    fn read_sdg(&mut self, element: &Node, document: &mut Document) -> Result<SdgId, ParseError> {
        let mut sdg = Sdg::new();
        self.read_ar_object(element, sdg.base_mut());
        if let Some(gid) = element.attrs.get("GID") {
            sdg.set_gid(gid.as_str());
        }

        // py readSdgCaption — SDG-CAPTION with a required SHORT-NAME.
        if let Some(caption_node) = find(element, "SDG-CAPTION") {
            let mut caption = SdgCaption::new();
            // py chain SdgCaption -> MultilanguageReferrable -> Referrable -> ARObject:
            // the generated model keeps every link, so the ARObject base is three hops out
            self.read_ar_object(caption_node, caption.base_mut().base_mut().base_mut());
            let caption_short_name = get_short_name(caption_node)?;
            caption.set_short_name(caption_short_name);
            if find(caption_node, "DESC").is_some() {
                self.not_implemented("SDG-CAPTION/DESC is not supported in P0".to_string())?;
            }
            let caption_id = document.sdg_captions.insert(caption);
            sdg.set_sdg_caption(caption_id);
        }

        // py readSd / readSdf / nested getSdg, collected into an SdgContents
        // that is attached only when non-empty.
        let mut contents = SdgContents::new();

        for sd_node in find_all(element, "SD") {
            let mut sd = Sd::new();
            self.read_ar_object(sd_node, sd.base_mut());
            if let Some(gid) = sd_node.attrs.get("GID") {
                sd.set_gid(gid.as_str());
            }
            if let Some(xml_space) = self.read_xml_space(sd_node)? {
                sd.set_xml_space(xml_space);
            }
            if let Some(value) = &sd_node.text {
                sd.set_value(value.as_str());
            }
            let sd_id = document.sds.insert(sd);
            contents.push_sd(sd_id);
        }

        for sdf_node in find_all(element, "SDF") {
            let mut sdf = Sdf::new();
            self.read_ar_object(sdf_node, sdf.base_mut());
            if let Some(gid) = sdf_node.attrs.get("GID") {
                sdf.set_gid(gid.as_str());
            }
            if let Some(value) = &sdf_node.text {
                sdf.set_value(value.as_str());
            }
            let sdf_id = document.sdfs.insert(sdf);
            contents.push_sdf(sdf_id);
        }

        for sdg_node in find_all(element, "SDG") {
            let nested_id = self.read_sdg(sdg_node, document)?;
            contents.push_sdg(nested_id);
        }

        // py SdgContents has no is_empty; contents exist iff any of SD/SDF/nested SDG was read
        if !(contents.get_sd().is_empty()
            && contents.get_sdf().is_empty()
            && contents.get_sdg().is_empty())
        {
            let contents_id = document.sdg_contents.insert(contents);
            sdg.set_sdg_contents_type(contents_id);
        }

        Ok(document.sdgs.insert(sdg))
    }

    /// py `readARPackages` — recurses into nested AR-PACKAGES. `parent` is
    /// `None` at the document root.
    fn read_ar_packages(
        &mut self,
        element: &Node,
        parent: Option<ARPackageId>,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        if let Some(packages_node) = find(element, "AR-PACKAGES") {
            for child in find_all(packages_node, "*") {
                if child.name == "AR-PACKAGE" {
                    self.read_ar_package(child, parent, document)?;
                } else {
                    self.not_implemented(format!("Unsupported ARPackage <{}>", child.name))?;
                }
            }
        }
        Ok(())
    }

    /// py `readARPackage` — allocates the package first (so children can link
    /// to it), then fills its own fields in one scoped arena borrow, then
    /// recurses.
    fn read_ar_package(
        &mut self,
        element: &Node,
        parent: Option<ARPackageId>,
        document: &mut Document,
    ) -> Result<ARPackageId, ParseError> {
        let short_name = get_short_name(element)?;
        let id = document.add_ar_package(parent, &short_name);

        // Scalar pieces are read before the arena borrow so the fill below is
        // a single scoped block (`docs/code_guide.md` §4).
        let admin_data = match find(element, "ADMIN-DATA") {
            Some(admin_data_node) => Some(self.read_admin_data(admin_data_node, document)?),
            None => None,
        };
        let uuid = element.attrs.get("UUID").cloned();
        let category = get_child_element_string(element, "CATEGORY").map(str::to_string);
        // py readIdentifiable on AR-PACKAGE: LONG-NAME/DESC/INTRODUCTION are
        // read before the arena borrow so the fill below stays one scoped block.
        let long_name = self.get_multilanguage_long_name(element, document)?;
        let desc = self.get_multi_language_overview_paragraph(element, document)?;
        let introduction = self.get_documentation_block(element, document)?;

        if let Some(package) = document.ar_packages.get_mut(id) {
            if let Some(admin_data) = admin_data {
                package.set_admin_data(admin_data);
            }
            if let Some(uuid) = uuid {
                package.set_uuid(uuid);
            }
            if let Some(category) = category {
                package.set_category(category);
            }
            // ARPackage forwards only a subset; the payload fields live on
            // its CollectableElement → Identifiable / MultilanguageReferrable
            // bases.
            let identifiable = package.base_mut().base_mut();
            if let Some(long_name) = long_name {
                identifiable.base_mut().set_long_name(long_name);
            }
            if let Some(desc) = desc {
                identifiable.set_desc(desc);
            }
            if let Some(introduction) = introduction {
                identifiable.set_introduction(introduction);
            }
        }

        // py readARPackageElements — dispatch through the P1 tag→constructor
        // registry. The common Identifiable parts (S/T attributes, UUID,
        // SHORT-NAME, CATEGORY) are read; per-class payload (DESC, ADMIN-DATA,
        // fields) is the P2–P4 mass port.
        if let Some(elements_node) = find(element, "ELEMENTS") {
            for child in find_all(elements_node, "*") {
                let factory = match element_factory_for_tag(&child.name) {
                    Some(factory) => factory,
                    None => {
                        self.not_implemented(format!(
                            "Unsupported ARPackage element <{}>",
                            child.name
                        ))?;
                        continue;
                    }
                };
                let short_name = get_short_name(child)?;
                let element_ref = factory(document, Some(id), &short_name);
                if let Some(checksum) = child.attrs.get("S") {
                    element_set_checksum(document, &element_ref, checksum);
                }
                if let Some(timestamp) = child.attrs.get("T") {
                    element_set_timestamp(document, &element_ref, timestamp);
                }
                if let Some(uuid) = child.attrs.get("UUID") {
                    element_set_uuid(document, &element_ref, uuid);
                }
                if let Some(category) = get_child_element_string(child, "CATEGORY") {
                    element_set_category(document, &element_ref, category);
                }
                // py readARPackageElements dispatches per class after the
                // common Identifiable parts; per-class payload lands arm by
                // arm (Tasks 4-9).
                self.read_element_payload(child, element_ref, document)?;
            }
        }
        if let Some(reference_bases_node) = find(element, "REFERENCE-BASES") {
            if !find_all(reference_bases_node, "*").is_empty() {
                self.not_implemented("REFERENCE-BASES are not supported in P0".to_string())?;
            }
        }

        self.read_ar_packages(element, Some(id), document)?;
        Ok(id)
    }

    /// py `getMultilanguageLongName` / `readLLongName` / `readMixedContentForLongName`.
    fn get_multilanguage_long_name(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<MultilanguageLongNameId>, ParseError> {
        let Some(long_name_node) = find(element, "LONG-NAME") else {
            return Ok(None);
        };
        let mut long_name = MultilanguageLongName::new();
        self.read_ar_object(long_name_node, long_name.base_mut());
        for l4_node in find_all(long_name_node, "L-4") {
            let mut l4 = LLongName::new();
            // py readMixedContentForLongName reads the L-4's ARObject (S/T);
            // LLongName forwards checksum/timestamp from its mixed-content base.
            if let Some(checksum) = l4_node.attrs.get("S") {
                l4.set_checksum(checksum.as_str());
            }
            if let Some(timestamp) = l4_node.attrs.get("T") {
                l4.set_timestamp(timestamp.as_str());
            }
            if let Some(text) = &l4_node.text {
                l4.set_value(text.as_str());
            }
            if let Some(l) = l4_node.attrs.get("L") {
                l4.set_l(l.as_str());
            }
            if let Some(sup) = l4_node.attrs.get("SUP") {
                l4.set_sup(sup.as_str());
            }
            if let Some(sub) = l4_node.attrs.get("SUB") {
                l4.set_sub(sub.as_str());
            }
            // py iterates inline children and only acts on E/IE/TT; neither
            // occurs in any pinned fixture, so they stay on the warning path.
            for inline in &l4_node.children {
                if matches!(inline.name.as_str(), "E" | "IE" | "TT") {
                    let message = format!("Unsupported inline element <{}> in L-4", inline.name);
                    self.not_implemented(message)?;
                }
            }
            let l4_id = document.l_long_names.insert(l4);
            long_name.push_l4(l4_id);
        }
        Ok(Some(document.multilanguage_long_names.insert(long_name)))
    }

    /// py `getMultiLanguageOverviewParagraph` / `readLOverviewParagraph`.
    fn get_multi_language_overview_paragraph(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<MultiLanguageOverviewParagraphId>, ParseError> {
        let Some(desc_node) = find(element, "DESC") else {
            return Ok(None);
        };
        let mut paragraph = MultiLanguageOverviewParagraph::new();
        self.read_ar_object(desc_node, paragraph.base_mut());
        for l2_node in find_all(desc_node, "L-2") {
            let mut l2 = LOverviewParagraph::new();
            if let Some(checksum) = l2_node.attrs.get("S") {
                l2.set_checksum(checksum.as_str());
            }
            if let Some(timestamp) = l2_node.attrs.get("T") {
                l2.set_timestamp(timestamp.as_str());
            }
            if let Some(text) = &l2_node.text {
                l2.set_value(text.as_str());
            }
            if let Some(l) = l2_node.attrs.get("L") {
                l2.set_l(l.as_str());
            }
            if let Some(blueprint) = l2_node.attrs.get("BLUEPRINT-VALUE") {
                l2.set_blueprint_value(blueprint.as_str());
            }
            let l2_id = document.l_overview_paragraphs.insert(l2);
            paragraph.push_l2(l2_id);
        }
        Ok(Some(
            document
                .multi_language_overview_paragraphs
                .insert(paragraph),
        ))
    }

    /// py `getDocumentationBlock` — the INTRODUCTION wrapper.
    fn get_documentation_block(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<DocumentationBlockId>, ParseError> {
        let Some(block_node) = find(element, "INTRODUCTION") else {
            return Ok(None);
        };
        let block = self.read_documentation_block(block_node, document)?;
        Ok(Some(block))
    }

    /// py `readDocumentationBlock` — P paragraphs and LIST items (+ ITEM
    /// recursion). The other block kinds (DEF-LIST, FORMULA, FIGURE, …) do
    /// not occur in any pinned fixture; py only reads them when present, so
    /// their absence here is behavior-identical (tracked on the checklist).
    fn read_documentation_block(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<DocumentationBlockId, ParseError> {
        let mut block = DocumentationBlock::new();
        self.read_ar_object(element, block.base_mut());
        // py getMultiLanguageParagraphs(element, "P")
        for p_node in find_all(element, "P") {
            let mut paragraph = MultiLanguageParagraph::new();
            self.read_paginateable(p_node, paragraph.base_mut())?;
            if let Some(help) = p_node.attrs.get("HELP-ENTRY") {
                paragraph.set_help_entry(help.as_str());
            }
            // py getLParagraphs(child_element, "L-1") — py reads inline
            // mixed content via readMixedContentForParagraph; no pinned
            // fixture carries it, so only the text/L form is read here.
            for l1_node in find_all(p_node, "L-1") {
                let mut l1 = LParagraph::new();
                if let Some(checksum) = l1_node.attrs.get("S") {
                    l1.set_checksum(checksum.as_str());
                }
                if let Some(timestamp) = l1_node.attrs.get("T") {
                    l1.set_timestamp(timestamp.as_str());
                }
                if let Some(text) = &l1_node.text {
                    l1.set_value(text.as_str());
                }
                if let Some(l) = l1_node.attrs.get("L") {
                    l1.set_l(l.as_str());
                }
                let l1_id = document.l_paragraphs.insert(l1);
                paragraph.push_l1(l1_id);
            }
            let paragraph_id = document.multi_language_paragraphs.insert(paragraph);
            block.push_p(paragraph_id);
        }
        // py getListElements(element, "LIST")
        for list_node in find_all(element, "LIST") {
            let mut list = ARList::new();
            self.read_paginateable(list_node, list.base_mut())?;
            if let Some(type_attr) = list_node.attrs.get("TYPE") {
                match ListEnum::try_from(type_attr.as_str()) {
                    Ok(value) => {
                        list.set_type(value);
                    }
                    Err(_) => {
                        let message = format!("Unsupported LIST TYPE <{type_attr}>");
                        self.not_implemented(message)?;
                    }
                }
            }
            for item_node in find_all(list_node, "ITEM") {
                let mut item = Item::new();
                self.read_paginateable(item_node, item.base_mut())?;
                let contents = self.read_documentation_block(item_node, document)?;
                item.set_item_contents(contents);
                let item_id = document.items.insert(item);
                list.push_item(item_id);
            }
            let list_id = document.ar_lists.insert(list);
            block.push_list(list_id);
        }
        Ok(document.documentation_blocks.insert(block))
    }

    /// py `readPaginateable` — BREAK / KEEP-WITH-PREVIOUS attributes.
    fn read_paginateable(
        &mut self,
        element: &Node,
        paginateable: &mut Paginateable,
    ) -> Result<(), ParseError> {
        if let Some(break_attr) = element.attrs.get("BREAK") {
            match ChapterEnumBreak::try_from(break_attr.as_str()) {
                Ok(value) => {
                    paginateable.set_chapter_break(value);
                }
                Err(_) => {
                    let message = format!("Unsupported BREAK <{break_attr}>");
                    self.not_implemented(message)?;
                }
            }
        }
        if let Some(keep) = element.attrs.get("KEEP-WITH-PREVIOUS") {
            match KeepWithPreviousEnum::try_from(keep.as_str()) {
                Ok(value) => {
                    paginateable.set_keep_with_previous(value);
                }
                Err(_) => {
                    let message = format!("Unsupported KEEP-WITH-PREVIOUS <{keep}>");
                    self.not_implemented(message)?;
                }
            }
        }
        Ok(())
    }

    /// py `readIdentifiable` — everything under the MultilanguageReferrable
    /// chain except SHORT-NAME/UUID/CATEGORY (read by the allocation path)
    /// and annotations/variation points (no pinned fixture carries them;
    /// tracked on the port checklist).
    fn read_identifiable_payload(
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

    /// py's per-class read dispatch (the tag→create+read chain in
    /// readARPackageElements). Grows one arm per ported family; the
    /// wildcard keeps unported families on the warnings-and-skip path.
    fn read_element_payload(
        &mut self,
        element: &Node,
        element_ref: ElementRef,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        match element_ref {
            ElementRef::SwBaseType(id) => {
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
                if let Some(definition) =
                    document.base_type_direct_definitions.get_mut(definition_id)
                {
                    self.read_base_type_direct_definition(element, definition)?;
                }
                Ok(())
            }
            // Arms land with Tasks 5-9.
            _ => Ok(()),
        }
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

/// py `xsd_to_version_mapping` (tests/integration_tests/conftest.py) —
/// XSD file name to release, defaulting to R23-11 (P0 design §6).
fn xsd_to_version(xsd: &str) -> &'static str {
    match xsd {
        "autosar.xsd" => "3.2.3",
        "AUTOSAR_4-0-3.xsd" => "4.0.3",
        "AUTOSAR_4-1-0.xsd" => "4.1.0",
        "AUTOSAR_4-1-1.xsd" => "4.1.1",
        "AUTOSAR_4-1-2.xsd" => "4.1.2",
        "AUTOSAR_4-1-3.xsd" => "4.1.3",
        "AUTOSAR_4-2-1.xsd" => "4.2.1",
        "AUTOSAR_4-2-2.xsd" => "4.2.2",
        "AUTOSAR_00043.xsd" => "4.3.0",
        "AUTOSAR_00044.xsd" => "4.3.1",
        "AUTOSAR_00046.xsd" => "4.4.0",
        "AUTOSAR_00048.xsd" => "R19-11",
        "AUTOSAR_00049.xsd" => "R20-11",
        "AUTOSAR_00050.xsd" => "R21-11",
        "AUTOSAR_00051.xsd" => "R22-11",
        "AUTOSAR_00052.xsd" => "R23-11",
        "AUTOSAR_00053.xsd" => "R24-11",
        _ => "R23-11",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ElementRef;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd">
  <ADMIN-DATA>
    <USED-LANGUAGES>
      <L-10 L="EN" xml:space="preserve">English</L-10>
    </USED-LANGUAGES>
    <SDGS>
      <SDG GID="demo">
        <SD GID="purpose" xml:space="preserve">special   data</SD>
      </SDG>
    </SDGS>
  </ADMIN-DATA>
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Outer</SHORT-NAME>
      <AR-PACKAGES>
        <AR-PACKAGE>
          <SHORT-NAME>Inner</SHORT-NAME>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    fn parse_sample() -> Document {
        let mut document = Document::new();
        ARXMLParser::new(default_options())
            .load_from_reader(Reader::from_str(SAMPLE), &mut document)
            .unwrap();
        document
    }

    #[test]
    fn load_sets_schema_location_and_release() {
        let document = parse_sample();
        assert_eq!(
            document.get_schema_location(),
            "http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd"
        );
        assert_eq!(document.get_ar_release(), "R21-11");
    }

    #[test]
    fn load_rejects_wrong_root() {
        let mut document = Document::new();
        let result = ARXMLParser::new(default_options()).load_from_reader(
            Reader::from_str("<?xml version=\"1.0\"?><WRONG/>"),
            &mut document,
        );
        assert!(matches!(result, Err(ParseError::UnexpectedRoot(_))));
    }

    const IDENTIFIABLE_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00052.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <ELEMENTS>
        <SW-BASE-TYPE>
          <SHORT-NAME>T</SHORT-NAME>
          <LONG-NAME>
            <L-4 L="DE">Ein Typ</L-4>
          </LONG-NAME>
          <DESC>
            <L-2 L="EN" xml:space="preserve">A  type</L-2>
          </DESC>
          <INTRODUCTION>
            <P>
              <L-1 L="EN">Intro text</L-1>
            </P>
            <LIST TYPE="LIST">
              <ITEM>
                <P><L-1>Nested</L-1></P>
              </ITEM>
            </LIST>
          </INTRODUCTION>
        </SW-BASE-TYPE>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    #[test]
    fn identifiable_payload_round_trips_text_nodes() {
        let mut document = Document::new();
        ARXMLParser::new(default_options())
            .load_from_reader(Reader::from_str(IDENTIFIABLE_SAMPLE), &mut document)
            .unwrap();

        let pkg = document
            .get_ar_package(document.get_ar_packages()[0])
            .unwrap();
        let ElementRef::SwBaseType(id) = pkg.get_elements()[0] else {
            panic!("expected SwBaseType element");
        };
        let base_type = document.get_sw_base_type(id).unwrap();
        let long_name = document
            .get_multilanguage_long_name(base_type.get_long_name().unwrap())
            .unwrap();
        let l4 = document.get_l_long_name(long_name.get_l4()[0]).unwrap();
        assert_eq!(l4.get_l(), Some("DE"));
        assert_eq!(l4.get_value(), Some("Ein Typ"));

        let desc = document
            .get_multi_language_overview_paragraph(base_type.get_desc().unwrap())
            .unwrap();
        let l2 = document.get_l_overview_paragraph(desc.get_l2()[0]).unwrap();
        assert_eq!(l2.get_value(), Some("A  type"));

        let introduction = document
            .get_documentation_block(base_type.get_introduction().unwrap())
            .unwrap();
        assert_eq!(introduction.get_ps().len(), 1);
        assert_eq!(introduction.get_lists().len(), 1);
    }

    #[test]
    fn admin_data_round_trips_through_the_model() {
        let document = parse_sample();

        let admin_data = document.get_admin_data().unwrap();

        let sdg = document.get_sdg(admin_data.get_sdgs()[0]).unwrap();
        assert_eq!(sdg.get_gid(), Some("demo"));

        let contents = document
            .get_sdg_contents(sdg.get_sdg_contents_type().unwrap())
            .unwrap();
        let sd = document.get_sd(contents.get_sd()[0]).unwrap();
        assert_eq!(sd.get_gid(), Some("purpose"));
        assert_eq!(sd.get_xml_space(), Some(XmlSpace::Preserve));
        assert_eq!(sd.get_value(), Some("special   data"));

        let mlpt = document
            .get_multi_language_plain_text(admin_data.get_used_languages().unwrap())
            .unwrap();
        let l10 = document.get_l_plain_text(mlpt.get_l10s()[0]).unwrap();
        assert_eq!(l10.get_l(), Some("EN"));
        assert_eq!(l10.get_value(), Some("English"));
    }

    #[test]
    fn packages_nest_through_the_document_factory() {
        let document = parse_sample();
        assert_eq!(document.get_ar_packages().len(), 1);

        let outer_id = document.get_ar_packages()[0];
        let outer = document.get_ar_package(outer_id).unwrap();
        assert_eq!(outer.get_short_name(), Some("Outer"));
        assert!(outer.get_parent().is_none());

        let inner_id = outer.get_ar_packages()[0];
        let inner = document.get_ar_package(inner_id).unwrap();
        assert_eq!(inner.get_short_name(), Some("Inner"));
        assert!(
            matches!(inner.get_parent(), Some(ElementRef::ARPackage(parent)) if parent == outer_id)
        );
    }
}
