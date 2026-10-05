//! Application/Implementation datatype emitters incl. the shared
//! SW-DATA-DEF-PROPS wrapper. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::common_structure::implementation_data_types::ImplementationDataTypeId;
use crate::m2::autosar_templates::sw_component_template::datatype::datatypes::{
    ApplicationArrayDataTypeId, ApplicationPrimitiveDataTypeId, ApplicationRecordDataTypeId,
};
use crate::m2::msr::data_dictionary::data_def_properties::SwDataDefProps;

impl ARXMLWriter {
    /// py `setSwDataDefProps` — wrapper + VARIANTS + CONDITIONAL; the
    /// model-backed fields in py's exact emission order.
    fn set_sw_data_def_props<W: Write>(
        &self,
        writer: &mut Writer<W>,
        props: &SwDataDefProps,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut wrapper = BytesStart::new("SW-DATA-DEF-PROPS");
        self.write_ar_object_attributes(&mut wrapper, props.base());
        writer.write_event(Event::Start(wrapper))?;
        writer.write_event(Event::Start(BytesStart::new("SW-DATA-DEF-PROPS-VARIANTS")))?;
        let mut conditional = BytesStart::new("SW-DATA-DEF-PROPS-CONDITIONAL");
        self.write_ar_object_attributes(&mut conditional, props.base());
        writer.write_event(Event::Start(conditional))?;
        if let Some(display) = props.get_display_presentation() {
            write_text_element(
                writer,
                "DISPLAY-PRESENTATION",
                BytesStart::new("DISPLAY-PRESENTATION"),
                Some(display.as_str()),
            )?;
        }
        write_optional_ref_type(
            writer,
            "BASE-TYPE-REF",
            props
                .get_base_type_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        write_optional_ref_type(
            writer,
            "SW-ADDR-METHOD-REF",
            props
                .get_sw_addr_method_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        write_optional_text_element(writer, "SW-ALIGNMENT", props.get_sw_alignment())?;
        write_optional_text_element(
            writer,
            "SW-CALIBRATION-ACCESS",
            props.get_sw_calibration_access(),
        )?;
        write_optional_ref_type(
            writer,
            "COMPU-METHOD-REF",
            props
                .get_compu_method_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        write_optional_text_element(writer, "STEP-SIZE", props.get_step_size())?;
        write_optional_ref_type(
            writer,
            "DATA-CONSTR-REF",
            props
                .get_data_constr_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        write_optional_ref_type(
            writer,
            "IMPLEMENTATION-DATA-TYPE-REF",
            props
                .get_implementation_data_type_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        write_optional_text_element(
            writer,
            "SW-INTENDED-RESOLUTION",
            props.get_sw_intended_resolution(),
        )?;
        write_optional_ref_type(
            writer,
            "UNIT-REF",
            props.get_unit_ref().and_then(|r| document.ref_types.get(r)),
        )?;
        write_optional_text_element(writer, "DISPLAY-FORMAT", props.get_display_format())?;
        writer.write_event(Event::End(BytesEnd::new("SW-DATA-DEF-PROPS-CONDITIONAL")))?;
        writer.write_event(Event::End(BytesEnd::new("SW-DATA-DEF-PROPS-VARIANTS")))?;
        writer.write_event(Event::End(BytesEnd::new("SW-DATA-DEF-PROPS")))?;
        Ok(())
    }

    /// py `writeAutosarDataType` — the Identifiable chain followed by
    /// SW-DATA-DEF-PROPS.
    fn write_autosar_data_type_parts<W: Write>(
        &self,
        writer: &mut Writer<W>,
        parts: IdentifiableParts<'_>,
        document: &Document,
    ) -> Result<(), WriteError> {
        let sw_data_def_props = parts.sw_data_def_props;
        self.write_identifiable_parts(writer, parts, document)?;
        if let Some(props) = sw_data_def_props.and_then(|id| document.sw_data_def_props.get(id)) {
            self.set_sw_data_def_props(writer, props, document)?;
        }
        Ok(())
    }

    /// py `writeApplicationPrimitiveDataType`.
    pub(super) fn write_application_primitive_data_type<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: ApplicationPrimitiveDataTypeId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(data_type) = document.application_primitive_data_types.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("APPLICATION-PRIMITIVE-DATA-TYPE");
        self.write_identifiable_attributes(
            &mut element,
            data_type.get_checksum(),
            data_type.get_timestamp(),
            data_type.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = data_type.get_short_name() {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_autosar_data_type_parts(
            writer,
            IdentifiableParts {
                long_name: data_type.get_long_name(),
                desc: data_type.get_desc(),
                category: data_type.get_category(),
                introduction: data_type.get_introduction(),
                admin_data: data_type.get_admin_data(),
                sw_data_def_props: data_type.get_sw_data_def_props(),
            },
            document,
        )?;
        writer.write_event(Event::End(BytesEnd::new("APPLICATION-PRIMITIVE-DATA-TYPE")))?;
        Ok(())
    }

    /// py `writeApplicationCompositeElementDataPrototype` — the shared
    /// prototype body: chain + props + TYPE-TREF.
    fn write_composite_element_prototype_body<W: Write>(
        &self,
        writer: &mut Writer<W>,
        short_name: Option<&str>,
        parts: IdentifiableParts<'_>,
        type_t_ref: Option<crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::TRefTypeId>,
        document: &Document,
    ) -> Result<(), WriteError> {
        if let Some(short_name) = short_name {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_autosar_data_type_parts(writer, parts, document)?;
        write_optional_t_ref_type(
            writer,
            "TYPE-TREF",
            type_t_ref.and_then(|r| document.t_ref_types.get(r)),
        )?;
        Ok(())
    }

    /// py `writeApplicationArrayDataType` (+ setApplicationArrayElement).
    pub(super) fn write_application_array_data_type<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: ApplicationArrayDataTypeId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(data_type) = document.application_array_data_types.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("APPLICATION-ARRAY-DATA-TYPE");
        self.write_identifiable_attributes(
            &mut element,
            data_type.get_checksum(),
            data_type.get_timestamp(),
            data_type.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = data_type.get_short_name() {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_autosar_data_type_parts(
            writer,
            IdentifiableParts {
                long_name: data_type.get_long_name(),
                desc: data_type.get_desc(),
                category: data_type.get_category(),
                introduction: data_type.get_introduction(),
                admin_data: data_type.get_admin_data(),
                sw_data_def_props: data_type.get_sw_data_def_props(),
            },
            document,
        )?;
        write_optional_text_element(
            writer,
            "DYNAMIC-ARRAY-SIZE-PROFILE",
            data_type.get_dynamic_array_size_profile(),
        )?;
        if let Some(array_element) = data_type
            .get_element()
            .and_then(|e| document.application_array_elements.get(e))
        {
            // py setApplicationArrayElement — ELEMENT wrapper.
            let mut element_wrapper = BytesStart::new("ELEMENT");
            self.write_identifiable_attributes(
                &mut element_wrapper,
                array_element.get_checksum(),
                array_element.get_timestamp(),
                array_element.get_uuid(),
            );
            writer.write_event(Event::Start(element_wrapper))?;
            self.write_composite_element_prototype_body(
                writer,
                array_element.get_short_name(),
                IdentifiableParts {
                    long_name: array_element.get_long_name(),
                    desc: array_element.get_desc(),
                    category: array_element.get_category(),
                    introduction: array_element.get_introduction(),
                    admin_data: array_element.get_admin_data(),
                    sw_data_def_props: array_element.get_sw_data_def_props(),
                },
                array_element.get_type_t_ref(),
                document,
            )?;
            write_optional_text_element(
                writer,
                "ARRAY-SIZE-HANDLING",
                array_element.get_array_size_handling(),
            )?;
            write_optional_text_element(
                writer,
                "ARRAY-SIZE-SEMANTICS",
                array_element.get_array_size_semantics(),
            )?;
            write_optional_ref_type(
                writer,
                "INDEX-DATA-TYPE-REF",
                array_element
                    .get_index_data_type_ref()
                    .and_then(|r| document.ref_types.get(r)),
            )?;
            write_optional_text_element(
                writer,
                "MAX-NUMBER-OF-ELEMENTS",
                array_element.get_max_number_of_elements(),
            )?;
            writer.write_event(Event::End(BytesEnd::new("ELEMENT")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("APPLICATION-ARRAY-DATA-TYPE")))?;
        Ok(())
    }

    /// py `writeApplicationRecordDataType` (+ record ELEMENTS children).
    pub(super) fn write_application_record_data_type<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: ApplicationRecordDataTypeId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(data_type) = document.application_record_data_types.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("APPLICATION-RECORD-DATA-TYPE");
        self.write_identifiable_attributes(
            &mut element,
            data_type.get_checksum(),
            data_type.get_timestamp(),
            data_type.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = data_type.get_short_name() {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_autosar_data_type_parts(
            writer,
            IdentifiableParts {
                long_name: data_type.get_long_name(),
                desc: data_type.get_desc(),
                category: data_type.get_category(),
                introduction: data_type.get_introduction(),
                admin_data: data_type.get_admin_data(),
                sw_data_def_props: data_type.get_sw_data_def_props(),
            },
            document,
        )?;
        let record_elements = data_type.get_record_elements();
        if !record_elements.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("ELEMENTS")))?;
            for record_id in record_elements {
                if let Some(record_element) = document.application_record_elements.get(*record_id) {
                    // py writeApplicationRecordElement
                    let mut record_wrapper = BytesStart::new("APPLICATION-RECORD-ELEMENT");
                    self.write_identifiable_attributes(
                        &mut record_wrapper,
                        record_element.get_checksum(),
                        record_element.get_timestamp(),
                        record_element.get_uuid(),
                    );
                    writer.write_event(Event::Start(record_wrapper))?;
                    self.write_composite_element_prototype_body(
                        writer,
                        record_element.get_short_name(),
                        IdentifiableParts {
                            long_name: record_element.get_long_name(),
                            desc: record_element.get_desc(),
                            category: record_element.get_category(),
                            introduction: record_element.get_introduction(),
                            admin_data: record_element.get_admin_data(),
                            sw_data_def_props: record_element.get_sw_data_def_props(),
                        },
                        record_element.get_type_t_ref(),
                        document,
                    )?;
                    write_optional_text_element(
                        writer,
                        "IS-OPTIONAL",
                        record_element.get_is_optional(),
                    )?;
                    writer.write_event(Event::End(BytesEnd::new("APPLICATION-RECORD-ELEMENT")))?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("ELEMENTS")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("APPLICATION-RECORD-DATA-TYPE")))?;
        Ok(())
    }

    /// py `writeImplementationDataType` — chain, array-profile/struct flags,
    /// TYPE-EMITTER (sub-elements/symbol props deferred: no fixture).
    pub(super) fn write_implementation_data_type<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: ImplementationDataTypeId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(data_type) = document.implementation_data_types.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("IMPLEMENTATION-DATA-TYPE");
        self.write_identifiable_attributes(
            &mut element,
            data_type.get_checksum(),
            data_type.get_timestamp(),
            data_type.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = data_type.get_short_name() {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_autosar_data_type_parts(
            writer,
            IdentifiableParts {
                long_name: data_type.get_long_name(),
                desc: data_type.get_desc(),
                category: data_type.get_category(),
                introduction: data_type.get_introduction(),
                admin_data: data_type.get_admin_data(),
                sw_data_def_props: data_type.get_sw_data_def_props(),
            },
            document,
        )?;
        write_optional_text_element(
            writer,
            "DYNAMIC-ARRAY-SIZE-PROFILE",
            data_type.get_dynamic_array_size_profile(),
        )?;
        write_optional_text_element(
            writer,
            "IS-STRUCT-WITH-OPTIONAL-ELEMENT",
            data_type.get_is_struct_with_optional_element(),
        )?;
        write_optional_text_element(writer, "TYPE-EMITTER", data_type.get_type_emitter())?;
        writer.write_event(Event::End(BytesEnd::new("IMPLEMENTATION-DATA-TYPE")))?;
        Ok(())
    }
}
