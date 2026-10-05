//! `ARXMLReader::load` with release detection, `read_ar_packages`
//! (P0 design §6). Mirrors py's `arxml_parser.py` method-for-method so
//! the P2–P4 port stays reviewable.

use std::io::BufRead;
use std::path::Path;

use quick_xml::reader::Reader;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
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
    ByteOrderEnum, RefTypeId,
};
use crate::m2::autosar_templates::generic_structure::life_cycles::{
    LifeCycleInfo, LifeCycleInfoId, LifeCycleInfoSetId, LifeCyclePeriod, LifeCyclePeriodId,
};
use crate::m2::element_registry::{
    element_factory_for_tag, element_set_category, element_set_checksum, element_set_timestamp,
    element_set_uuid,
};
use crate::m2::msr::asam_hdo::admin_data::{AdminData, AdminDataId};
use crate::m2::msr::asam_hdo::base_types::{BaseTypeDirectDefinition, SwBaseTypeId};
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
    get_child_element_ref_type_list,
    get_child_element_string,
    get_short_name,
};

mod common;

mod documentation;

mod admin_data;

mod compu_method;

mod data_constr;

mod keyword;

mod datatypes;

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
