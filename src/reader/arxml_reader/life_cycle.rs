//! LifeCycleInfoSet readers (py readLifeCycleInfoSet → readLifeCycleInfo →
//! getLifeCyclePeriod). Part of the arxml_reader domain split
//! (docs/superpowers/specs/2026-10-04-parser-writer-domain-split-design.md).
use super::*;

use crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::RefTypeId;
use crate::m2::autosar_templates::generic_structure::life_cycles::{
    LifeCycleInfo, LifeCycleInfoId, LifeCycleInfoSetId, LifeCyclePeriod, LifeCyclePeriodId,
};

/// The `LifeCycleInfoSet`-owned XML payload read by
/// `read_life_cycle_info_set_payload` (py `readLifeCycleInfoSet` minus the
/// common Identifiable parts; refs are already arena ids).
struct LifeCycleInfoSetPayload {
    default_lc_state_ref: Option<RefTypeId>,
    default_period_begin: Option<LifeCyclePeriodId>,
    default_period_end: Option<LifeCyclePeriodId>,
    life_cycle_infos: Vec<LifeCycleInfoId>,
    used_life_cycle_state_definition_group_ref: Option<RefTypeId>,
}

impl ARXMLReader {
    /// py `readLifeCycleInfoSet`.
    pub(super) fn read_life_cycle_info_set(
        &mut self,
        element: &Node,
        id: LifeCycleInfoSetId,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        let payload = self.read_identifiable_payload(element, document)?;
        let set_payload = self.read_life_cycle_info_set_payload(element, document)?;
        if let Some(info_set) = document.life_cycle_info_sets.get_mut(id) {
            if let Some(long_name) = payload.long_name {
                info_set.set_long_name(long_name);
            }
            if let Some(desc) = payload.desc {
                info_set.set_desc(desc);
            }
            if let Some(introduction) = payload.introduction {
                info_set.set_introduction(introduction);
            }
            if let Some(admin_data) = payload.admin_data {
                info_set.set_admin_data(admin_data);
            }
            if let Some(ref_id) = set_payload.default_lc_state_ref {
                info_set.set_default_lc_state_ref(ref_id);
            }
            if let Some(period_id) = set_payload.default_period_begin {
                info_set.set_default_period_begin(period_id);
            }
            if let Some(period_id) = set_payload.default_period_end {
                info_set.set_default_period_end(period_id);
            }
            for info_id in set_payload.life_cycle_infos {
                info_set.push_life_cycle_info(info_id);
            }
            if let Some(ref_id) = set_payload.used_life_cycle_state_definition_group_ref {
                info_set.set_used_life_cycle_state_definition_group_ref(ref_id);
            }
        }
        Ok(())
    }

    /// py `getLifeCyclePeriod` — the wrapper node (e.g. `PERIOD-BEGIN`)
    /// becomes a `LifeCyclePeriod` arena entry. py does not call
    /// `readARObject` here, so S/T attributes are intentionally not read.
    fn get_life_cycle_period(
        &mut self,
        element: &Node,
        key: &str,
        document: &mut Document,
    ) -> Result<Option<LifeCyclePeriodId>, ParseError> {
        let Some(node) = find(element, key) else {
            return Ok(None);
        };
        let mut period = LifeCyclePeriod::new();
        if let Some(date) = get_child_element_string(node, "DATE") {
            period.set_date(date);
        }
        if let Some(version) = get_child_element_string(node, "AR-RELEASE-VERSION") {
            period.set_ar_release_version(version);
        }
        if let Some(release) = get_child_element_string(node, "PRODUCT-RELEASE") {
            period.set_product_release(release);
        }
        Ok(Some(document.life_cycle_periods.insert(period)))
    }

    /// py `readLifeCycleInfo` — returns the inserted arena id.
    fn read_life_cycle_info(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<LifeCycleInfoId, ParseError> {
        let mut info = LifeCycleInfo::new();
        self.read_ar_object(element, info.base_mut());
        if let Some(r#ref) = get_child_element_optional_ref_type(element, "LC-OBJECT-REF") {
            info.set_lc_object_ref(document.ref_types.insert(r#ref));
        }
        if let Some(r#ref) = get_child_element_optional_ref_type(element, "LC-STATE-REF") {
            info.set_lc_state_ref(document.ref_types.insert(r#ref));
        }
        if let Some(period_id) = self.get_life_cycle_period(element, "PERIOD-BEGIN", document)? {
            info.set_period_begin(period_id);
        }
        if let Some(period_id) = self.get_life_cycle_period(element, "PERIOD-END", document)? {
            info.set_period_end(period_id);
        }
        if let Some(remark_node) = find(element, "REMARK") {
            let remark = self.read_documentation_block(remark_node, document)?;
            info.set_remark(remark);
        }
        // py readLifeCycleInfoUseInsteadRefs
        for r#ref in get_child_element_ref_type_list(element, "USE-INSTEAD-REFS/USE-INSTEAD-REF") {
            info.push_use_instead_ref(document.ref_types.insert(r#ref));
        }
        Ok(document.life_cycle_infos.insert(info))
    }

    /// py `readLifeCycleInfoSet` — the family-specific part (the common
    /// Identifiable payload is read separately).
    fn read_life_cycle_info_set_payload(
        &mut self,
        element: &Node,
        document: &mut Document,
    ) -> Result<LifeCycleInfoSetPayload, ParseError> {
        let default_lc_state_ref =
            match get_child_element_optional_ref_type(element, "DEFAULT-LC-STATE-REF") {
                Some(r#ref) => Some(document.ref_types.insert(r#ref)),
                None => None,
            };
        let default_period_begin =
            self.get_life_cycle_period(element, "DEFAULT-PERIOD-BEGIN", document)?;
        let default_period_end =
            self.get_life_cycle_period(element, "DEFAULT-PERIOD-END", document)?;
        let mut life_cycle_infos = Vec::new();
        for child in find_all(element, "LIFE-CYCLE-INFOS/*") {
            if child.name == "LIFE-CYCLE-INFO" {
                life_cycle_infos.push(self.read_life_cycle_info(child, document)?);
            } else {
                let message = format!("Unsupported Life Cycle Info <{}>", child.name);
                self.not_implemented(message)?;
            }
        }
        let used_life_cycle_state_definition_group_ref = match get_child_element_optional_ref_type(
            element,
            "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF",
        ) {
            Some(r#ref) => Some(document.ref_types.insert(r#ref)),
            None => None,
        };
        Ok(LifeCycleInfoSetPayload {
            default_lc_state_ref,
            default_period_begin,
            default_period_end,
            life_cycle_infos,
            used_life_cycle_state_definition_group_ref,
        })
    }
}
