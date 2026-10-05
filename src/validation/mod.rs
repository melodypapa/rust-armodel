//! XSD validation for ARXML parse and write
//! (docs/superpowers/specs/2026-10-05-xsd-validation-design.md — Rust port).
/// `allow(dead_code)` until the release-detection/validator tasks consume it.
#[allow(dead_code)]
pub(crate) static SCHEMAS: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/schemas");

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
}
