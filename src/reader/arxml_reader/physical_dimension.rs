//! PhysicalDimension readers (py readPhysicalDimension). Part of the
//! arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::units::PhysicalDimensionId;

impl ARXMLReader {
    /// py `readPhysicalDimension` — fixed child order, numerical text.
    pub(super) fn read_physical_dimension(
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
}
