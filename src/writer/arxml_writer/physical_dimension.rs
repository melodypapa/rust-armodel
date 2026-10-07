//! PhysicalDimension emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::units::PhysicalDimensionId;

impl ARXMLWriter {
    /// py `writePhysicalDimension` — the ARElement chain level, then the seven
    /// numerical children in fixed order.
    pub(super) fn write_physical_dimension<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: PhysicalDimensionId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(dimension) = document.physical_dimensions.get(id) else {
            return Ok(());
        };
        self.write_ar_element(
            writer,
            BytesStart::new("PHYSICAL-DIMENSION"),
            IdentifiableAttrs {
                checksum: dimension.get_checksum(),
                timestamp: dimension.get_timestamp(),
                uuid: dimension.get_uuid(),
                short_name: dimension.get_short_name(),
            },
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
}
