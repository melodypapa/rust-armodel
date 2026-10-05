//! Unit emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::units::UnitId;

impl ARXMLWriter {
    /// py `writeUnit` — DISPLAY-NAME wrapper, then factor/offset and the
    /// physical-dimension ref after the Identifiable parts.
    pub(super) fn write_unit<W: Write>(
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
