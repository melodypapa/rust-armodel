//! spec: M2::MSR::AsamHdo::SpecialData
//!
//! Spec classes `Sd`, `Sdf`, `Sdg`, `SdgCaption`, `SdgContents` (P0 design §5).
//! `Sd`/`Sdf` carry explicit `xml_space`/`value` fields — this is how
//! whitespace fidelity is preserved in the round trip.

use slotmap::new_key_type;

use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ARObject;
use crate::m2::autosar_templates::generic_structure::general_template_classes::identifiable::Referrable;
use crate::m2::msr::documentation::text_model::language_data_model::XmlSpace;
use crate::m2::msr::documentation::text_model::multilanguage_data::MultiLanguageOverviewParagraphId;

new_key_type! {
    /// Arena keys for the `SpecialData` classes.
    pub struct SdId;
    pub struct SdfId;
    pub struct SdgId;
    pub struct SdgCaptionId;
    pub struct SdgContentsId;
}

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

/// spec class `SdgCaption` — `SdgCaption : MultilanguageReferrable` (P0 §5).
#[derive(Debug, Default)]
pub struct SdgCaption {
    base: Referrable,
    desc: Option<MultiLanguageOverviewParagraphId>,
}

impl SdgCaption {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &Referrable {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut Referrable {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.base.get_short_name()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.base.set_short_name(value);
        self
    }

    pub fn get_desc(&self) -> Option<MultiLanguageOverviewParagraphId> {
        self.desc
    }

    pub fn set_desc(&mut self, desc: MultiLanguageOverviewParagraphId) -> &mut Self {
        self.desc = Some(desc);
        self
    }
}

/// spec class `SdgContents`. The spec marks it as an interface; modelled as a
/// concrete struct per §3.
#[derive(Debug, Default)]
pub struct SdgContents {
    sd: Vec<SdId>,
    sdf: Vec<SdfId>,
    sdg: Vec<SdgId>,
}

impl SdgContents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_sd(&self) -> &[SdId] {
        &self.sd
    }

    /// py `addSd`
    pub fn push_sd(&mut self, sd: SdId) {
        self.sd.push(sd);
    }

    pub fn get_sdf(&self) -> &[SdfId] {
        &self.sdf
    }

    /// py `addSdf`
    pub fn push_sdf(&mut self, sdf: SdfId) {
        self.sdf.push(sdf);
    }

    pub fn get_sdg(&self) -> &[SdgId] {
        &self.sdg
    }

    /// py `addSdg`
    pub fn push_sdg(&mut self, sdg: SdgId) {
        self.sdg.push(sdg);
    }

    /// py `getSdg`'s emptiness check: contents attach to an SDG only when
    /// non-empty.
    pub fn is_empty(&self) -> bool {
        self.sd.is_empty() && self.sdf.is_empty() && self.sdg.is_empty()
    }
}

/// spec class `Sdg`
#[derive(Debug, Default)]
pub struct Sdg {
    base: ARObject,
    gid: Option<String>,
    sdg_caption: Option<SdgCaptionId>,
    sdg_contents_type: Option<SdgContentsId>,
}

impl Sdg {
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

    pub fn get_sdg_caption(&self) -> Option<SdgCaptionId> {
        self.sdg_caption
    }

    pub fn set_sdg_caption(&mut self, caption: SdgCaptionId) -> &mut Self {
        self.sdg_caption = Some(caption);
        self
    }

    pub fn get_sdg_contents_type(&self) -> Option<SdgContentsId> {
        self.sdg_contents_type
    }

    pub fn set_sdg_contents_type(&mut self, contents: SdgContentsId) -> &mut Self {
        self.sdg_contents_type = Some(contents);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

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

    #[test]
    fn sdg_with_contents_and_caption() {
        let mut sdg = Sdg::new();
        sdg.set_gid("demo");
        assert_eq!(sdg.get_gid(), Some("demo"));

        let mut caption = SdgCaption::new();
        caption.set_short_name("Caption");
        assert_eq!(caption.get_short_name(), Some("Caption"));
        assert!(caption.get_desc().is_none());

        let mut contents = SdgContents::new();
        assert!(contents.is_empty());

        let mut arena: SlotMap<SdId, Sd> = SlotMap::with_key();
        let mut sd = Sd::new();
        sd.set_gid("purpose");
        let sd_id = arena.insert(sd);

        contents.push_sd(sd_id);
        assert!(!contents.is_empty());
        assert_eq!(contents.get_sd(), &[sd_id]);
    }
}
