//! DATA-CONSTR readers (py readDataConstr → readDataConstrRule → constrs;
//! get_child_limit_element is the shared LIMIT helper). Part of the
//! arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::{
    Limit, LimitId, MonotonyEnum,
};
use crate::m2::msr::asam_hdo::constraints::global_constraints::{
    DataConstrId, InternalConstrs, PhysConstrs, ScaleConstr, ScaleConstrValidityEnum,
};

impl ARXMLReader {
    /// py `readScaleConstr`.
    fn read_scale_constr(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<crate::m2::msr::asam_hdo::constraints::global_constraints::ScaleConstrId, ParseError>
    {
        let mut scale_constr = ScaleConstr::new();
        self.read_ar_object(element, scale_constr.base_mut());
        if let Some(desc) = self.get_multi_language_overview_paragraph(element, document)? {
            scale_constr.set_desc(desc);
        }
        if let Some(limit) = self.get_child_limit_element(element, "LOWER-LIMIT", document)? {
            scale_constr.set_lower_limit(limit);
        }
        if let Some(label) = get_child_element_string(element, "SHORT-LABEL") {
            scale_constr.set_short_label(label);
        }
        if let Some(limit) = self.get_child_limit_element(element, "UPPER-LIMIT", document)? {
            scale_constr.set_upper_limit(limit);
        }
        if let Some(validity) = element.attrs.get("VALIDITY") {
            match ScaleConstrValidityEnum::try_from(validity.as_str()) {
                Ok(value) => {
                    scale_constr.set_validity(value);
                }
                Err(_) => {
                    let message = format!("Unsupported VALIDITY <{validity}>");
                    self.not_implemented(message)?;
                }
            }
        }
        Ok(document.scale_constrs.insert(scale_constr))
    }

    /// py `readDataConstr` — Identifiable chain + DATA-CONSTR-RULES.
    pub(super) fn read_data_constr(
        &mut self,
        element: &Node,
        id: DataConstrId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        // py readDataConstrRule
        let mut rules = Vec::new();
        for rule_node in find_all(element, "DATA-CONSTR-RULES/DATA-CONSTR-RULE") {
            let mut rule =
                crate::m2::msr::asam_hdo::constraints::global_constraints::DataConstrRule::new();
            self.read_ar_object(rule_node, rule.base_mut());
            if let Some(level) = get_child_element_string(rule_node, "CONSTR-LEVEL") {
                rule.set_constr_level(level);
            }
            // py readInternalConstrs
            if let Some(constrs_node) = find(rule_node, "INTERNAL-CONSTRS") {
                let mut constrs = InternalConstrs::new();
                self.read_ar_object(constrs_node, constrs.base_mut());
                if let Some(limit) =
                    self.get_child_limit_element(constrs_node, "LOWER-LIMIT", document)?
                {
                    constrs.set_lower_limit(limit);
                }
                if let Some(limit) =
                    self.get_child_limit_element(constrs_node, "UPPER-LIMIT", document)?
                {
                    constrs.set_upper_limit(limit);
                }
                for sc in find_all(constrs_node, "SCALE-CONSTRS/SCALE-CONSTR") {
                    let sc_id = self.read_scale_constr(sc, document)?;
                    constrs.push_scale_constr(sc_id);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MAX-GRADIENT") {
                    constrs.set_max_gradient(text);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MAX-DIFF") {
                    constrs.set_max_diff(text);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MONOTONY") {
                    match MonotonyEnum::try_from(text) {
                        Ok(value) => {
                            constrs.set_monotony(value);
                        }
                        Err(_) => {
                            let message = format!("Unsupported MONOTONY <{text}>");
                            self.not_implemented(message)?;
                        }
                    }
                }
                let constrs_id = document.internal_constrs.insert(constrs);
                rule.set_internal_constrs(constrs_id);
            }
            // py readPhysConstrs
            if let Some(constrs_node) = find(rule_node, "PHYS-CONSTRS") {
                let mut constrs = PhysConstrs::new();
                self.read_ar_object(constrs_node, constrs.base_mut());
                if let Some(limit) =
                    self.get_child_limit_element(constrs_node, "LOWER-LIMIT", document)?
                {
                    constrs.set_lower_limit(limit);
                }
                if let Some(limit) =
                    self.get_child_limit_element(constrs_node, "UPPER-LIMIT", document)?
                {
                    constrs.set_upper_limit(limit);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MAX-DIFF") {
                    constrs.set_max_diff(text);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MAX-GRADIENT") {
                    constrs.set_max_gradient(text);
                }
                if let Some(text) = get_child_element_string(constrs_node, "MONOTONY") {
                    match MonotonyEnum::try_from(text) {
                        Ok(value) => {
                            constrs.set_monotony(value);
                        }
                        Err(_) => {
                            let message = format!("Unsupported MONOTONY <{text}>");
                            self.not_implemented(message)?;
                        }
                    }
                }
                for sc in find_all(constrs_node, "SCALE-CONSTRS/SCALE-CONSTR") {
                    let sc_id = self.read_scale_constr(sc, document)?;
                    constrs.push_scale_constr(sc_id);
                }
                if let Some(unit_ref) =
                    get_child_element_optional_ref_type(constrs_node, "UNIT-REF")
                {
                    let unit_ref_id = document.ref_types.insert(unit_ref);
                    constrs.set_unit_ref(unit_ref_id);
                }
                let constrs_id = document.phys_constrs.insert(constrs);
                rule.set_phys_constrs(constrs_id);
            }
            rules.push(rule);
        }
        if let Some(data_constr) = document.data_constrs.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                data_constr.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                data_constr.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                data_constr.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                data_constr.set_admin_data(admin_data);
            }
            for rule in rules {
                let rule_id = document.data_constr_rules.insert(rule);
                data_constr.push_data_constr_rule(rule_id);
            }
        }
        Ok(())
    }

    /// py `getChildLimitElement` — a `<LOWER-LIMIT INTERVAL-TYPE="…">value
    /// </LOWER-LIMIT>` child as an arena-stored `Limit`.
    /// `allow(dead_code)` until Task 3 wires the Compu arms.
    #[allow(dead_code)]
    pub(super) fn get_child_limit_element(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<LimitId>, ParseError> {
        let Some(limit_node) = find(element, key) else {
            return Ok(None);
        };
        let mut limit = Limit::new();
        if let Some(checksum) = limit_node.attrs.get("S") {
            limit.set_checksum(checksum.as_str());
        }
        if let Some(timestamp) = limit_node.attrs.get("T") {
            limit.set_timestamp(timestamp.as_str());
        }
        // py stores the attribute text verbatim (AREnum.setValue); fixtures
        // carry uppercase CLOSED against the enum's lowercase literal.
        if let Some(interval) = limit_node.attrs.get("INTERVAL-TYPE") {
            limit.set_interval_type(interval.as_str());
        }
        if let Some(text) = &limit_node.text {
            limit.set_value(text.as_str());
        }
        Ok(Some(document.limits.insert(limit)))
    }
}
