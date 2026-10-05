//! Unit readers (py readUnit, with the DISPLAY-NAME wrapper). Part of the
//! arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::units::{SingleLanguageUnitNames, UnitId};

impl ARXMLReader {
    /// py `readUnit`.
    pub(super) fn read_unit(
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
}
