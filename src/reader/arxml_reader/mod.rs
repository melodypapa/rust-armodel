//! `ARXMLReader::load` with release detection, `read_admin_data`,
//! `read_ar_packages` (P0 design §6). Mirrors py's `arxml_parser.py`
//! method-for-method so the P2–P4 port stays reviewable.

use std::io::BufRead;
use std::path::Path;

use quick_xml::reader::Reader;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::autosar_templates::common_structure::standardization_template::keyword::{
    Keyword, KeywordId, KeywordSetId,
};
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::{
    ARObject, ElementRef,
};
use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::{
    ARPackageId, ReferenceBase,
};
use crate::m2::autosar_templates::generic_structure::general_template_classes::element_collection::{
    AutoCollectEnum, CollectionId,
};
use crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::{
    ByteOrderEnum, Limit, LimitId, MonotonyEnum, RefTypeId,
};
use crate::m2::autosar_templates::generic_structure::life_cycles::{
    LifeCycleInfo, LifeCycleInfoId, LifeCycleInfoSetId, LifeCyclePeriod, LifeCyclePeriodId,
};
use crate::m2::element_registry::{
    element_factory_for_tag, element_set_category, element_set_checksum, element_set_timestamp,
    element_set_uuid,
};
use crate::m2::msr::asam_hdo::admin_data::{AdminData, AdminDataId};
use crate::m2::autosar_templates::sw_component_template::datatype::data_prototypes::{
    ApplicationArrayElement, ApplicationRecordElement,
};
use crate::m2::autosar_templates::sw_component_template::datatype::datatypes::{
    ApplicationArrayDataTypeId, ApplicationPrimitiveDataTypeId, ApplicationRecordDataTypeId,
};
use crate::m2::autosar_templates::common_structure::implementation_data_types::ImplementationDataTypeId;
use crate::m2::msr::asam_hdo::base_types::{BaseTypeDirectDefinition, SwBaseTypeId};
use crate::m2::msr::data_dictionary::data_def_properties::{
    DisplayPresentationEnum, SwDataDefProps, SwDataDefPropsId,
};
use crate::m2::msr::asam_hdo::constraints::global_constraints::{
    DataConstrId, InternalConstrs, PhysConstrs, ScaleConstr,
    ScaleConstrValidityEnum,
};
use crate::m2::msr::asam_hdo::computation_method::{
    Compu, CompuConst, CompuConstContentRef, CompuConstFormulaContent, CompuConstId,
    CompuConstNumericContent, CompuConstTextContent, CompuContentRef, CompuId, CompuMethodId,
    CompuNominatorDenominator, CompuRationalCoeffs, CompuScale, CompuScales,
    CompuScaleConstantContents, CompuScaleContentsRef, CompuScaleId, CompuScaleRationalFormula,
};
use crate::m2::msr::asam_hdo::special_data::{Sd, Sdf, Sdg, SdgCaption, SdgContents, SdgId};
use crate::m2::msr::asam_hdo::units::{PhysicalDimensionId, SingleLanguageUnitNames, UnitId};
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
use crate::reader::abstract_arxml_reader::{
    Node,
    ParseError,
    build_dom_from_reader,
    find,
    find_all,
    get_child_element_optional_boolean,
    get_child_element_optional_ref_type,
    get_child_element_optional_t_ref_type,
    get_child_element_ref_type_list,
    get_child_element_string,
    get_short_name,
};

mod common;

/// The `LifeCycleInfoSet`-owned XML payload read by
/// `read_life_cycle_info_set_payload` (py `readLifeCycleInfoSet` minus the
/// common Identifiable parts; refs are already arena ids).
struct LifeCycleInfoSetPayload {
    default_lc_state_ref: Option<RefTypeId>,
    default_period_begin: Option<LifeCyclePeriodId>,
    default_period_end: Option<LifeCyclePeriodId>,
    life_cycle_infos: Vec<LifeCycleInfoId>,
    used_life_cycle_state_definition_group_ref: Option<RefTypeId>,
}

/// py `ARXMLParser(options)` — `warning: true` (the default) collects warnings
/// and continues; `warning: false` fails on the first problem.
#[derive(Debug, Clone)]
pub struct ReaderOptions {
    pub warning: bool,
}

impl Default for ReaderOptions {
    fn default() -> Self {
        Self { warning: true }
    }
}

/// py's default options (`warning: true`).
pub fn default_options() -> ReaderOptions {
    ReaderOptions::default()
}

/// py `ARXMLParser`
#[derive(Debug)]
pub struct ARXMLReader {
    options: ReaderOptions,
    warnings: Vec<String>,
}

impl Default for ARXMLReader {
    fn default() -> Self {
        Self::new(ReaderOptions::default())
    }
}

