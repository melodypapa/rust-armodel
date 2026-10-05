//! XSD validation for ARXML parse and write
//! (docs/superpowers/specs/2026-10-05-xsd-validation-design.md — Rust port).
/// `allow(dead_code)` until the release-detection/validator tasks consume it.
#[allow(dead_code)]
pub(crate) static SCHEMAS: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/schemas");

/// `schemaLocation` filename → bundled release (case-insensitive on filename).
const REGISTRY: &[(&str, &str)] = &[
    ("AUTOSAR_00052.XSD", "R23-11"),
    ("AUTOSAR_00046.XSD", "R4.4.0"),
    ("AUTOSAR_00044.XSD", "R4.3.1"),
    ("AUTOSAR.XSD", "R3.2.3"),
];

/// py `detect_schema_info` (AMENDED: `xsi:schemaLocation` only — no
/// ADMIN-DATA fallback). Returns the bundled release for the document's
/// schema file, or `None` when unresolvable.
pub fn detect_release(data: &[u8]) -> Option<&'static str> {
    let file = schema_location_file(data)?;
    let upper = file.to_ascii_uppercase();
    REGISTRY
        .iter()
        .find(|(name, _)| name.as_bytes() == upper)
        .map(|(_, release)| *release)
}

/// The last whitespace-separated token of `xsi:schemaLocation="ns FILE"`.
fn schema_location_file(data: &[u8]) -> Option<&[u8]> {
    let pos = data.windows(16).position(|w| w == b"schemaLocation=\"")?;
    let value = &data[pos + 16..];
    let end = value.iter().position(|b| *b == b'"')?;
    let value = &value[..end];
    let start = value.iter().rposition(|b| b.is_ascii_whitespace())? + 1;
    Some(&value[start..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_four_releases_are_embedded() {
        for dir in ["R23-11", "R4.4.0", "R4.3.1", "R3.2.3"] {
            assert!(
                SCHEMAS.get_dir(dir).is_some(),
                "{dir} missing from schemas/"
            );
        }
        assert!(SCHEMAS.get_file("R4.4.0/xml.xsd").is_some());
    }

    /// Byte-sync against the pinned py-armodel checkout (skipped when the
    /// gitignored checkout is absent — CI).
    #[test]
    fn schemas_match_pinned_py_armodel() {
        let pinned = std::path::Path::new("target/py-armodel/autosar");
        if !pinned.is_dir() {
            return;
        }
        for (dir, file) in [
            ("R23-11", "AUTOSAR_00052.xsd"),
            ("R4.4.0", "AUTOSAR_00046.xsd"),
            ("R4.4.0", "xml.xsd"),
            ("R4.3.1", "AUTOSAR_00044.xsd"),
            ("R3.2.3", "AUTOSAR.xsd"),
        ] {
            let pinned_bytes = std::fs::read(pinned.join(dir).join("xsd").join(file)).unwrap();
            let embedded = SCHEMAS
                .get_file(format!("{dir}/{file}"))
                .unwrap()
                .contents();
            assert_eq!(embedded, pinned_bytes, "{dir}/{file} drifted from the pin");
        }
    }

    #[test]
    fn detects_release_from_schema_location() {
        let xml = br#"<?xml version="1.0"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd"/>"#;
        assert_eq!(super::detect_release(xml), Some("R4.4.0"));
    }

    #[test]
    fn detection_is_case_insensitive_on_filename() {
        let xml = br#"<AUTOSAR xmlns="http://autosar.org" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org autosar.xsd"/>"#;
        assert_eq!(super::detect_release(xml), Some("R3.2.3"));
    }

    #[test]
    fn unknown_schema_file_is_none() {
        // 24 of our 32 fixtures reference R21-11's AUTOSAR_00050.xsd, which is
        // not bundled — unresolvable means "unvalidated", not an error.
        let xml = br#"<AUTOSAR xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd"/>"#;
        assert_eq!(super::detect_release(xml), None);
    }

    #[test]
    fn no_schema_location_is_none() {
        assert_eq!(super::detect_release(b"<AUTOSAR/>"), None);
    }
}
