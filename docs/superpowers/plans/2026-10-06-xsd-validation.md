# XSD Validation Implementation Plan (Rust conversion of the py-armodel design)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Port the approved XSD-validation design (`docs/superpowers/specs/2026-10-05-xsd-validation-design.md`, including its AMENDED as-built notes) to rust-armodel: pre-parse validation in `ARXMLReader::load` and pre-save validation in `ARXMLWriter::save`, against bundled AUTOSAR schemas, with the reader's two-mode warning convention.

**Architecture:** New `src/validation/` module (`ARXMLValidator` + `ValidationError`) backed by the **`libxml` crate** (0.3.21) — the same libxml2 engine py's lxml wraps, so schema-surface fidelity carries over. Bundled schemas are byte-copies of `target/py-armodel/autosar/<release>/xsd/` committed under `schemas/`, embedded at compile time with `include_dir`, materialized to a temp dir once (libxml2 resolves `xs:include`/`xs:import` from file paths), compiled lazily, and cached behind a mutex (libxml validation contexts are `!Send`/`!Sync`). The reader gate reads the file to bytes, validates, then parses from the in-memory buffer; the writer gate validates its existing output buffer before the file write.

**Tech Stack:** Rust stable / edition 2021; new deps: `libxml = "0.3"` (needs system `libxml2` via pkg-config), `include_dir = "0.7"`. Everything else existing (quick-xml, thiserror, slotmap, tempfile).