impl ARXMLReader {
    pub fn new(options: ReaderOptions) -> Self {
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
        // py readIdentifiable → readARObject: the package's own S/T attrs.
        let checksum = element.attrs.get("S").cloned();
        let timestamp = element.attrs.get("T").cloned();
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
            if let Some(checksum) = checksum {
                package.set_checksum(checksum);
            }
            if let Some(timestamp) = timestamp {
                package.set_timestamp(timestamp);
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
        // py readReferenceBases
        if let Some(bases_node) = find(element, "REFERENCE-BASES") {
            for base_node in find_all(bases_node, "REFERENCE-BASE") {
                let mut base = ReferenceBase::new();
                self.read_ar_object(base_node, base.base_mut());
                if let Some(label) = get_child_element_string(base_node, "SHORT-LABEL") {
                    base.set_short_label(label);
                }
                if let Some(value) = get_child_element_optional_boolean(base_node, "IS-DEFAULT") {
                    base.set_is_default(value);
                }
                if let Some(value) = get_child_element_optional_boolean(base_node, "IS-GLOBAL") {
                    base.set_is_global(value);
                }
                if let Some(value) =
                    get_child_element_optional_boolean(base_node, "BASE-IS-THIS-PACKAGE")
                {
                    base.set_base_is_this_package(value);
                }
                // RefType values live in the arena; the model links by id.
                for r#ref in get_child_element_ref_type_list(
                    base_node,
                    "GLOBAL-IN-PACKAGE-REFS/GLOBAL-IN-PACKAGE-REF",
                ) {
                    let ref_id = document.ref_types.insert(r#ref);
                    base.push_global_in_package_ref(ref_id);
                }
                for global in find_all(base_node, "GLOBAL-ELEMENTS/GLOBAL-ELEMENT") {
                    if let Some(text) = &global.text {
                        base.push_global_elements(text.as_str());
                    }
                }
                if let Some(r#ref) = get_child_element_optional_ref_type(base_node, "PACKAGE-REF") {
                    let ref_id = document.ref_types.insert(r#ref);
                    base.set_package_ref(ref_id);
                }
                let base_id = document.reference_bases.insert(base);
                if let Some(package) = document.ar_packages.get_mut(id) {
                    package.push_reference_base(base_id);
                }
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
                // py: ListEnum().setValue(value.lower()); the writer uppercases
                match ListEnum::try_from(type_attr.to_lowercase().as_str()) {
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
            ElementRef::CompuMethod(id) => self.read_compu_method(element, id, document),
            ElementRef::DataConstr(id) => self.read_data_constr(element, id, document),
            ElementRef::KeywordSet(id) => self.read_keyword_set(element, id, document),
            ElementRef::ApplicationPrimitiveDataType(id) => {
                self.read_application_primitive_data_type(element, id, document)
            }
            ElementRef::ApplicationArrayDataType(id) => {
                self.read_application_array_data_type(element, id, document)
            }
            ElementRef::ApplicationRecordDataType(id) => {
                self.read_application_record_data_type(element, id, document)
            }
            ElementRef::ImplementationDataType(id) => {
                self.read_implementation_data_type(element, id, document)
            }
            ElementRef::SwBaseType(id) => self.read_sw_base_type(element, id, document),
            ElementRef::Collection(id) => self.read_collection(element, id, document),
            ElementRef::LifeCycleInfoSet(id) => {
                self.read_life_cycle_info_set(element, id, document)
            }
            ElementRef::PhysicalDimension(id) => {
                self.read_physical_dimension(element, id, document)
            }
            ElementRef::Unit(id) => self.read_unit(element, id, document),
            // Remaining families land in later P2 batches.
            _ => Ok(()),
        }
    }

    /// py `getCompuConstContent` — the first child element dispatches
    /// VT/V/VF into the concrete content class.
    fn get_compu_const_content(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<CompuConstContentRef>, ParseError> {
        let Some(child) = find(element, "*") else {
            return Ok(None);
        };
        match child.name.as_str() {
            "VT" => {
                let mut content = CompuConstTextContent::new();
                if let Some(vt) = get_child_element_string(element, "VT") {
                    content.set_vt(vt);
                }
                let id = document.compu_const_text_contents.insert(content);
                Ok(Some(CompuConstContentRef::CompuConstTextContent(id)))
            }
            "V" => {
                let mut content = CompuConstNumericContent::new();
                if let Some(v) = get_child_element_string(element, "V") {
                    content.set_v(v);
                }
                let id = document.compu_const_numeric_contents.insert(content);
                Ok(Some(CompuConstContentRef::CompuConstNumericContent(id)))
            }
            "VF" => {
                let mut content = CompuConstFormulaContent::new();
                if let Some(vf) = get_child_element_string(element, "VF") {
                    content.set_vf(vf);
                }
                let id = document.compu_const_formula_contents.insert(content);
                Ok(Some(CompuConstContentRef::CompuConstFormulaContent(id)))
            }
            other => {
                let message = format!("Unsupported CompuConstContent <{other}>");
                self.not_implemented(message)?;
                Ok(None)
            }
        }
    }

    /// py `getCompuConst` — a keyed `<COMPU-…>` wrapper carrying content.
    fn get_compu_const(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<CompuConstId>, ParseError> {
        let Some(child) = find(element, key) else {
            return Ok(None);
        };
        let mut compu_const = CompuConst::new();
        self.read_ar_object(child, compu_const.base_mut());
        if let Some(content) = self.get_compu_const_content(child, document)? {
            compu_const.set_compu_const_content_type(content);
        }
        Ok(Some(document.compu_consts.insert(compu_const)))
    }

    /// py `readCompuScale`.
    fn read_compu_scale(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<CompuScaleId, ParseError> {
        let mut scale = CompuScale::new();
        self.read_ar_object(element, scale.base_mut());
        if let Some(text) = get_child_element_string(element, "A2L-DISPLAY-TEXT") {
            scale.set_a2l_display_text(text);
        }
        if let Some(inverse) = self.get_compu_const(element, "COMPU-INVERSE-VALUE", document)? {
            scale.set_compu_inverse_value(inverse);
        }
        if let Some(text) = get_child_element_string(element, "SHORT-LABEL") {
            scale.set_short_label(text);
        }
        if let Some(text) = get_child_element_string(element, "SYMBOL") {
            scale.set_symbol(text);
        }
        if let Some(desc) = self.get_multi_language_overview_paragraph(element, document)? {
            scale.set_desc(desc);
        }
        if let Some(mask) = get_child_element_string(element, "MASK") {
            scale.set_mask(mask);
        }
        if let Some(limit) = self.get_child_limit_element(element, "LOWER-LIMIT", document)? {
            scale.set_lower_limit(limit);
        }
        if let Some(limit) = self.get_child_limit_element(element, "UPPER-LIMIT", document)? {
            scale.set_upper_limit(limit);
        }
        // py readCompuScaleContents — constant (COMPU-CONST/VT) or rational
        // (COMPU-RATIONAL-COEFFS) contents.
        if find(element, "COMPU-CONST/VT").is_some() {
            let vt = get_child_element_string(element, "COMPU-CONST/VT").unwrap_or("");
            let mut text_content = CompuConstTextContent::new();
            text_content.set_vt(vt);
            let vt_id = document.compu_const_text_contents.insert(text_content);
            let mut const_ = CompuConst::new();
            const_.set_compu_const_content_type(CompuConstContentRef::CompuConstTextContent(vt_id));
            let const_id = document.compu_consts.insert(const_);
            let mut constant = CompuScaleConstantContents::new();
            constant.set_compu_const(const_id);
            let content_id = document.compu_scale_constant_contents.insert(constant);
            scale.set_compu_scale_contents(CompuScaleContentsRef::CompuScaleConstantContents(
                content_id,
            ));
        } else if find(element, "COMPU-RATIONAL-COEFFS").is_some() {
            // py readCompuNominatorDenominator — V children in document order.
            let coeffs_node = find(element, "COMPU-RATIONAL-COEFFS").expect("checked above");
            let numerator = {
                let mut nom_denom = CompuNominatorDenominator::new();
                for v_node in find_all(coeffs_node, "COMPU-NUMERATOR/V") {
                    if let Some(text) = &v_node.text {
                        nom_denom.push_v(text.as_str().to_string());
                    }
                }
                document.compu_nominator_denominators.insert(nom_denom)
            };
            let denominator = {
                let mut nom_denom = CompuNominatorDenominator::new();
                for v_node in find_all(coeffs_node, "COMPU-DENOMINATOR/V") {
                    if let Some(text) = &v_node.text {
                        nom_denom.push_v(text.as_str().to_string());
                    }
                }
                document.compu_nominator_denominators.insert(nom_denom)
            };
            let mut coeffs = CompuRationalCoeffs::new();
            coeffs.set_compu_numerator(numerator);
            coeffs.set_compu_denominator(denominator);
            let coeffs_id = document.compu_rational_coeffs.insert(coeffs);
            let mut formula = CompuScaleRationalFormula::new();
            formula.set_compu_rational_coeffs(coeffs_id);
            let content_id = document.compu_scale_rational_formulas.insert(formula);
            scale.set_compu_scale_contents(CompuScaleContentsRef::CompuScaleRationalFormula(
                content_id,
            ));
        }
        Ok(document.compu_scales.insert(scale))
    }

    /// py `getCompuScales` — the COMPU-SCALES wrapper and its scales.
    fn get_compu_scales(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<CompuContentRef>, ParseError> {
        let Some(scales_node) = find(element, "COMPU-SCALES") else {
            return Ok(None);
        };
        let mut scales = CompuScales::new();
        if let Some(checksum) = scales_node.attrs.get("S") {
            scales.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = scales_node.attrs.get("T") {
            scales.set_timestamp(timestamp.as_str());
        }
        for scale_node in find_all(scales_node, "COMPU-SCALE") {
            let scale_id = self.read_compu_scale(scale_node, document)?;
            scales.push_compu_scale(scale_id);
        }
        let id = document.compu_scales_arena.insert(scales);
        Ok(Some(CompuContentRef::CompuScales(id)))
    }

    /// py `getCompu` — a keyed `<COMPU-…>` wrapper: scales + default value.
    fn get_compu(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<CompuId>, ParseError> {
        let Some(child) = find(element, key) else {
            return Ok(None);
        };
        let mut compu = Compu::new();
        self.read_ar_object(child, compu.base_mut());
        if let Some(content) = self.get_compu_scales(child, document)? {
            compu.set_compu_content(content);
        }
        if let Some(default) = self.get_compu_const(child, "COMPU-DEFAULT-VALUE", document)? {
            compu.set_compu_default_value(default);
        }
        Ok(Some(document.compus.insert(compu)))
    }

    /// py `readCompuMethod` — Identifiable chain + the compu payload.
    fn read_compu_method(
        &mut self,
        element: &Node,
        id: CompuMethodId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let display_format =
            get_child_element_string(element, "DISPLAY-FORMAT").map(str::to_string);
        let unit_ref = get_child_element_optional_ref_type(element, "UNIT-REF");
        let internal = self.get_compu(element, "COMPU-INTERNAL-TO-PHYS", document)?;
        let phys = self.get_compu(element, "COMPU-PHYS-TO-INTERNAL", document)?;
        if let Some(compu_method) = document.compu_methods.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                compu_method.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                compu_method.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                compu_method.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                compu_method.set_admin_data(admin_data);
            }
            if let Some(display_format) = display_format {
                compu_method.set_display_format(display_format);
            }
            if let Some(unit_ref) = unit_ref {
                let unit_ref_id = document.ref_types.insert(unit_ref);
                compu_method.set_unit_ref(unit_ref_id);
            }
            if let Some(internal) = internal {
                compu_method.set_compu_internal_to_phys(internal);
            }
            if let Some(phys) = phys {
                compu_method.set_compu_phys_to_internal(phys);
            }
        }
        Ok(())
    }

    /// py `readScaleConstr`.
    fn read_scale_constr(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<crate::m2::msr::asam_hdo::constraints::global_constraints::ScaleConstrId, ParseError>
    {
        let mut scale_constr = ScaleConstr::new();
        self.read_ar_object(element, scale_constr.base_mut());
        if let Some(desc) = self.get_multi_language_overview_paragraph(element, document)? {
            scale_constr.set_desc(desc);
        }
        if let Some(limit) = self.get_child_limit_element(element, "LOWER-LIMIT", document)? {
            scale_constr.set_lower_limit(limit);
        }
        if let Some(label) = get_child_element_string(element, "SHORT-LABEL") {
            scale_constr.set_short_label(label);
        }
        if let Some(limit) = self.get_child_limit_element(element, "UPPER-LIMIT", document)? {
            scale_constr.set_upper_limit(limit);
        }
        if let Some(validity) = element.attrs.get("VALIDITY") {
            match ScaleConstrValidityEnum::try_from(validity.as_str()) {
                Ok(value) => {
                    scale_constr.set_validity(value);
                }
                Err(_) => {
                    let message = format!("Unsupported VALIDITY <{validity}>");
                    self.not_implemented(message)?;
                }
            }
        }
        Ok(document.scale_constrs.insert(scale_constr))
    }

    /// py `readDataConstr` — Identifiable chain + DATA-CONSTR-RULES.
    fn read_data_constr(
        &mut self,
        element: &Node,
        id: DataConstrId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        // py readDataConstrRule
        let mut rules = Vec::new();
        for rule_node in find_all(element, "DATA-CONSTR-RULES/DATA-CONSTR-RULE") {
            let mut rule =
                crate::m2::msr::asam_hdo::constraints::global_constraints::DataConstrRule::new();
            self.read_ar_object(rule_node, rule.base_mut());
            if let Some(level) = get_child_element_string(rule_node, "CONSTR-LEVEL") {
                rule.set_constr_level(level);
            }
            // py readInternalConstrs
            if let Some(constrs_node) = find(rule_node, "INTERNAL-CONSTRS") {
                let mut constrs = InternalConstrs::new();
                self.read_ar_object(constrs_node, constrs.base_mut());
                if let Some(limit) =
                    self.get_child_limit_element(constrs_node, "LOWER-LIMIT", document)?
                {
                    constrs.set_lower_limit(limit);
                }
                if let Some(limit) =
                    self.get_child_limit_element(constrs_node, "UPPER-LIMIT", document)?
                {
                    constrs.set_upper_limit(limit);
                }
                for sc in find_all(constrs_node, "SCALE-CONSTRS/SCALE-CONSTR") {
                    let sc_id = self.read_scale_constr(sc, document)?;
                    constrs.push_scale_constr(sc_id);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MAX-GRADIENT") {
                    constrs.set_max_gradient(text);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MAX-DIFF") {
                    constrs.set_max_diff(text);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MONOTONY") {
                    match MonotonyEnum::try_from(text) {
                        Ok(value) => {
                            constrs.set_monotony(value);
                        }
                        Err(_) => {
                            let message = format!("Unsupported MONOTONY <{text}>");
                            self.not_implemented(message)?;
                        }
                    }
                }
                let constrs_id = document.internal_constrs.insert(constrs);
                rule.set_internal_constrs(constrs_id);
            }
            // py readPhysConstrs
            if let Some(constrs_node) = find(rule_node, "PHYS-CONSTRS") {
                let mut constrs = PhysConstrs::new();
                self.read_ar_object(constrs_node, constrs.base_mut());
                if let Some(limit) =
                    self.get_child_limit_element(constrs_node, "LOWER-LIMIT", document)?
                {
                    constrs.set_lower_limit(limit);
                }
                if let Some(limit) =
                    self.get_child_limit_element(constrs_node, "UPPER-LIMIT", document)?
                {
                    constrs.set_upper_limit(limit);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MAX-DIFF") {
                    constrs.set_max_diff(text);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MAX-GRADIENT") {
                    constrs.set_max_gradient(text);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MONOTONY") {
                    match MonotonyEnum::try_from(text) {
                        Ok(value) => {
                            constrs.set_monotony(value);
                        }
                        Err(_) => {
                            let message = format!("Unsupported MONOTONY <{text}>");
                            self.not_implemented(message)?;
                        }
                    }
                }
                for sc in find_all(constrs_node, "SCALE-CONSTRS/SCALE-CONSTR") {
                    let sc_id = self.read_scale_constr(sc, document)?;
                    constrs.push_scale_constr(sc_id);
                }
                if let Some(unit_ref) =
                    get_child_element_optional_ref_type(constrs_node, "UNIT-REF")
                {
                    let unit_ref_id = document.ref_types.insert(unit_ref);
                    constrs.set_unit_ref(unit_ref_id);
                }
                let constrs_id = document.phys_constrs.insert(constrs);
                rule.set_phys_constrs(constrs_id);
            }
            rules.push(rule);
        }
        if let Some(data_constr) = document.data_constrs.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                data_constr.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                data_constr.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                data_constr.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                data_constr.set_admin_data(admin_data);
            }
            for rule in rules {
                let rule_id = document.data_constr_rules.insert(rule);
                data_constr.push_data_constr_rule(rule_id);
            }
        }
        Ok(())
    }

    /// py `readKeyword` — payload + ABBR-NAME + CLASSIFICATIONS texts.
    fn read_keyword(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<KeywordId, ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let abbr_name = get_child_element_string(element, "ABBR-NAME").map(str::to_string);
        // py readKeywordClassifications
        let mut classifications = Vec::new();
        for class_node in find_all(element, "CLASSIFICATIONS/CLASSIFICATION") {
            if let Some(text) = &class_node.text {
                classifications.push(text.as_str().to_string());
            }
        }
        let short_name = get_short_name(element)?;
        let mut keyword = Keyword::new();
        keyword.set_short_name(short_name);
        if let Some(checksum) = element.attrs.get("S") {
            keyword.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = element.attrs.get("T") {
            keyword.set_timestamp(timestamp.as_str());
        }
        if let Some(uuid) = element.attrs.get("UUID") {
            keyword.set_uuid(uuid);
        }
        if let Some(category) = get_child_element_string(element, "CATEGORY") {
            keyword.set_category(category);
        }
        if let Some(long_name) = payload.long_name {
            keyword.set_long_name(long_name);
        }
        if let Some(desc) = payload.desc {
            keyword.set_desc(desc);
        }
        if let Some(introduction) = payload.introduction {
            keyword.set_introduction(introduction);
        }
        if let Some(admin_data) = payload.admin_data {
            keyword.set_admin_data(admin_data);
        }
        if let Some(abbr_name) = abbr_name {
            keyword.set_abbr_name(abbr_name);
        }
        for classification in classifications {
            keyword.push_classification(classification);
        }
        Ok(document.keywords.insert(keyword))
    }

    /// py `readKeywordSet` — Identifiable chain + KEYWORDS children.
    fn read_keyword_set(
        &mut self,
        element: &Node,
        id: KeywordSetId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let mut keywords = Vec::new();
        for keyword_node in find_all(element, "KEYWORDS/KEYWORD") {
            keywords.push(self.read_keyword(keyword_node, document)?);
        }
        if let Some(keyword_set) = document.keyword_sets.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                keyword_set.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                keyword_set.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                keyword_set.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                keyword_set.set_admin_data(admin_data);
            }
            for keyword in keywords {
                keyword_set.push_keyword(keyword);
            }
        }
        Ok(())
    }

    /// py `getSwDataDefProps` — the VARIANTS/CONDITIONAL walk. py's model
    /// stores the conditional inline (no wrapper class); the Rust side does
    /// the same. Only the model-backed fields py reads unconditionally are
    /// ported; absent fixtures never trigger the rest.
    fn get_sw_data_def_props(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<SwDataDefPropsId>, ParseError> {
        let Some(child) = find(element, key) else {
            return Ok(None);
        };
        let conditional = find(
            child,
            "SW-DATA-DEF-PROPS-VARIANTS/SW-DATA-DEF-PROPS-CONDITIONAL",
        );
        let mut read_props = |document: &mut Document,
                              source: &Node|
         -> Result<SwDataDefPropsId, ParseError> {
            let mut props = SwDataDefProps::new();
            if let Some(checksum) = source.attrs.get("S") {
                props.set_checksum(checksum.as_str());
            }
            if let Some(timestamp) = source.attrs.get("T") {
                props.set_timestamp(timestamp.as_str());
            }
            if let Some(text) = get_child_element_string(source, "DISPLAY-PRESENTATION") {
                match DisplayPresentationEnum::try_from(text) {
                    Ok(value) => {
                        props.set_display_presentation(value);
                    }
                    Err(_) => {
                        let message = format!("Unsupported DISPLAY-PRESENTATION <{text}>");
                        self.not_implemented(message)?;
                    }
                }
            }
            if let Some(r#ref) = get_child_element_optional_ref_type(source, "BASE-TYPE-REF") {
                let id = document.ref_types.insert(r#ref);
                props.set_base_type_ref(id);
            }
            if let Some(r#ref) = get_child_element_optional_ref_type(source, "SW-ADDR-METHOD-REF") {
                let id = document.ref_types.insert(r#ref);
                props.set_sw_addr_method_ref(id);
            }
            if let Some(text) = get_child_element_string(source, "SW-ALIGNMENT") {
                props.set_sw_alignment(text);
            }
            if let Some(text) = get_child_element_string(source, "SW-CALIBRATION-ACCESS") {
                props.set_sw_calibration_access(text);
            }
            if let Some(r#ref) = get_child_element_optional_ref_type(source, "COMPU-METHOD-REF") {
                let id = document.ref_types.insert(r#ref);
                props.set_compu_method_ref(id);
            }
            if let Some(text) = get_child_element_string(source, "STEP-SIZE") {
                props.set_step_size(text);
            }
            if let Some(r#ref) = get_child_element_optional_ref_type(source, "DATA-CONSTR-REF") {
                let id = document.ref_types.insert(r#ref);
                props.set_data_constr_ref(id);
            }
            if let Some(r#ref) =
                get_child_element_optional_ref_type(source, "IMPLEMENTATION-DATA-TYPE-REF")
            {
                let id = document.ref_types.insert(r#ref);
                props.set_implementation_data_type_ref(id);
            }
            if let Some(text) = get_child_element_string(source, "SW-INTENDED-RESOLUTION") {
                props.set_sw_intended_resolution(text);
            }
            if let Some(r#ref) = get_child_element_optional_ref_type(source, "UNIT-REF") {
                let id = document.ref_types.insert(r#ref);
                props.set_unit_ref(id);
            }
            if let Some(text) = get_child_element_string(source, "DISPLAY-FORMAT") {
                props.set_display_format(text);
            }
            Ok(document.sw_data_def_props.insert(props))
        };
        match conditional {
            Some(conditional_node) => {
                let id = read_props(document, conditional_node)?;
                Ok(Some(id))
            }
            None => {
                // py keeps the props even without the conditional wrapper
                let id = read_props(document, child)?;
                Ok(Some(id))
            }
        }
    }

    /// py `readApplicationPrimitiveDataType`.
    fn read_application_primitive_data_type(
        &mut self,
        element: &Node,
        id: ApplicationPrimitiveDataTypeId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let props = self.get_sw_data_def_props(element, "SW-DATA-DEF-PROPS", document)?;
        if let Some(data_type) = document.application_primitive_data_types.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                data_type.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                data_type.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                data_type.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                data_type.set_admin_data(admin_data);
            }
            if let Some(props) = props {
                data_type.set_sw_data_def_props(props);
            }
        }
        Ok(())
    }

    /// py `readApplicationArrayDataType` (+ readApplicationArrayElement).
    fn read_application_array_data_type(
        &mut self,
        element: &Node,
        id: ApplicationArrayDataTypeId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let props = self.get_sw_data_def_props(element, "SW-DATA-DEF-PROPS", document)?;
        let dynamic_profile =
            get_child_element_string(element, "DYNAMIC-ARRAY-SIZE-PROFILE").map(str::to_string);
        // py readApplicationArrayElement — the ELEMENT child.
        let array_element = match find(element, "ELEMENT") {
            Some(element_node) => {
                let short_name = get_short_name(element_node)?;
                let element_id = document
                    .application_array_elements
                    .insert(ApplicationArrayElement::new());
                let element_payload = self.read_identifiable_payload(element_node, document)?;
                let props =
                    self.get_sw_data_def_props(element_node, "SW-DATA-DEF-PROPS", document)?;
                let type_t_ref = get_child_element_optional_t_ref_type(element_node, "TYPE-TREF");
                let handling = get_child_element_string(element_node, "ARRAY-SIZE-HANDLING")
                    .map(str::to_string);
                let semantics = get_child_element_string(element_node, "ARRAY-SIZE-SEMANTICS")
                    .map(str::to_string);
                let index_ref =
                    get_child_element_optional_ref_type(element_node, "INDEX-DATA-TYPE-REF");
                let max_elements = get_child_element_string(element_node, "MAX-NUMBER-OF-ELEMENTS")
                    .map(str::to_string);
                if let Some(array_element) = document.application_array_elements.get_mut(element_id)
                {
                    array_element.set_short_name(short_name);
                    if let Some(long_name) = element_payload.long_name {
                        array_element.set_long_name(long_name);
                    }
                    if let Some(desc) = element_payload.desc {
                        array_element.set_desc(desc);
                    }
                    if let Some(introduction) = element_payload.introduction {
                        array_element.set_introduction(introduction);
                    }
                    if let Some(admin_data) = element_payload.admin_data {
                        array_element.set_admin_data(admin_data);
                    }
                    if let Some(props) = props {
                        array_element.set_sw_data_def_props(props);
                    }
                    if let Some(type_t_ref) = type_t_ref {
                        let type_t_ref_id = document.t_ref_types.insert(type_t_ref);
                        array_element.set_type_t_ref(type_t_ref_id);
                    }
                    if let Some(handling) = handling {
                        array_element.set_array_size_handling(handling);
                    }
                    if let Some(semantics) = semantics {
                        array_element.set_array_size_semantics(semantics);
                    }
                    if let Some(index_ref) = index_ref {
                        let id = document.ref_types.insert(index_ref);
                        array_element.set_index_data_type_ref(id);
                    }
                    if let Some(max_elements) = max_elements {
                        array_element.set_max_number_of_elements(max_elements);
                    }
                }
                Some(element_id)
            }
            None => None,
        };
        if let Some(data_type) = document.application_array_data_types.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                data_type.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                data_type.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                data_type.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                data_type.set_admin_data(admin_data);
            }
            if let Some(props) = props {
                data_type.set_sw_data_def_props(props);
            }
            if let Some(profile) = dynamic_profile {
                data_type.set_dynamic_array_size_profile(profile);
            }
            if let Some(array_element) = array_element {
                data_type.set_element(array_element);
            }
        }
        Ok(())
    }

    /// py `readApplicationRecordDataType` (+ record ELEMENTS children).
    fn read_application_record_data_type(
        &mut self,
        element: &Node,
        id: ApplicationRecordDataTypeId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let props = self.get_sw_data_def_props(element, "SW-DATA-DEF-PROPS", document)?;
        let mut record_elements = Vec::new();
        for record_node in find_all(element, "ELEMENTS/APPLICATION-RECORD-ELEMENT") {
            let short_name = get_short_name(record_node)?;
            let record_id = document
                .application_record_elements
                .insert(ApplicationRecordElement::new());
            let element_payload = self.read_identifiable_payload(record_node, document)?;
            let element_props =
                self.get_sw_data_def_props(record_node, "SW-DATA-DEF-PROPS", document)?;
            let type_t_ref = get_child_element_optional_t_ref_type(record_node, "TYPE-TREF");
            let is_optional =
                get_child_element_string(record_node, "IS-OPTIONAL").map(str::to_string);
            if let Some(record_element) = document.application_record_elements.get_mut(record_id) {
                record_element.set_short_name(short_name);
                if let Some(long_name) = element_payload.long_name {
                    record_element.set_long_name(long_name);
                }
                if let Some(desc) = element_payload.desc {
                    record_element.set_desc(desc);
                }
                if let Some(introduction) = element_payload.introduction {
                    record_element.set_introduction(introduction);
                }
                if let Some(admin_data) = element_payload.admin_data {
                    record_element.set_admin_data(admin_data);
                }
                if let Some(props) = element_props {
                    record_element.set_sw_data_def_props(props);
                }
                if let Some(type_t_ref) = type_t_ref {
                    let type_t_ref_id = document.t_ref_types.insert(type_t_ref);
                    record_element.set_type_t_ref(type_t_ref_id);
                }
                if let Some(is_optional) = is_optional {
                    record_element.set_is_optional(is_optional);
                }
            }
            record_elements.push(record_id);
        }
        if let Some(data_type) = document.application_record_data_types.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                data_type.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                data_type.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                data_type.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                data_type.set_admin_data(admin_data);
            }
            if let Some(props) = props {
                data_type.set_sw_data_def_props(props);
            }
            for record_element in record_elements {
                data_type.push_record_element(record_element);
            }
        }
        Ok(())
    }

    /// py `readImplementationDataType` — the Identifiable chain, the array
    /// profile / struct flags and `TYPE-EMITTER`. The sub-element and symbol
    /// prop branches are deferred (no pinned fixture carries them; tracked on
    /// the port checklist).
    fn read_implementation_data_type(
        &mut self,
        element: &Node,
        id: ImplementationDataTypeId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let props = self.get_sw_data_def_props(element, "SW-DATA-DEF-PROPS", document)?;
        let dynamic_profile =
            get_child_element_string(element, "DYNAMIC-ARRAY-SIZE-PROFILE").map(str::to_string);
        let is_struct = get_child_element_string(element, "IS-STRUCT-WITH-OPTIONAL-ELEMENT")
            .map(str::to_string);
        let type_emitter = get_child_element_string(element, "TYPE-EMITTER").map(str::to_string);
        if let Some(data_type) = document.implementation_data_types.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                data_type.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                data_type.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                data_type.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                data_type.set_admin_data(admin_data);
            }
            if let Some(props) = props {
                data_type.set_sw_data_def_props(props);
            }
            if let Some(profile) = dynamic_profile {
                data_type.set_dynamic_array_size_profile(profile);
            }
            if let Some(is_struct) = is_struct {
                data_type.set_is_struct_with_optional_element(is_struct);
            }
            if let Some(type_emitter) = type_emitter {
                data_type.set_type_emitter(type_emitter);
            }
        }
        Ok(())
    }

    /// py `readSwBaseType`.
    fn read_sw_base_type(
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

    /// py `readCollection`.
    fn read_collection(
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

    /// py `readLifeCycleInfoSet`.
    fn read_life_cycle_info_set(
        &mut self,
        element: &Node,
        id: LifeCycleInfoSetId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let set_payload = self.read_life_cycle_info_set_payload(element, document)?;
        if let Some(info_set) = document.life_cycle_info_sets.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                info_set.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                info_set.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                info_set.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                info_set.set_admin_data(admin_data);
            }
            if let Some(ref_id) = set_payload.default_lc_state_ref {
                info_set.set_default_lc_state_ref(ref_id);
            }
            if let Some(period_id) = set_payload.default_period_begin {
                info_set.set_default_period_begin(period_id);
            }
            if let Some(period_id) = set_payload.default_period_end {
                info_set.set_default_period_end(period_id);
            }
            for info_id in set_payload.life_cycle_infos {
                info_set.push_life_cycle_info(info_id);
            }
            if let Some(ref_id) = set_payload.used_life_cycle_state_definition_group_ref {
                info_set.set_used_life_cycle_state_definition_group_ref(ref_id);
            }
        }
        Ok(())
    }

    /// py `readPhysicalDimension` — fixed child order, numerical text.
    fn read_physical_dimension(
        &mut self,
        element: &Node,
        id: PhysicalDimensionId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let length_exp = get_child_element_string(element, "LENGTH-EXP").map(str::to_string);
        let luminous_intensity_exp =
            get_child_element_string(element, "LUMINOUS-INTENSITY-EXP").map(str::to_string);
        let mass_exp = get_child_element_string(element, "MASS-EXP").map(str::to_string);
        let molar_amount_exp =
            get_child_element_string(element, "MOLAR-AMOUNT-EXP").map(str::to_string);
        let temperature_exp =
            get_child_element_string(element, "TEMPERATURE-EXP").map(str::to_string);
        let time_exp = get_child_element_string(element, "TIME-EXP").map(str::to_string);
        let current_exp = get_child_element_string(element, "CURRENT-EXP").map(str::to_string);

        if let Some(dimension) = document.physical_dimensions.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                dimension.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                dimension.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                dimension.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                dimension.set_admin_data(admin_data);
            }
            if let Some(value) = length_exp {
                dimension.set_length_exp(value);
            }
            if let Some(value) = luminous_intensity_exp {
                dimension.set_luminous_intensity_exp(value);
            }
            if let Some(value) = mass_exp {
                dimension.set_mass_exp(value);
            }
            if let Some(value) = molar_amount_exp {
                dimension.set_molar_amount_exp(value);
            }
            if let Some(value) = temperature_exp {
                dimension.set_temperature_exp(value);
            }
            if let Some(value) = time_exp {
                dimension.set_time_exp(value);
            }
            if let Some(value) = current_exp {
                dimension.set_current_exp(value);
            }
        }
        Ok(())
    }

    /// py `readUnit`.
    fn read_unit(
        &mut self,
        element: &Node,
        id: UnitId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        // py getSingleLanguageUnitNames — DISPLAY-NAME wrapper whose
        // text is the mixed string (SUP/SUB attrs and the wrapper's
        // own S/T never occur in this batch).
        let display_name = match find(element, "DISPLAY-NAME") {
            Some(node) => {
                let mut names = SingleLanguageUnitNames::new();
                if let Some(text) = &node.text {
                    names
                        .base_mut()
                        .atp_mixed_string_mut()
                        .set_mixed_string(text.as_str());
                }
                Some(document.single_language_unit_names.insert(names))
            }
            None => None,
        };
        let factor = get_child_element_string(element, "FACTOR-SI-TO-UNIT").map(str::to_string);
        let offset = get_child_element_string(element, "OFFSET-SI-TO-UNIT").map(str::to_string);
        let dimension_ref =
            match get_child_element_optional_ref_type(element, "PHYSICAL-DIMENSION-REF") {
                Some(r#ref) => Some(document.ref_types.insert(r#ref)),
                None => None,
            };

        if let Some(unit) = document.units.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                unit.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                unit.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                unit.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                unit.set_admin_data(admin_data);
            }
            if let Some(names_id) = display_name {
                unit.set_display_name(names_id);
            }
            if let Some(value) = factor {
                unit.set_factor_si_to_unit(value);
            }
            if let Some(value) = offset {
                unit.set_offset_si_to_unit(value);
            }
            if let Some(ref_id) = dimension_ref {
                unit.set_physical_dimension_ref(ref_id);
            }
        }
        Ok(())
    }

    /// py `getChildLimitElement` — a `<LOWER-LIMIT INTERVAL-TYPE="…">value
    /// </LOWER-LIMIT>` child as an arena-stored `Limit`.
    /// `allow(dead_code)` until Task 3 wires the Compu arms.
    #[allow(dead_code)]
    fn get_child_limit_element(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<LimitId>, ParseError> {
        let Some(limit_node) = find(element, key) else {
            return Ok(None);
        };
        let mut limit = Limit::new();
        if let Some(checksum) = limit_node.attrs.get("S") {
            limit.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = limit_node.attrs.get("T") {
            limit.set_timestamp(timestamp.as_str());
        }
        // py stores the attribute text verbatim (AREnum.setValue); fixtures
        // carry uppercase CLOSED against the enum's lowercase literal.
        if let Some(interval) = limit_node.attrs.get("INTERVAL-TYPE") {
            limit.set_interval_type(interval.as_str());
        }
        if let Some(text) = &limit_node.text {
            limit.set_value(text.as_str());
        }
        Ok(Some(document.limits.insert(limit)))
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

    /// py `getLifeCyclePeriod` — the wrapper node (e.g. `PERIOD-BEGIN`)
    /// becomes a `LifeCyclePeriod` arena entry. py does not call
    /// `readARObject` here, so S/T attributes are intentionally not read.
    fn get_life_cycle_period(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<LifeCyclePeriodId>, ParseError> {
        let Some(node) = find(element, key) else {
            return Ok(None);
        };
        let mut period = LifeCyclePeriod::new();
        if let Some(date) = get_child_element_string(node, "DATE") {
            period.set_date(date);
        }
        if let Some(version) = get_child_element_string(node, "AR-RELEASE-VERSION") {
            period.set_ar_release_version(version);
        }
        if let Some(release) = get_child_element_string(node, "PRODUCT-RELEASE") {
            period.set_product_release(release);
        }
        Ok(Some(document.life_cycle_periods.insert(period)))
    }

    /// py `readLifeCycleInfo` — returns the inserted arena id.
    fn read_life_cycle_info(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<LifeCycleInfoId, ParseError> {
        let mut info = LifeCycleInfo::new();
        self.read_ar_object(element, info.base_mut());
        if let Some(r#ref) = get_child_element_optional_ref_type(element, "LC-OBJECT-REF") {
            info.set_lc_object_ref(document.ref_types.insert(r#ref));
        }
        if let Some(r#ref) = get_child_element_optional_ref_type(element, "LC-STATE-REF") {
            info.set_lc_state_ref(document.ref_types.insert(r#ref));
        }
        if let Some(period_id) = self.get_life_cycle_period(element, "PERIOD-BEGIN", document)? {
            info.set_period_begin(period_id);
        }
        if let Some(period_id) = self.get_life_cycle_period(element, "PERIOD-END", document)? {
            info.set_period_end(period_id);
        }
        if let Some(remark_node) = find(element, "REMARK") {
            let remark = self.read_documentation_block(remark_node, document)?;
            info.set_remark(remark);
        }
        // py readLifeCycleInfoUseInsteadRefs
        for r#ref in get_child_element_ref_type_list(element, "USE-INSTEAD-REFS/USE-INSTEAD-REF") {
            info.push_use_instead_ref(document.ref_types.insert(r#ref));
        }
        Ok(document.life_cycle_infos.insert(info))
    }

    /// py `readLifeCycleInfoSet` — the family-specific part (the common
    /// Identifiable payload is read separately).
    fn read_life_cycle_info_set_payload(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<LifeCycleInfoSetPayload, ParseError> {
        let default_lc_state_ref =
            match get_child_element_optional_ref_type(element, "DEFAULT-LC-STATE-REF") {
                Some(r#ref) => Some(document.ref_types.insert(r#ref)),
                None => None,
            };
        let default_period_begin =
            self.get_life_cycle_period(element, "DEFAULT-PERIOD-BEGIN", document)?;
        let default_period_end =
            self.get_life_cycle_period(element, "DEFAULT-PERIOD-END", document)?;
        let mut life_cycle_infos = Vec::new();
        for child in find_all(element, "LIFE-CYCLE-INFOS/*") {
            if child.name == "LIFE-CYCLE-INFO" {
                life_cycle_infos.push(self.read_life_cycle_info(child, document)?);
            } else {
                let message = format!("Unsupported Life Cycle Info <{}>", child.name);
                self.not_implemented(message)?;
            }
        }
        let used_life_cycle_state_definition_group_ref = match get_child_element_optional_ref_type(
            element,
            "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF",
        ) {
            Some(r#ref) => Some(document.ref_types.insert(r#ref)),
            None => None,
        };
        Ok(LifeCycleInfoSetPayload {
            default_lc_state_ref,
            default_period_begin,
            default_period_end,
            life_cycle_infos,
            used_life_cycle_state_definition_group_ref,
        })
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
        ARXMLReader::new(default_options())
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
        let result = ARXMLReader::new(default_options()).load_from_reader(
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
        ARXMLReader::new(default_options())
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

    const REFERENCE_BASE_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00052.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <REFERENCE-BASES>
        <REFERENCE-BASE>
          <SHORT-LABEL>AUTOSAR</SHORT-LABEL>
          <IS-DEFAULT>false</IS-DEFAULT>
          <IS-GLOBAL>true</IS-GLOBAL>
          <BASE-IS-THIS-PACKAGE>false</BASE-IS-THIS-PACKAGE>
          <PACKAGE-REF DEST="AR-PACKAGE">/AUTOSAR/Platform</PACKAGE-REF>
        </REFERENCE-BASE>
      </REFERENCE-BASES>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    #[test]
    fn reference_bases_parse_into_the_arena() {
        let mut document = Document::new();
        ARXMLReader::new(default_options())
            .load_from_reader(Reader::from_str(REFERENCE_BASE_SAMPLE), &mut document)
            .unwrap();

        let pkg = document
            .get_ar_package(document.get_ar_packages()[0])
            .unwrap();
        let base_id = pkg.get_reference_bases()[0];
        let base = document.reference_bases.get(base_id).unwrap();
        assert_eq!(base.get_short_label(), Some("AUTOSAR"));
        assert_eq!(base.get_is_default(), Some(false));
        assert_eq!(base.get_is_global(), Some(true));
        assert_eq!(base.get_base_is_this_package(), Some(false));
        let package_ref = document
            .ref_types
            .get(base.get_package_ref().unwrap())
            .unwrap();
        assert_eq!(package_ref.get_dest(), Some("AR-PACKAGE"));
        assert_eq!(package_ref.get_value(), Some("/AUTOSAR/Platform"));
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
