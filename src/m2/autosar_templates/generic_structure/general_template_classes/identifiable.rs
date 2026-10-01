//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::Identifiable
//!
//! Spec classes `Referrable`, `MultilanguageReferrable`, `Identifiable`
//! (P0 design §5), modelled as composition per `docs/code_guide.md` §3.

use super::ar_object::ARObject;

/// spec class `Referrable` — `Referrable : ARObject`.
///
/// The `parent: Option<ElementRef>` field is added in Task 5 together with
/// `ElementRef` (it needs `ar_package::ARPackage` to exist first).
#[derive(Debug, Default)]
pub struct Referrable {
    base: ARObject,
    short_name: Option<String>,
}

impl Referrable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &ARObject {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut ARObject {
        &mut self.base
    }

    pub fn get_short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }

    pub fn set_short_name(&mut self, value: impl Into<String>) -> &mut Self {
        self.short_name = Some(value.into());
        self
    }
}

/// spec class `MultilanguageReferrable` — `MultilanguageReferrable : Referrable`.
/// No new fields in P0; `long_name` and friends arrive with the P1 converter.
#[derive(Debug, Default)]
pub struct MultilanguageReferrable {
    base: Referrable,
}

impl MultilanguageReferrable {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn referrable_accessors() {
        let mut referrable = Referrable::new();
        assert_eq!(referrable.get_short_name(), None);

        referrable.set_short_name("Pkg");
        assert_eq!(referrable.get_short_name(), Some("Pkg"));
        assert_eq!(referrable.base().get_checksum(), None);
    }

    #[test]
    fn multilanguage_referrable_forwards_short_name() {
        let mut referrable = MultilanguageReferrable::new();
        referrable.set_short_name("X");
        assert_eq!(referrable.get_short_name(), Some("X"));
        assert_eq!(referrable.base().get_short_name(), Some("X"));
    }
}
