//! COMPU-METHOD readers (py readCompuMethod → getCompu → getCompuScales →
//! readCompuScale → contents). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::msr::asam_hdo::computation_method::{
    Compu, CompuConst, CompuConstContentRef, CompuConstFormulaContent, CompuConstId,
    CompuConstNumericContent, CompuConstTextContent, CompuContentRef, CompuId, CompuMethodId,
    CompuNominatorDenominator, CompuRationalCoeffs, CompuScale, CompuScaleConstantContents,
    CompuScaleContentsRef, CompuScaleId, CompuScaleRationalFormula, CompuScales,
};

impl ARXMLReader {
    /// py `getCompuConstContent` — the first child element dispatches
    /// VT/V/VF into the concrete content class.
    fn get_compu_const_content(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<CompuConstContentRef>, ParseError> {
        let Some(child) = find(element, "*") else {
            return Ok(None);
        };
        match child.name.as_str() {
            "VT" => {
                let mut content = CompuConstTextContent::new();
                if let Some(vt) = get_child_element_string(element, "VT") {
                    content.set_vt(vt);
                }
                let id = document.compu_const_text_contents.insert(content);
                Ok(Some(CompuConstContentRef::CompuConstTextContent(id)))
            }
            "V" => {
                let mut content = CompuConstNumericContent::new();
                if let Some(v) = get_child_element_string(element, "V") {
                    content.set_v(v);
                }
                let id = document.compu_const_numeric_contents.insert(content);
                Ok(Some(CompuConstContentRef::CompuConstNumericContent(id)))
            }
            "VF" => {
                let mut content = CompuConstFormulaContent::new();
                if let Some(vf) = get_child_element_string(element, "VF") {
                    content.set_vf(vf);
                }
                let id = document.compu_const_formula_contents.insert(content);
                Ok(Some(CompuConstContentRef::CompuConstFormulaContent(id)))
            }
            other => {
                let message = format!("Unsupported CompuConstContent <{other}>");
                self.not_implemented(message)?;
                Ok(None)
            }
        }
    }

    /// py `getCompuConst` — a keyed `<COMPU-…>` wrapper carrying content.
    fn get_compu_const(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<CompuConstId>, ParseError> {
        let Some(child) = find(element, key) else {
            return Ok(None);
        };
        let mut compu_const = CompuConst::new();
        self.read_ar_object(child, compu_const.base_mut());
        if let Some(content) = self.get_compu_const_content(child, document)? {
            compu_const.set_compu_const_content_type(content);
        }
        Ok(Some(document.compu_consts.insert(compu_const)))
    }

    /// py `readCompuScale`.
    fn read_compu_scale(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<CompuScaleId, ParseError> {
        let mut scale = CompuScale::new();
        self.read_ar_object(element, scale.base_mut());
        if let Some(text) = get_child_element_string(element, "A2L-DISPLAY-TEXT") {
            scale.set_a2l_display_text(text);
        }
        if let Some(inverse) = self.get_compu_const(element, "COMPU-INVERSE-VALUE", document)? {
            scale.set_compu_inverse_value(inverse);
        }
        if let Some(text) = get_child_element_string(element, "SHORT-LABEL") {
            scale.set_short_label(text);
        }
        if let Some(text) = get_child_element_string(element, "SYMBOL") {
            scale.set_symbol(text);
        }
        if let Some(desc) = self.get_multi_language_overview_paragraph(element, document)? {
            scale.set_desc(desc);
        }
        if let Some(mask) = get_child_element_string(element, "MASK") {
            scale.set_mask(mask);
        }
        if let Some(limit) = self.get_child_limit_element(element, "LOWER-LIMIT", document)? {
            scale.set_lower_limit(limit);
        }
        if let Some(limit) = self.get_child_limit_element(element, "UPPER-LIMIT", document)? {
            scale.set_upper_limit(limit);
        }
        // py readCompuScaleContents — constant (COMPU-CONST/VT) or rational
        // (COMPU-RATIONAL-COEFFS) contents.
        if find(element, "COMPU-CONST/VT").is_some() {
            let vt = get_child_element_string(element, "COMPU-CONST/VT").unwrap_or("");
            let mut text_content = CompuConstTextContent::new();
            text_content.set_vt(vt);
            let vt_id = document.compu_const_text_contents.insert(text_content);
            let mut const_ = CompuConst::new();
            const_.set_compu_const_content_type(CompuConstContentRef::CompuConstTextContent(vt_id));
            let const_id = document.compu_consts.insert(const_);
            let mut constant = CompuScaleConstantContents::new();
            constant.set_compu_const(const_id);
            let content_id = document.compu_scale_constant_contents.insert(constant);
            scale.set_compu_scale_contents(CompuScaleContentsRef::CompuScaleConstantContents(
                content_id,
            ));
        } else if find(element, "COMPU-RATIONAL-COEFFS").is_some() {
            // py readCompuNominatorDenominator — V children in document order.
            let coeffs_node = find(element, "COMPU-RATIONAL-COEFFS").expect("checked above");
            let numerator = {
                let mut nom_denom = CompuNominatorDenominator::new();
                for v_node in find_all(coeffs_node, "COMPU-NUMERATOR/V") {
                    if let Some(text) = &v_node.text {
                        nom_denom.push_v(text.as_str().to_string());
                    }
                }
                document.compu_nominator_denominators.insert(nom_denom)
            };
            let denominator = {
                let mut nom_denom = CompuNominatorDenominator::new();
                for v_node in find_all(coeffs_node, "COMPU-DENOMINATOR/V") {
                    if let Some(text) = &v_node.text {
                        nom_denom.push_v(text.as_str().to_string());
                    }
                }
                document.compu_nominator_denominators.insert(nom_denom)
            };
            let mut coeffs = CompuRationalCoeffs::new();
            coeffs.set_compu_numerator(numerator);
            coeffs.set_compu_denominator(denominator);
            let coeffs_id = document.compu_rational_coeffs.insert(coeffs);
            let mut formula = CompuScaleRationalFormula::new();
            formula.set_compu_rational_coeffs(coeffs_id);
            let content_id = document.compu_scale_rational_formulas.insert(formula);
            scale.set_compu_scale_contents(CompuScaleContentsRef::CompuScaleRationalFormula(
                content_id,
            ));
        }
        Ok(document.compu_scales.insert(scale))
    }

    /// py `getCompuScales` — the COMPU-SCALES wrapper and its scales.
    fn get_compu_scales(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<Option<CompuContentRef>, ParseError> {
        let Some(scales_node) = find(element, "COMPU-SCALES") else {
            return Ok(None);
        };
        let mut scales = CompuScales::new();
        if let Some(checksum) = scales_node.attrs.get("S") {
            scales.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = scales_node.attrs.get("T") {
            scales.set_timestamp(timestamp.as_str());
        }
        for scale_node in find_all(scales_node, "COMPU-SCALE") {
            let scale_id = self.read_compu_scale(scale_node, document)?;
            scales.push_compu_scale(scale_id);
        }
        let id = document.compu_scales_arena.insert(scales);
        Ok(Some(CompuContentRef::CompuScales(id)))
    }

    /// py `getCompu` — a keyed `<COMPU-…>` wrapper: scales + default value.
    fn get_compu(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<CompuId>, ParseError> {
        let Some(child) = find(element, key) else {
            return Ok(None);
        };
        let mut compu = Compu::new();
        self.read_ar_object(child, compu.base_mut());
        if let Some(content) = self.get_compu_scales(child, document)? {
            compu.set_compu_content(content);
        }
        if let Some(default) = self.get_compu_const(child, "COMPU-DEFAULT-VALUE", document)? {
            compu.set_compu_default_value(default);
        }
        Ok(Some(document.compus.insert(compu)))
    }

    /// py `readCompuMethod` — Identifiable chain + the compu payload.
    pub(super) fn read_compu_method(
        &mut self,
        element: &Node,
        id: CompuMethodId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let display_format =
            get_child_element_string(element, "DISPLAY-FORMAT").map(str::to_string);
        let unit_ref = get_child_element_optional_ref_type(element, "UNIT-REF");
        let internal = self.get_compu(element, "COMPU-INTERNAL-TO-PHYS", document)?;
        let phys = self.get_compu(element, "COMPU-PHYS-TO-INTERNAL", document)?;
        if let Some(compu_method) = document.compu_methods.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                compu_method.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                compu_method.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                compu_method.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                compu_method.set_admin_data(admin_data);
            }
            if let Some(display_format) = display_format {
                compu_method.set_display_format(display_format);
            }
            if let Some(unit_ref) = unit_ref {
                let unit_ref_id = document.ref_types.insert(unit_ref);
                compu_method.set_unit_ref(unit_ref_id);
            }
            if let Some(internal) = internal {
                compu_method.set_compu_internal_to_phys(internal);
            }
            if let Some(phys) = phys {
                compu_method.set_compu_phys_to_internal(phys);
            }
        }
        Ok(())
    }
}
