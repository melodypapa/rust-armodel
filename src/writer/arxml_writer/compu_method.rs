//! COMPU-METHOD emitters (py writeCompuMethod / setCompu family). Part of the
//! arxml_writer domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::computation_method::{
    Compu, CompuConst, CompuConstContentRef, CompuMethodId, CompuScaleContentsRef,
};

impl ARXMLWriter {
    /// py `writeCompuMethod`.
    pub(super) fn write_compu_method<W: Write>(
        &self,
        writer: &mut Writer<W>,
        id: CompuMethodId,
        document: &Document,
    ) -> Result<(), WriteError> {
        let Some(compu_method) = document.compu_methods.get(id) else {
            return Ok(());
        };
        let mut element = BytesStart::new("COMPU-METHOD");
        self.write_identifiable_attributes(
            &mut element,
            compu_method.get_checksum(),
            compu_method.get_timestamp(),
            compu_method.get_uuid(),
        );
        writer.write_event(Event::Start(element))?;
        if let Some(short_name) = compu_method.get_short_name() {
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
                long_name: compu_method.get_long_name(),
                desc: compu_method.get_desc(),
                category: compu_method.get_category(),
                introduction: compu_method.get_introduction(),
                admin_data: compu_method.get_admin_data(),
                sw_data_def_props: None,
            },
            document,
        )?;
        write_optional_text_element(writer, "DISPLAY-FORMAT", compu_method.get_display_format())?;
        write_optional_ref_type(
            writer,
            "UNIT-REF",
            compu_method
                .get_unit_ref()
                .and_then(|r| document.ref_types.get(r)),
        )?;
        if let Some(internal) = compu_method
            .get_compu_internal_to_phys()
            .and_then(|c| document.compus.get(c))
        {
            self.set_compu(writer, "COMPU-INTERNAL-TO-PHYS", internal, document)?;
        }
        if let Some(phys) = compu_method
            .get_compu_phys_to_internal()
            .and_then(|c| document.compus.get(c))
        {
            self.set_compu(writer, "COMPU-PHYS-TO-INTERNAL", phys, document)?;
        }
        writer.write_event(Event::End(BytesEnd::new("COMPU-METHOD")))?;
        Ok(())
    }

    /// py `setCompu` — S/T attrs, COMPU-SCALES, COMPU-DEFAULT-VALUE.
    fn set_compu<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        compu: &Compu,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new(key);
        self.write_ar_object_attributes(&mut element, compu.base());
        writer.write_event(Event::Start(element))?;
        if let Some(crate::m2::msr::asam_hdo::computation_method::CompuContentRef::CompuScales(
            scales_id,
        )) = compu.get_compu_content()
        {
            if let Some(scales) = document.compu_scales_arena.get(scales_id) {
                self.set_compu_scales(writer, scales, document)?;
            }
        }
        if let Some(default) = compu
            .get_compu_default_value()
            .and_then(|c| document.compu_consts.get(c))
        {
            self.set_compu_const(writer, "COMPU-DEFAULT-VALUE", default, document)?;
        }
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    /// py `setCompuScales` — wrapper (no attrs) + per-scale bodies.
    fn set_compu_scales<W: Write>(
        &self,
        writer: &mut Writer<W>,
        scales: &crate::m2::msr::asam_hdo::computation_method::CompuScales,
        document: &Document,
    ) -> Result<(), WriteError> {
        writer.write_event(Event::Start(BytesStart::new("COMPU-SCALES")))?;
        for scale_id in scales.get_compu_scales() {
            if let Some(scale) = document.compu_scales.get(*scale_id) {
                self.write_compu_scale(writer, "COMPU-SCALE", scale, document)?;
            }
        }
        writer.write_event(Event::End(BytesEnd::new("COMPU-SCALES")))?;
        Ok(())
    }

    /// py `writeCompuScale` — fixed child order per the py body.
    fn write_compu_scale<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        scale: &crate::m2::msr::asam_hdo::computation_method::CompuScale,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new(key);
        self.write_ar_object_attributes(&mut element, scale.base());
        writer.write_event(Event::Start(element))?;
        write_optional_text_element(writer, "A2L-DISPLAY-TEXT", scale.get_a2l_display_text())?;
        if let Some(inverse) = scale
            .get_compu_inverse_value()
            .and_then(|c| document.compu_consts.get(c))
        {
            self.set_compu_const(writer, "COMPU-INVERSE-VALUE", inverse, document)?;
        }
        write_optional_text_element(writer, "SHORT-LABEL", scale.get_short_label())?;
        write_optional_text_element(writer, "SYMBOL", scale.get_symbol())?;
        if let Some(desc) = scale
            .get_desc()
            .and_then(|id| document.multi_language_overview_paragraphs.get(id))
        {
            self.set_multi_language_overview_paragraph(writer, desc, document)?;
        }
        write_optional_text_element(writer, "MASK", scale.get_mask())?;
        if let Some(limit) = scale.get_lower_limit().and_then(|l| document.limits.get(l)) {
            write_limit_element(writer, "LOWER-LIMIT", limit)?;
        }
        if let Some(limit) = scale.get_upper_limit().and_then(|l| document.limits.get(l)) {
            write_limit_element(writer, "UPPER-LIMIT", limit)?;
        }
        match scale.get_compu_scale_contents() {
            Some(CompuScaleContentsRef::CompuScaleConstantContents(id)) => {
                if let Some(constant) = document.compu_scale_constant_contents.get(id) {
                    // py writeCompuScaleConstantContents: bare COMPU-CONST +
                    // VT literal child (no ARObject attrs on either).
                    writer.write_event(Event::Start(BytesStart::new("COMPU-CONST")))?;
                    if let Some(const_id) = constant.get_compu_const() {
                        if let Some(const_) = document.compu_consts.get(const_id) {
                            if let Some(CompuConstContentRef::CompuConstTextContent(text_id)) =
                                const_.get_compu_const_content_type()
                            {
                                if let Some(text) = document.compu_const_text_contents.get(text_id)
                                {
                                    write_optional_text_element(writer, "VT", text.get_vt())?;
                                }
                            }
                        }
                    }
                    writer.write_event(Event::End(BytesEnd::new("COMPU-CONST")))?;
                }
            }
            Some(CompuScaleContentsRef::CompuScaleRationalFormula(id)) => {
                if let Some(formula) = document.compu_scale_rational_formulas.get(id) {
                    writer.write_event(Event::Start(BytesStart::new("COMPU-RATIONAL-COEFFS")))?;
                    if let Some(coeffs) = formula
                        .get_compu_rational_coeffs()
                        .and_then(|c| document.compu_rational_coeffs.get(c))
                    {
                        if let Some(numerator) = coeffs
                            .get_compu_numerator()
                            .and_then(|n| document.compu_nominator_denominators.get(n))
                        {
                            self.write_nominator_denominator(writer, "COMPU-NUMERATOR", numerator)?;
                        }
                        if let Some(denominator) = coeffs
                            .get_compu_denominator()
                            .and_then(|d| document.compu_nominator_denominators.get(d))
                        {
                            self.write_nominator_denominator(
                                writer,
                                "COMPU-DENOMINATOR",
                                denominator,
                            )?;
                        }
                    }
                    writer.write_event(Event::End(BytesEnd::new("COMPU-RATIONAL-COEFFS")))?;
                }
            }
            None => {}
        }
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    /// py `writeCompuNominatorDenominator` — V children in list order.
    fn write_nominator_denominator<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        nom_denom: &crate::m2::msr::asam_hdo::computation_method::CompuNominatorDenominator,
    ) -> Result<(), WriteError> {
        writer.write_event(Event::Start(BytesStart::new(key)))?;
        for v in nom_denom.get_v() {
            write_text_element(writer, "V", BytesStart::new("V"), Some(v))?;
        }
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }

    /// py `setCompuConst` — S/T attrs + content child (VT/V/VF).
    fn set_compu_const<W: Write>(
        &self,
        writer: &mut Writer<W>,
        key: &str,
        compu_const: &CompuConst,
        document: &Document,
    ) -> Result<(), WriteError> {
        let mut element = BytesStart::new(key);
        self.write_ar_object_attributes(&mut element, compu_const.base());
        writer.write_event(Event::Start(element))?;
        match compu_const.get_compu_const_content_type() {
            Some(CompuConstContentRef::CompuConstTextContent(id)) => {
                if let Some(text) = document.compu_const_text_contents.get(id) {
                    write_optional_text_element(writer, "VT", text.get_vt())?;
                }
            }
            Some(CompuConstContentRef::CompuConstNumericContent(id)) => {
                if let Some(numeric) = document.compu_const_numeric_contents.get(id) {
                    write_optional_text_element(writer, "V", numeric.get_v())?;
                }
            }
            Some(CompuConstContentRef::CompuConstFormulaContent(id)) => {
                if let Some(formula) = document.compu_const_formula_contents.get(id) {
                    write_optional_text_element(writer, "VF", formula.get_vf())?;
                }
            }
            None => {}
        }
        writer.write_event(Event::End(BytesEnd::new(key)))?;
        Ok(())
    }
}
