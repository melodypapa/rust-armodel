//! XSD validation for ARXML parse and write
//! (docs/superpowers/specs/2026-10-05-xsd-validation-design.md — Rust port).
#[cfg(test)]
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

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

/// A single XSD violation (py `ValidationError` minus the unused `domain`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

// Compiled-schema cache. libxml validation contexts are `!Send`/`!Sync`, so
// a process-wide `static` is off the table even under a `Mutex` (a mutex needs
// `Send` for `Sync`); a thread-local keeps the compiled schema warm per thread
// without unsafe.
thread_local! {
    static SCHEMA_CACHE: RefCell<HashMap<&'static str, libxml::schemas::SchemaValidationContext>> =
        RefCell::new(HashMap::new());
    #[cfg(test)]
    static SCHEMA_COUNT: Cell<usize> = const { Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn compiled_schema_count() -> usize {
    SCHEMA_COUNT.with(Cell::get)
}

/// Extract the embedded `schemas/` tree once so libxml2 can resolve includes
/// from real paths (py ships them as package data; a published Rust crate
/// cannot address its packaged files at runtime).
fn materialized_dir() -> &'static PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir =
            std::env::temp_dir().join(format!("armodel-schemas-{}", env!("CARGO_PKG_VERSION")));
        if !dir.exists() {
            SCHEMAS.extract(&dir).expect("materialize embedded schemas");
        }
        dir
    })
}

/// py `ARXMLValidator` (AMENDED as-built API, Rust spelling).
pub struct ARXMLValidator {
    release: &'static str,
}

impl ARXMLValidator {
    /// py `for_document`: `None` when the document's schema file is not
    /// bundled (caller decides: reader warns and continues unvalidated).
    pub fn for_document(data: &[u8]) -> Option<Self> {
        let release = detect_release(data)?;
        Some(Self { release })
    }

    pub fn release(&self) -> &'static str {
        self.release
    }

    /// Validate `data` against the release schema; returns ALL violations.
    /// Ill-formed XML yields no violations: well-formedness is the reader's
    /// quick-xml gate, not this XSD gate's.
    pub fn validate(&self, data: &[u8]) -> Vec<ValidationError> {
        let dir = materialized_dir();
        let xsd = dir.join(self.release).join(schema_file_name(self.release));
        let libxml_doc = match libxml::parser::Parser::default().parse_string(data) {
            Ok(doc) => doc,
            Err(_) => return Vec::new(),
        };

        SCHEMA_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let ctx = cache.entry(self.release).or_insert_with(|| {
                #[cfg(test)]
                SCHEMA_COUNT.with(|count| count.set(count.get() + 1));
                let mut parser_ctx = libxml::schemas::SchemaParserContext::from_file(
                    xsd.to_str().expect("materialized temp path is utf-8"),
                ); // CONFIRM-1: `from_file(&str) -> Self` exists as planned
                libxml::schemas::SchemaValidationContext::from_parser(&mut parser_ctx)
                    .expect("bundled schema compiles") // CONFIRM-2: takes `&mut SchemaParserContext`
            });
            match ctx.validate_document(&libxml_doc) {
                // CONFIRM-3: no `validate(&Document) -> bool`; the Result
                // carries the violations, so `drain_errors` is redundant.
                Ok(()) => Vec::new(),
                Err(errors) => errors
                    .into_iter()
                    .map(|e| ValidationError {
                        line: usize::try_from(e.line.unwrap_or(0)).unwrap_or(0),
                        column: usize::try_from(e.col.unwrap_or(0)).unwrap_or(0),
                        message: e
                            .message
                            .map(|m| m.trim_end_matches('\n').to_owned())
                            .unwrap_or_default(),
                    })
                    .collect(),
            }
        })
    }
}

/// The registry's canonical top-level schema file per release.
fn schema_file_name(release: &str) -> &'static str {
    match release {
        "R23-11" => "AUTOSAR_00052.xsd",
        "R4.4.0" => "AUTOSAR_00046.xsd",
        "R4.3.1" => "AUTOSAR_00044.xsd",
        "R3.2.3" => "AUTOSAR.xsd",
        other => unreachable!("registry release {other} has no schema file"),
    }
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

    #[test]
    fn validates_a_wellformed_document_clean() {
        let data = br#"<?xml version="1.0"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>P</SHORT-NAME></AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;
        let validator = ARXMLValidator::for_document(data).expect("R4.4.0 is bundled");
        assert_eq!(validator.validate(data), vec![]);
    }

    #[test]
    fn reports_element_order_violations_with_position() {
        // ADMIN-DATA sequence violated: SDGS before USED-LANGUAGES.
        let data = br#"<?xml version="1.0"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd">
  <ADMIN-DATA>
    <SDGS><SDG/></SDGS>
    <USED-LANGUAGES><L-10 L="EN">English</L-10></USED-LANGUAGES>
  </ADMIN-DATA>
</AUTOSAR>"#;
        let validator = ARXMLValidator::for_document(data).expect("R4.4.0 is bundled");
        let errors = validator.validate(data);
        assert!(!errors.is_empty(), "expected sequence violations");
        assert!(errors[0].line >= 1);
        let lower = errors[0].message.to_lowercase();
        assert!(
            lower.contains("used-languages") || lower.contains("sdgs"),
            "{:?}",
            errors[0]
        );
    }

    #[test]
    fn schema_compilation_is_cached() {
        // Two validators over the same release must reuse the compiled schema:
        // compilation is lazy (first `validate`), so assert the SECOND
        // validate compiles nothing (the count is thread-local, so only the
        // delta is meaningful).
        let data = br#"<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd"/>"#;
        drop(ARXMLValidator::for_document(data).unwrap().validate(data));
        let after_first = compiled_schema_count();
        drop(ARXMLValidator::for_document(data).unwrap().validate(data));
        assert_eq!(compiled_schema_count(), after_first);
    }
}
