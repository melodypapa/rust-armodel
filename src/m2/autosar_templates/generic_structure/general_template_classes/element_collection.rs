//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ElementCollection
//!
//! Spec classes `CollectableElement`, `Collection` (P0 design §5).

use super::identifiable::Identifiable;

/// spec class `CollectableElement` — `CollectableElement : Identifiable`.
/// No new fields in P0.
#[derive(Debug, Default)]
pub struct CollectableElement {
    base: Identifiable,
}

impl CollectableElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base(&self) -> &Identifiable {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut Identifiable {
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
    fn collectable_element_forwards_short_name() {
        let mut element = CollectableElement::new();
        element.set_short_name("Coll");
        assert_eq!(element.get_short_name(), Some("Coll"));
    }
}
