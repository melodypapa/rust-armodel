//! LifeCycleInfoSet emitters. Part of the arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::generic_structure::life_cycles::{
    LifeCycleInfoId, LifeCycleInfoSetId, LifeCyclePeriod,
};

impl ARXMLWriter {
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
    pub(super) fn write_life_cycle_info_set<W: Write>(
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
}
