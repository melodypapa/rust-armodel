//! spec: M2::MSR::AsamHdo::SpecialData
//!
//! Spec classes `Sd`, `Sdf`, `Sdg`, `SdgCaption`, `SdgContents` (P0 design §5).
//! `Sd`/`Sdf` carry explicit `xml_space`/`value` fields — this is how
//! whitespace fidelity is preserved in the round trip.
//! `Sdg`, `SdgCaption` and `SdgContents` are added in Task 7.

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::msr::documentation::text_model::language_data_model::XmlSpace;

/// spec class `Sd` — `<SD GID="…" xml:space="preserve">text</SD>`.
#[derive(Debug, Default)]
pub struct Sd {
    base: ARObject,
    gid: Option<String>,
    value: Option<String>,
    xml_space: Option<XmlSpace>,
}

impl Sd {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }

    pub fn get_gid(&self) -> Option<&str> {
        self.gid.as_deref()
    }

    pub fn set_gid(&mut self, value: impl Into<String>) -> &mut Self {
        self.gid = Some(value.into());
        self
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn set_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.value = Some(value.into());
        self
    }

    pub fn get_xml_space(&self) -> Option<XmlSpace> {
        self.xml_space
    }

    pub fn set_xml_space(&mut self, value: XmlSpace) -> &mut Self {
        self.xml_space = Some(value);
        self
    }
}

/// spec class `Sdf`
#[derive(Debug, Default)]
pub struct Sdf {
    base: ARObject,
    gid: Option<String>,
    value: Option<String>,
}

impl Sdf {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }

    pub fn get_gid(&self) -> Option<&str> {
        self.gid.as_deref()
    }

    pub fn set_gid(&mut self, value: impl Into<String>) -> &mut Self {
        self.gid = Some(value.into());
        self
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn set_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.value = Some(value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sd_accessors() {
        let mut sd = Sd::new();
        assert_eq!(sd.get_gid(), None);
        sd.set_gid("purpose")
            .set_value("special   data")
            .set_xml_space(XmlSpace::Preserve);
        assert_eq!(sd.get_gid(), Some("purpose"));
        assert_eq!(sd.get_value(), Some("special   data"));
        assert_eq!(sd.get_xml_space(), Some(XmlSpace::Preserve));
    }

    #[test]
    fn sdf_accessors() {
        let mut sdf = Sdf::new();
        sdf.set_gid("g").set_value("42");
        assert_eq!(sdf.get_gid(), Some("g"));
        assert_eq!(sdf.get_value(), Some("42"));
    }
}
