//! DATA-CONSTR emitters (py writeDataConstr family; writer rule order is
//! CONSTR-LEVEL, PHYS-CONSTRS, INTERNAL-CONSTRS). Part of the arxml_writer
//! domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::constraints::global_constraints::{
    DataConstrId, DataConstrRule, InternalConstrs, PhysConstrs,
};

impl ARXMLWriter {
    /// py `writeDataConstr`.
    pub(super) fn write_data_constr<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: DataConstrId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(data_constr) = document.data_constrs.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("DATA-CONSTR");
        self.write_identifiable_attributes(
            &mut element,
            data_constr.get_checksum(),
            data_constr.get_timestamp(),
            data_constr.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = data_constr.get_short_name() {
            write_text_element(
                writer,
                "SHORT-NAME",
                BytesStart::new("SHORT-NAME"),
                Some(short_name),
            )?;
        }
        self.write_identifiable_parts(
            writer,
            IdentifiableParts {
                long_name: data_constr.get_long_name(),
                desc: data_constr.get_desc(),
                category: data_constr.get_category(),
                introduction: data_constr.get_introduction(),
                admin_data: data_constr.get_admin_data(),
                sw_data_def_props: None,
            },
            document,
        )?;
        // py writeDataConstrRules — wrapper only when non-empty
        let rules = data_constr.get_data_constr_rule();
        if !rules.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("DATA-CONSTR-RULES")))?;
            for rule_id in rules {
                if let Some(rule) = document.data_constr_rules.get(*rule_id) {
                    self.write_data_constr_rule(writer, rule, document)?;
                }
            }
            writer.write_event(Event::End(BytesEnd::new("DATA-CONSTR-RULES")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("DATA-CONSTR")))?;
        Ok(())
    }

    /// py `writeDataConstrRules` per-rule body — CONSTR-LEVEL, then
    /// PHYS-CONSTRS, then INTERNAL-CONSTRS (writer order differs from the
    /// reader's).
    fn write_data_constr_rule<W: Write>(
        &self,
        writer: &mut Writer<W>,
        rule: &DataConstrRule,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("DATA-CONSTR-RULE");
        self.write_ar_object_attributes(&mut element, rule.base());
        writer.write_event(Event::Start(element))?;
        write_optional_text_element(writer, "CONSTR-LEVEL", rule.get_constr_level())?;
        if let Some(constrs) = rule
            .get_phys_constrs()
            .and_then(|c| document.phys_constrs.get(c))
        {
            self.write_phys_constrs(writer, constrs, document)?;
        }
        if let Some(constrs) = rule
            .get_internal_constrs()
            .and_then(|c| document.internal_constrs.get(c))
        {
            self.write_internal_constrs(writer, constrs, document)?;
        }
        writer.write_event(Event::End(BytesEnd::new("DATA-CONSTR-RULE")))?;
        Ok(())
    }

    /// py `setInternalConstrs`.
    fn write_internal_constrs<W: Write>(
        &self,
        writer: &mut Writer<W>,
        constrs: &InternalConstrs,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("INTERNAL-CONSTRS");
        self.write_ar_object_attributes(&mut element, constrs.base());
        writer.write_event(Event::Start(element))?;
        if let Some(limit) = constrs
            .get_lower_limit()
            .and_then(|l| document.limits.get(l))
        {
            write_limit_element(writer, "LOWER-LIMIT", limit)?;
        }
        if let Some(limit) = constrs
            .get_upper_limit()
            .and_then(|l| document.limits.get(l))
        {
            write_limit_element(writer, "UPPER-LIMIT", limit)?;
        }
        self.write_scale_constrs(
            writer,
            "SCALE-CONSTRS",
            constrs.get_scale_constrs(),
            document,
        )?;
        write_optional_text_element(writer, "MAX-GRADIENT", constrs.get_max_gradient())?;
        write_optional_text_element(writer, "MAX-DIFF", constrs.get_max_diff())?;
        if let Some(monotony) = constrs.get_monotony() {
            write_text_element(
                writer,
                "MONOTONY",
                BytesStart::new("MONOTONY"),
                Some(monotony.as_str()),
            )?;
        }
        writer.write_event(Event::End(BytesEnd::new("INTERNAL-CONSTRS")))?;
        Ok(())
    }

    /// py `setPhysConstrs`.
    fn write_phys_constrs<W: Write>(
        &self,
        writer: &mut Writer<W>,
        constrs: &PhysConstrs,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new("PHYS-CONSTRS");
        self.write_ar_object_attributes(&mut element, constrs.base());
        writer.write_event(Event::Start(element))?;
        if let Some(limit) = constrs
            .get_lower_limit()
            .and_then(|l| document.limits.get(l))
        {
            write_limit_element(writer, "LOWER-LIMIT", limit)?;
        }
        if let Some(limit) = constrs
            .get_upper_limit()
            .and_then(|l| document.limits.get(l))
        {
            write_limit_element(writer, "UPPER-LIMIT", limit)?;
        }
        self.write_scale_constrs(
            writer,
            "SCALE-CONSTRS",
            constrs.get_scale_constrs(),
            document,
        )?;
        write_optional_text_element(writer, "MAX-GRADIENT", constrs.get_max_gradient())?;
        write_optional_text_element(writer, "MAX-DIFF", constrs.get_max_diff())?;
        if let Some(monotony) = constrs.get_monotony() {
            write_text_element(
                writer,
                "MONOTONY",
                BytesStart::new("MONOTONY"),
                Some(monotony.as_str()),
            )?;
        }
        write_optional_ref_type(
            writer,
            "UNIT-REF",
            constrs
                .get_unit_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        writer.write_event(Event::End(BytesEnd::new("PHYS-CONSTRS")))?;
        Ok(())
    }

    /// py's SCALE-CONSTRS wrapper + `setScaleConstr` per item.
    fn write_scale_constrs<W: Write>(
        &self,
        writer: &mut Writer<W>,
        wrapper: &str,
        scale_constrs: &[crate::m2::msr::asam_hdo::constraints::global_constraints::ScaleConstrId],
        document: &Document,
    ) -> Result<(), WriteError> {
        if scale_constrs.is_empty() {
            return Ok(());
        }
        writer.write_event(Event::Start(BytesStart::new(wrapper)))?;
        for sc_id in scale_constrs {
            if let Some(scale_constr) = document.scale_constrs.get(*sc_id) {
                let mut element = BytesStart::new("SCALE-CONSTR");
                self.write_ar_object_attributes(&mut element, scale_constr.base());
                if let Some(validity) = scale_constr.get_validity() {
                    element.push_attribute(("VALIDITY", validity.as_str()));
                }
                writer.write_event(Event::Start(element))?;
                write_optional_text_element(writer, "SHORT-LABEL", scale_constr.get_short_label())?;
                if let Some(desc) = scale_constr
                    .get_desc()
                    .and_then(|id| document.multi_language_overview_paragraphs.get(id))
                {
                    self.set_multi_language_overview_paragraph(writer, desc, document)?;
                }
                if let Some(limit) = scale_constr
                    .get_lower_limit()
                    .and_then(|l| document.limits.get(l))
                {
                    write_limit_element(writer, "LOWER-LIMIT", limit)?;
                }
                if let Some(limit) = scale_constr
                    .get_upper_limit()
                    .and_then(|l| document.limits.get(l))
                {
                    write_limit_element(writer, "UPPER-LIMIT", limit)?;
                }
                writer.write_event(Event::End(BytesEnd::new("SCALE-CONSTR")))?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new(wrapper)))?;
        Ok(())
    }
}
