//! spec: M2::MSR::Documentation::TextModel::LanguageDataModel
//!
//! Spec classes `LPlainText`, `WhitespaceControlled`, `LanguageSpecific`
//! (P0 design §5). The spec marks `LPlainText` as an interface; per the locked
//! decision (P0 design §3) it is a concrete struct.

use std::fmt;

/// `xml:space` enumeration. Whitespace-fidelity carrier: text elements
/// (`SD`, `L-10`) keep this as an explicit field (`docs/code_guide.md` §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSpace {
    Preserve,
    Default,
}

impl TryFrom<&str> for XmlSpace {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "preserve" => Ok(XmlSpace::Preserve),
            "default" => Ok(XmlSpace::Default),
            _ => Err(()),
        }
    }
}

impl fmt::Display for XmlSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmlSpace::Preserve => write!(f, "preserve"),
            XmlSpace::Default => write!(f, "default"),
        }
    }
}

/// spec class `LPlainText` — the `<L-10 L="EN" xml:space="preserve">…</L-10>`
/// element. `l` comes from `LanguageSpecific`, `xml_space` from
/// `WhitespaceControlled`, `value` from the mixed-content base.
#[derive(Debug, Default)]
pub struct LPlainText {
    l: Option<String>,
    xml_space: Option<XmlSpace>,
    value: Option<String>,
}

impl LPlainText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_l(&self) -> Option<&str> {
        self.l.as_deref()
    }

    pub fn set_l(&mut self, value: impl Into<String>) -> &mut Self {
        self.l = Some(value.into());
        self
    }

    pub fn get_xml_space(&self) -> Option<XmlSpace> {
        self.xml_space
    }

    pub fn set_xml_space(&mut self, value: XmlSpace) -> &mut Self {
        self.xml_space = Some(value);
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
    fn l_plain_text_accessors() {
        let mut l10 = LPlainText::new();
        assert_eq!(l10.get_l(), None);
        assert_eq!(l10.get_xml_space(), None);
        assert_eq!(l10.get_value(), None);

        l10.set_l("EN")
            .set_xml_space(XmlSpace::Preserve)
            .set_value("English");

        assert_eq!(l10.get_l(), Some("EN"));
        assert_eq!(l10.get_xml_space(), Some(XmlSpace::Preserve));
        assert_eq!(l10.get_value(), Some("English"));
    }

    #[test]
    fn xml_space_parses_and_formats() {
        assert_eq!(XmlSpace::try_from("preserve"), Ok(XmlSpace::Preserve));
        assert_eq!(XmlSpace::try_from("default"), Ok(XmlSpace::Default));
        assert!(XmlSpace::try_from("bogus").is_err());
        assert_eq!(XmlSpace::Preserve.to_string(), "preserve");
        assert_eq!(XmlSpace::Default.to_string(), "default");
    }
}
