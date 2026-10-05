//! `ARXMLWriter::save` emitting the XML declaration, `AUTOSAR` with
//! namespaces, `ADMIN-DATA`, `AR-PACKAGES` (P0 design §9 step 5).
//! Mirrors py's `arxml_writer.py`.

use std::io::Write;
use std::path::Path;

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::writer::Writer;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::msr::data_dictionary::data_def_properties::SwDataDefPropsId;
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
use crate::m2::msr::asam_hdo::admin_data::AdminDataId;
use crate::m2::msr::asam_hdo::base_types::SwBaseTypeId;
use crate::m2::msr::asam_hdo::units::{PhysicalDimensionId, UnitId};
use crate::m2::msr::documentation::text_model::block_elements::DocumentationBlockId;
use crate::m2::msr::documentation::text_model::multilanguage_data::{
    MultiLanguageOverviewParagraphId, MultilanguageLongNameId,
};
use crate::writer::abstract_arxml_writer::{
    WriteError,
    write_limit_element,
    write_optional_boolean_element,
    write_optional_ref_type,
    write_optional_t_ref_type,
    write_optional_text_element,
    write_ref_type_list,
    write_text_element,
};

mod admin_data;
mod common;
mod compu_method;
mod data_constr;
mod datatypes;
mod documentation;
mod keyword;

/// The per-class `Identifiable` payload pieces a family emitter hands to
/// `write_identifiable_parts` (ids resolve through the `Document` arenas).
struct IdentifiableParts<'a> {
    long_name: Option<MultilanguageLongNameId>,
    desc: Option<MultiLanguageOverviewParagraphId>,
    category: Option<&'a str>,
    introduction: Option<DocumentationBlockId>,
    admin_data: Option<AdminDataId>,
    /// The `AutosarDataType` tail (py `writeAutosarDataType`): emitted after
    /// the Identifiable chain. `None` for non-datatype elements.
    sw_data_def_props: Option<SwDataDefPropsId>,
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
                sw_data_def_props: None,
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
            ElementRef::KeywordSet(id) => {
                return self.write_keyword_set(writer, id, document);
            }
            ElementRef::ApplicationPrimitiveDataType(id) => {
                return self.write_application_primitive_data_type(writer, id, document);
            }
            ElementRef::ApplicationArrayDataType(id) => {
                return self.write_application_array_data_type(writer, id, document);
            }
            ElementRef::ApplicationRecordDataType(id) => {
                return self.write_application_record_data_type(writer, id, document);
            }
            ElementRef::ImplementationDataType(id) => {
                return self.write_implementation_data_type(writer, id, document);
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
                sw_data_def_props: None,
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
                sw_data_def_props: None,
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
                sw_data_def_props: None,
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
    use crate::m2::msr::asam_hdo::admin_data::AdminData;
    use crate::m2::msr::asam_hdo::special_data::{Sd, Sdg, SdgContents};
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
