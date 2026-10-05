//! Application/Implementation datatype readers incl. the shared
//! SW-DATA-DEF-PROPS wrapper. Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::common_structure::implementation_data_types::ImplementationDataTypeId;
use crate::m2::autosar_templates::sw_component_template::datatype::data_prototypes::{
    ApplicationArrayElement, ApplicationRecordElement,
};
use crate::m2::autosar_templates::sw_component_template::datatype::datatypes::{
    ApplicationArrayDataTypeId, ApplicationPrimitiveDataTypeId, ApplicationRecordDataTypeId,
};
use crate::m2::msr::data_dictionary::data_def_properties::{
    DisplayPresentationEnum, SwDataDefProps, SwDataDefPropsId,
};
use crate::reader::abstract_arxml_reader::get_child_element_optional_t_ref_type;

impl ARXMLReader {
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
    pub(super) fn read_application_primitive_data_type(
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
    pub(super) fn read_application_array_data_type(
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
    pub(super) fn read_application_record_data_type(
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
    pub(super) fn read_implementation_data_type(
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
}
