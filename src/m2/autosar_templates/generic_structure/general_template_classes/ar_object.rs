//! spec: M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ArObject
//!
//! Spec class `ARObject` — abstract base of all AUTOSAR meta-classes
//! (AUTOSAR_FO_TPS_GenericStructureTemplate, Table 6.1).
//! `ElementRef` (the heterogeneous-link enum) is added in Task 5.

/// spec class `ARObject`
#[derive(Debug, Default)]
pub struct ARObject {
    checksum: Option<String>,
    timestamp: Option<String>,
}

impl ARObject {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }

    pub fn set_checksum(&mut self, value: impl Into<String>) -> &mut Self {
        self.checksum = Some(value.into());
        self
    }

    pub fn get_timestamp(&self) -> Option<&str> {
        self.timestamp.as_deref()
    }

    pub fn set_timestamp(&mut self, value: impl Into<String>) -> &mut Self {
        self.timestamp = Some(value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ar_object_accessors() {
        let mut ar_object = ARObject::new();
        assert_eq!(ar_object.get_checksum(), None);
        assert_eq!(ar_object.get_timestamp(), None);

        ar_object
            .set_checksum("abc")
            .set_timestamp("2026-10-01T12:00:00+01:00");

        assert_eq!(ar_object.get_checksum(), Some("abc"));
        assert_eq!(ar_object.get_timestamp(), Some("2026-10-01T12:00:00+01:00"));
    }
}