**Spec → Rust conversion decisions** ( deviations are deliberate; the py spec's AMENDED notes are the baseline):

| # | py spec | rust-armodel adaptation | Why |
|---|---|---|---|
| 1 | lxml `XMLSchema` | `libxml` crate | Same libxml2 engine; the pure-Rust `xmlschema` crate is too young for AUTOSAR-scale schemas (10 MB, thousands of types). System `libxml2` becomes a build prerequisite (documented in README). |
| 2 | schemas as package data on disk | committed `schemas/` + `include_dir` + extract-once to `$TMPDIR/armodel-schemas-<version>/` | libxml2 needs real paths for include resolution; a published crate's files can't be addressed at runtime. |
| 3 | `load()` validates bytes then `ET.parse` | `load()` reads bytes, validates, then `Reader::from_reader(&data[..])` | quick-xml streams; the pre-parse gate needs the bytes first. `load_from_reader` stays public and **ungated** (advanced/tests entry, documented). |
| 4 | writer `warning=True` logs and writes anyway | writer has no warning sink → validation failure is always `Err` unless `validate = false` | Silently writing known-invalid files is worse; the escape hatch preserves legacy behavior. |
| 5 | bundled = R23-11, R4.4.0, R4.3.1, R3.2.3 | same four sets, byte-copies of the pinned checkout's `target/py-armodel/autosar/<release>/xsd/` | Only sets the repo vendors. Consequence (measured): 30 of 32 fixtures reference `AUTOSAR_00050.xsd`/`AUTOSAR_4-0-3.xsd`/`AUTOSAR_00043.xsd` — none bundled → unresolvable → warn + unvalidated, exactly the py spec's amended rule. Resolvable today: `Os_ECUC_4.4.0.arxml` (`AUTOSAR_00046.xsd` → R4.4.0) and the `autosar.xsd` fixture (→ R3.2.3, case-insensitive against `AUTOSAR.xsd`). |
| 6 | `ValidationError(line, column, message, domain)` | drop `domain` (unused), add `line/column/message` | YAGNI. |
| 7 | single `ValueError` after per-violation logs | new `ParseError::SchemaValidation { count, line, column, message }` | Thiserror convention (code_guide §7) instead of a bare `ValueError`. |

**Branch:** `feature/xsd-validation` (already checked out, tracks origin/main @ `c358142` — post PR #18/#20, so the reader is `ARXMLReader` under `src/reader/arxml_reader/`).

**Gate after every task:** `cargo build && cargo fmt --all && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test` — all green (8 suites, lib count per task) before the commit. Commit tag `feat(validation): …` / `chore(validation): …`; no AI attribution; never stage `.pdf_table_cache.json`.

---

### Task 0: vendor the schemas + embed + hash-sync guard

**Files:**
- Create: `schemas/R23-11/AUTOSAR_00052.xsd`, `schemas/R4.4.0/{AUTOSAR_00046.xsd,xml.xsd}`, `schemas/R4.3.1/AUTOSAR_00044.xsd`, `schemas/R3.2.3/AUTOSAR.xsd` (byte-copies)
- Create: `schemas/README.md`
- Modify: `Cargo.toml` (add `include_dir`)
- Test: `src/validation/mod.rs` (created empty in this task, test module with the guard tests)

- [ ] **Step 1: Copy the schema sets verbatim from the pinned checkout**

```bash
mkdir -p schemas/R23-11 schemas/R4.4.0 schemas/R4.3.1 schemas/R3.2.3
cp target/py-armodel/autosar/R23-11/xsd/AUTOSAR_00052.xsd schemas/R23-11/
cp target/py-armodel/autosar/R4.4.0/xsd/AUTOSAR_00046.xsd schemas/R4.4.0/
cp target/py-armodel/autosar/R4.4.0/xsd/xml.xsd          schemas/R4.4.0/
cp target/py-armodel/autosar/R4.3.1/xsd/AUTOSAR_00044.xsd schemas/R4.3.1/
cp target/py-armodel/autosar/R3.2.3/xsd/AUTOSAR.xsd       schemas/R3.2.3/
```

- [ ] **Step 2: Provenance README**

```markdown
# Bundled AUTOSAR schemas

Byte-copies of the XSD sets vendored by the pinned py-armodel checkout
(`tools/py2rust/PY_ARMODEL_VERSION` pin; source: autosar.org release tarballs,
see py-armodel `autosar/<release>/xsd/`). Registries:

| File (case-insensitive) | Release |
|---|---|
| `AUTOSAR_00052.xsd` | R23-11 |
| `AUTOSAR_00046.xsd` | R4.4.0 |
| `AUTOSAR_00044.xsd` | R4.3.1 |
| `AUTOSAR.xsd`       | R3.2.3 |

`xml.xsd` (W3C) is shared by R4.4.0. Sync-guard: the
`schemas_match_pinned_py_armodel` test compares bytes against
`target/py-armodel/autosar/<release>/xsd/` when that checkout exists locally
(it is gitignored; CI skips the test).
```

- [ ] **Step 3: Add `include_dir = "0.7"` to `[dependencies]` in `Cargo.toml`.** Note: `libxml` lands in Task 2; `include_dir` is needed here.

- [ ] **Step 4: Create `src/validation/mod.rs` with the embed + guard tests**

```rust
//! XSD validation for ARXML parse and write
//! (docs/superpowers/specs/2026-10-05-xsd-validation-design.md — Rust port).
pub(crate) static SCHEMAS: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/schemas");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_four_releases_are_embedded() {
        for dir in ["R23-11", "R4.4.0", "R4.3.1", "R3.2.3"] {
            assert!(SCHEMAS.get_dir(dir).is_some(), "{dir} missing from schemas/");
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
            let embedded = SCHEMAS.get_file(format!("{dir}/{file}")).unwrap().contents();
            assert_eq!(embedded, pinned_bytes, "{dir}/{file} drifted from the pin");
        }
    }
}
```

Register the module in `src/lib.rs`: add `pub mod validation;` after `pub mod reader;`.

- [ ] **Step 5: Gate + commit** — `chore(validation): vendor AUTOSAR schema sets, embed via include_dir`

### Task 1: detection — `schemaLocation` filename → release registry

**Files:** Modify `src/validation/mod.rs`.

- [ ] **Step 1: Write the failing tests** (append to the test module):

```rust
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
```

- [ ] **Step 2: Run to verify they fail** — `cargo test --lib validation` → `detect_release` not defined.

- [ ] **Step 3: Implement** (above the test module):

```rust
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
        .find(|(name, _)| *name == upper)
        .map(|(_, release)| *release)
}

/// The last whitespace-separated token of `xsi:schemaLocation="ns FILE"`.
fn schema_location_file(data: &[u8]) -> Option<&[u8]> {
    let pos = data.windows(15).position(|w| w == b"schemaLocation=\"")?;
    let value = &data[pos + 15..];
    let end = value.iter().position(|b| *b == b'"')?;
    let value = &value[..end];
    let start = value.iter().rposition(|b| b.is_ascii_whitespace())? + 1;
    Some(&value[start..])
}
```

- [ ] **Step 4: Gate + commit** — `feat(validation): schemaLocation release detection`

### Task 2: validator core — libxml compile, cache, `validate`

**Files:** Modify `src/validation/mod.rs`, `Cargo.toml` (add `libxml = "0.3"`).

- [ ] **Step 0: API confirmation (permitted adjustment point).** Read https://docs.rs/libxml/latest/libxml/schemas/ — `SchemaParserContext::from_file/from_buffer`, `SchemaValidationContext::from_parser`, `validate(&Document) -> bool`, `drain_errors() -> Vec<StructuredError>`, and `libxml::error::StructuredError`'s accessors (line/column/message). If any name differs from the code below, adjust **only** the three marked lines (`CONFIRM-1/2/3`) — nothing else.

- [ ] **Step 1: Add `libxml = "0.3"` to `[dependencies]`.** Verify it builds: `cargo build` (requires system libxml2 + pkg-config; if pkg-config cannot find libxml2, STOP and report BLOCKED with the error — this is an environment prerequisite, not a code fix). **CI check:** inspect `.github/workflows/` — the ubuntu job needs `libxml2-dev` (add `sudo apt-get update && sudo apt-get install -y libxml2-dev` before the build steps if absent).

- [ ] **Step 2: Write the failing tests:**

```rust
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
        assert!(errors[0].message.to_lowercase().contains("used-languages") || errors[0].message.to_lowercase().contains("sdgs"));
    }

    #[test]
    fn schema_compilation_is_cached() {
        // Two validators over the same release must reuse the compiled schema
        // (observability: the cache counter; see `compiled_schema_count`).
        let data = br#"<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd"/>"#;
        let before = compiled_schema_count();
        drop(ARXMLValidator::for_document(data).unwrap());
        drop(ARXMLValidator::for_document(data).unwrap());
        assert_eq!(compiled_schema_count(), before + 1);
    }
```

- [ ] **Step 3: Run to verify they fail** (`ARXMLValidator` not defined).

- [ ] **Step 4: Implement the core** (above the test module):

```rust
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

/// A single XSD violation (py `ValidationError` minus the unused `domain`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

/// Compiled-schema cache. libxml validation contexts are `!Send`/`!Sync`, so
/// every use goes through this mutex (documents are parsed outside it).
static SCHEMA_CACHE: OnceLock<Mutex<HashMap<&'static str, libxml::schemas::SchemaValidationContext>>> =
    OnceLock::new();
static SCHEMA_COUNT: OnceLock<Mutex<usize>> = OnceLock::new();

#[cfg(test)]
pub(crate) fn compiled_schema_count() -> usize {
    *SCHEMA_COUNT.get_or_init(|| Mutex::new(0)).lock().unwrap()
}

/// Extract the embedded `schemas/` tree once so libxml2 can resolve includes
/// from real paths (py ships them as package data; a published Rust crate
/// cannot address its packaged files at runtime).
fn materialized_dir() -> &'static PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("armodel-schemas-{}", env!("CARGO_PKG_VERSION")));
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
    pub fn validate(&self, data: &[u8]) -> Vec<ValidationError> {
        let dir = materialized_dir();
        let xsd = dir.join(self.release).join(schema_file_name(self.release));
        let libxml_doc = libxml::parser::Parser::default()
            .parse_string(data)
            .expect("well-formed input: the reader's DOM pass already parsed it");

        let mut cache = SCHEMA_CACHE.get_or_init(Default::default).lock().unwrap();
        cache.entry(self.release).or_insert_with(|| {
            #[cfg(test)]
            {
                *SCHEMA_COUNT.get_or_init(|| Mutex::new(0)).lock().unwrap() += 1;
            }
            let parser_ctx = libxml::schemas::SchemaParserContext::from_file(xsd.to_str().expect("utf-8 path")); // CONFIRM-1
            libxml::schemas::SchemaValidationContext::from_parser(parser_ctx).expect("schema compiles") // CONFIRM-2
        });
        let ctx = cache.get_mut(self.release).unwrap();

        let ok = ctx.validate(&libxml_doc); // CONFIRM-3
        if ok {
            return Vec::new();
        }
        ctx.drain_errors()
            .into_iter()
            .map(|e| ValidationError {
                line: e.line.unwrap_or(0) as usize,
                column: e.column.unwrap_or(0) as usize,
                message: e.message.unwrap_or_default(),
            })
            .collect()
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
```

Export from `src/lib.rs`: `pub use validation::{ARXMLValidator, ValidationError};`

- [ ] **Step 5: Run the three tests; fix only CONFIRM-marked lines if the docs differ.** Then `cargo test --lib` (lib = 28 + 4 detection + 2 embed + 1 hash-sync + 3 core = 38).

- [ ] **Step 6: Gate + commit** — `feat(validation): ARXMLValidator (libxml compile cache + validate)`

### Task 3: reader integration — pre-parse gate in `ARXMLReader::load`

**Files:** Modify `src/reader/arxml_reader/mod.rs`, `src/reader/abstract_arxml_reader.rs` (ParseError variant).

- [ ] **Step 1: Write the failing tests** (in `mod.rs`'s `mod tests`):

```rust
    const SCHEMA_VALID_SAMPLE: &str = r#"<?xml version="1.0"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>P</SHORT-NAME></AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    const SCHEMA_INVALID_SAMPLE: &str = r#"<?xml version="1.0"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd">
  <ADMIN-DATA>
    <SDGS><SDG/></SDGS>
    <USED-LANGUAGES><L-10 L="EN">English</L-10></USED-LANGUAGES>
  </ADMIN-DATA>
</AUTOSAR>"#;

    #[test]
    fn load_rejects_schema_invalid_file_in_strict_mode() {
        let path = write_temp(SCHEMA_INVALID_SAMPLE);
        let mut document = Document::new();
        let result = ARXMLReader::new(ReaderOptions { warning: false, validate: true })
            .load(&path, &mut document);
        let Err(ParseError::SchemaValidation { count, .. }) = result else {
            panic!("expected SchemaValidation, got {result:?}");
        };
        assert!(count >= 1);
    }

    #[test]
    fn load_warns_and_continues_on_schema_invalid_file_in_warning_mode() {
        let path = write_temp(SCHEMA_INVALID_SAMPLE);
        let mut document = Document::new();
        let mut reader = ARXMLReader::new(default_options()); // warning: true, validate: true
        reader.load(&path, &mut document).unwrap();
        assert!(reader
            .get_warnings()
            .iter()
            .any(|w| w.contains("schema validation") || w.contains("USED-LANGUAGES")));
        // the document still parsed
        assert_eq!(document.get_ar_packages().len(), 1);
    }

    #[test]
    fn validate_false_disables_the_gate() {
        let path = write_temp(SCHEMA_INVALID_SAMPLE);
        let mut document = Document::new();
        ARXMLReader::new(ReaderOptions { warning: false, validate: false })
            .load(&path, &mut document)
            .unwrap();
    }

    #[test]
    fn load_still_works_for_unresolvable_schemas() {
        let path = write_temp(SAMPLE); // existing SAMPLE uses AUTOSAR_00050.xsd — not bundled
        let mut document = Document::new();
        let mut reader = ARXMLReader::new(ReaderOptions { warning: false, validate: true });
        reader.load(&path, &mut document).unwrap(); // no failure, no validation
        assert!(reader.get_warnings().is_empty());
    }
```

Plus a helper in the test module:

```rust
    fn write_temp(contents: &str) -> std::path::PathBuf {
        let path = tempfile::NamedTempFile::new().unwrap().into_temp_path().to_path_buf();
        std::fs::write(&path, contents).unwrap();
        path
    }
```

- [ ] **Step 2: Run to verify they fail** (compile error: no `validate` field on `ReaderOptions`, no `SchemaValidation` variant).

- [ ] **Step 3: Implement:**

`ReaderOptions` gains the field (both tests above construct it literally, so this is forced):

```rust
pub struct ReaderOptions {
    pub warning: bool,
    /// Pre-parse XSD gate (py `options={"validate": False}` escape hatch).
    pub validate: bool,
}

impl Default for ReaderOptions {
    fn default() -> Self {
        Self { warning: true, validate: true }
    }
}
```

`ParseError` gains (in `src/reader/abstract_arxml_reader.rs`):

```rust
    #[error("failed schema validation with {count} error(s) (first: line {line}, col {column}: {message})")]
    SchemaValidation {
        count: usize,
        line: usize,
        column: usize,
        message: String,
    },
```

`load` becomes the gated path (py §5 as amended — bytes first, then parse):

```rust
    /// py `load` — validates the bytes against the bundled release schema
    /// before any model construction (escape hatch: `ReaderOptions::validate`).
    pub fn load(&mut self, path: &Path, document: &mut Document) -> Result<(), ParseError> {
        let data = std::fs::read(path)?;
        self.validate_or_fail(&data)?;
        self.load_from_reader(Reader::from_reader(&data[..]), document)
    }

    /// Unresolvable schemas are NOT errors (spec §4 AMENDED): the writer's
    /// default `AUTOSAR_4-0-3.xsd` and legacy documents have no bundled
    /// schema — processing continues unvalidated in every mode.
    fn validate_or_fail(&mut self, data: &[u8]) -> Result<(), ParseError> {
        if !self.options.validate {
            return Ok(());
        }
        let Some(validator) = ARXMLValidator::for_document(data) else {
            return Ok(());
        };
        let errors = validator.validate(data);
        if errors.is_empty() {
            return Ok(());
        }
        for error in &errors {
            let message = format!(
                "schema validation ({}): line {}, col {}: {}",
                validator.release(),
                error.line,
                error.column,
                error.message
            );
            if self.options.warning {
                self.warnings.push(message);
            } else {
                return Err(ParseError::SchemaValidation {
                    count: errors.len(),
                    line: error.line,
                    column: error.column,
                    message: error.message.clone(),
                });
            }
        }
        Ok(())
    }
```

Note: in strict mode the first error aborts (matching the py amendment's "single error, first violation in the message"); in warning mode every violation becomes a warning line.

- [ ] **Step 4: Run the four tests to green.** Then the FULL gate — expect the existing `load_rejects_wrong_root` and other mod.rs tests to still pass (they go through `load` with `Reader::from_str`? NO — check: they call `load_from_reader` directly, which is UNGATED by design; only `write_temp`-based tests use `load`).

- [ ] **Step 5: Gate + commit** — `feat(validation): pre-parse XSD gate in ARXMLReader::load`

### Task 4: writer integration — pre-save gate in `ARXMLWriter::save`

**Files:** Modify `src/writer/arxml_writer/mod.rs`, `src/writer/abstract_arxml_writer.rs` (WriteError variant).

- [ ] **Step 1: Write the failing tests** (writer `mod tests`). The invalid document is built through the model API with a SHORT-NAME the XSD's identifier pattern rejects (the writer emits what the model holds — code_guide §8 — so a space/`!` in a package name serializes and MUST fail the gate; note the writer's fixed emission order makes ordering violations unconstructible, which is why this is the realistic invalid state):

```rust
    #[test]
    fn save_rejects_schema_invalid_document() {
        let mut document = Document::new();
        document.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd");
        let pkg_id = document.add_ar_package(None, "Pkg");
        document
            .ar_packages
            .get_mut(pkg_id)
            .unwrap()
            .set_short_name("bad name!"); // space + '!' violate the XSD identifier pattern

        let output = tempfile::NamedTempFile::new().unwrap();
        let result = ARXMLWriter::new().save(output.path(), &document);
        let Err(WriteError::SchemaValidation { count, .. }) = result else {
            panic!("expected SchemaValidation, got {result:?}");
        };
        assert!(count >= 1);
    }

    #[test]
    fn save_writes_when_validation_disabled() {
        let mut document = Document::new();
        document.set_schema_location("http://autosar.org/schema/r4.0 AUTOSAR_00046.xsd");
        let pkg_id = document.add_ar_package(None, "Pkg");
        document
            .ar_packages
            .get_mut(pkg_id)
            .unwrap()
            .set_short_name("bad name!");
        let output = tempfile::NamedTempFile::new().unwrap();
        ARXMLWriter::with_options(WriterOptions { unescape_entities: false, validate: false })
            .save(output.path(), &document)
            .unwrap();
        assert!(output.path().metadata().unwrap().len() > 0);
    }
```

- [ ] **Step 1b: Forced edit** — the existing `unescape_entities_option_unescapes_attribute_quotes` test constructs `WriterOptions { unescape_entities: true }`; adding the `validate` field breaks that literal. Update it to `WriterOptions { unescape_entities: true, validate: false }` (the unescape test must not newly depend on schema compilation).

- [ ] **Step 2: Implement** (adjust the invalid-document construction to a real schema violation discovered at Step 1's Red run — e.g. `document.set_admin_data(...)` with SDGS-before-USED-LANGUAGES built through the Document API, or an empty AR-PACKAGES wrapper; the gate code itself is fixed):

`WriterOptions` (manual `Default` — `validate` defaults to `true`):

```rust
pub struct WriterOptions {
    pub unescape_entities: bool,
    /// Pre-save XSD gate. Unlike the reader there is no warning mode here:
    /// the writer has no warning sink, so a failing gate is an error; set
    /// `validate = false` for the py `warning=True` write-anyway behavior.
    pub validate: bool,
}

impl Default for WriterOptions {
    fn default() -> Self {
        Self { unescape_entities: false, validate: true }
    }
}
```

`WriteError` gains:

```rust
    #[error("failed schema validation with {count} error(s) (first: line {line}, col {column}: {message})")]
    SchemaValidation {
        count: usize,
        line: usize,
        column: usize,
        message: String,
    },
```

`save` gate — inserted after the existing buffer is complete (post `unescape_entities` pass), before the file write:

```rust
        if self.options.validate {
            if let Some(validator) = ARXMLValidator::for_document(&buffer) {
                let errors = validator.validate(&buffer);
                if let Some(first) = errors.first() {
                    return Err(WriteError::SchemaValidation {
                        count: errors.len(),
                        line: first.line,
                        column: first.column,
                        message: first.message.clone(),
                    });
                }
            }
        }
```

- [ ] **Step 3: Tests to green + full gate.** The unresolvable-default case must still write (gate skips) — add:

```rust
    #[test]
    fn save_skips_gate_for_unresolvable_schema() {
        let mut document = Document::new(); // default AUTOSAR_4-0-3.xsd — not bundled
        document.add_ar_package(None, "Pkg");
        let output = tempfile::NamedTempFile::new().unwrap();
        ARXMLWriter::new().save(output.path(), &document).unwrap();
    }
```

- [ ] **Step 4: Gate + commit** — `feat(validation): pre-save XSD gate in ARXMLWriter::save`

### Task 5: corpus audit + close-out

**Files:** Modify `src/validation/mod.rs` (audit test), `README.md` (prerequisite line).

- [ ] **Step 1: Audit test** (in `src/validation`'s test module):

```rust
    #[test]
    fn integration_corpus_passes_its_detected_schema() {
        // Spec §7 (adapted): every fixture whose schema file IS bundled must
        // validate clean; fixtures referencing unbundled schema files
        // (AUTOSAR_00050.xsd etc.) take the unvalidated path — asserted here
        // so a future bundling of those sets flips them into this check.
        let mut validated = 0;
        for entry in std::fs::read_dir("tests/integration/test_files").unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|e| e != "arxml") {
                continue;
            }
            let data = std::fs::read(&path).unwrap();
            match detect_release(&data) {
                Some(release) => {
                    let validator = ARXMLValidator::for_document(&data).unwrap();
                    let errors = validator.validate(&data);
                    assert!(
                        errors.is_empty(),
                        "{} failed {} schema: {:?}",
                        path.display(),
                        release,
                        errors.first()
                    );
                    validated += 1;
                }
                None => {}
            }
        }
        assert_eq!(validated, 2, "expected Os_ECUC_4.4.0 + the R3.2.3 fixture to validate");
    }
```

- [ ] **Step 2: Run. If a real fixture FAILS validation:** STOP — do not fix the fixture (they are verbatim, Rule 0006) and do not weaken the assert; report the failure list for a decision (that is exactly the audit working as designed).

- [ ] **Step 3: README prerequisite + close-out**

`README.md` (root): under a build/deps note add: `XSD validation requires system libxml2 (pkg-config); disable the gates with ReaderOptions/WriterOptions { validate: false }`.

Final gates + py2rust check (checklist must be unchanged — no new read_/write_ methods) + commit: `feat(validation): corpus audit + docs (xsd validation complete)`.

Then push and open the PR:

```bash
git push -u origin feature/xsd-validation
gh pr create --base main --title "feat: XSD validation for parse and write" \
  --body "Rust port of docs/superpowers/specs/2026-10-05-xsd-validation-design.md (py-armodel lxml design → libxml crate). Closes #<tracking issue>"
```

---

**Done when:** `ARXMLReader::load` validates pre-parse (strict = `ParseError::SchemaValidation` after no model construction; warning mode = per-violation warnings + parse continues; `validate: false` = off; unresolvable schema = unvalidated in every mode); `ARXMLWriter::save` validates pre-write (`validate: false` escape); 4 release sets embedded byte-identical to the pin with a sync guard; corpus audit green (2 validated / 30 documented-unvalidated); no new `read_`/`write_` methods (checklist untouched).
