//! Every fixture copied verbatim from py-armodel's
//! `tests/integration_tests/test_files` must parse, write, re-parse and
//! compare structurally equal (AGENTS.md: fixtures are never regenerated).
//!
//! py's own `test_roundtrip.py` additionally compares the written file
//! byte-for-byte against the original; that step is out of scope until the
//! parser/writer cover the full model (P2–P4) — here the model-level
//! roundtrip (parse → write → re-parse → `assert_structurally_equal`) is
//! what P1 can honestly verify.

use std::path::{Path, PathBuf};

use armodel::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ElementRef;
use armodel::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackageId;
use armodel::parser::arxml_parser::{default_options, ARXMLParser};
use armodel::writer::arxml_writer::ARXMLWriter;
use armodel::Document;

const FIXTURE_DIR: &str = "tests/integration/test_files";
/// The pinned py-armodel revision ships exactly this many `.arxml` fixtures
/// (plus `Os_ECUC.yaml`, which is not an ARXML file).
const FIXTURE_COUNT: usize = 32;

/// Fixtures whose parse must produce zero warnings — i.e. the parser
/// consumes every element they contain. A fixture joins this list exactly
/// when its batch lands (roadmap spec §5.3); byte-identity additionally
/// requires a FORMAT_SOURCES entry in format_byte_roundtrip.rs.
const WARNING_FREE_SOURCES: &[&str] = &[
    "AdminDataWhitespace.arxml",
    "AUTOSAR_MOD_AISpecification_BaseTypes_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_Body_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_Chassis_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_MmedTelmHmi_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_OccptPedSfty_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_Pt_Blueprint.arxml",
];

fn fixture_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(FIXTURE_DIR)
        .expect("fixture dir exists")
        .map(|entry| entry.expect("dir entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "arxml"))
        .collect();
    files.sort();
    files
}

/// parse → write → re-parse → structural equality, mirroring py's
/// `test_roundtrip.py` steps 1–4.
fn round_trip(file: &Path) -> Result<(), String> {
    let name = file
        .file_name()
        .expect("file name")
        .to_string_lossy()
        .into_owned();

    let mut document = Document::new();
    ARXMLParser::new(default_options())
        .load(file, &mut document)
        .map_err(|error| format!("{name}: parse failed: {error}"))?;

    let output =
        tempfile::NamedTempFile::new().map_err(|error| format!("{name}: tempfile: {error}"))?;
    ARXMLWriter::new()
        .save(output.path(), &document)
        .map_err(|error| format!("{name}: write failed: {error}"))?;

    let mut reparsed = Document::new();
    ARXMLParser::new(default_options())
        .load(output.path(), &mut reparsed)
        .map_err(|error| format!("{name}: re-parse failed: {error}"))?;

    document
        .assert_structurally_equal(&reparsed)
        .map_err(|error| format!("{name}: {error}"))
}

/// The P1 registry ingests ELEMENTS children: every package element must be
/// allocated in an arena and carry a SHORT-NAME. SoftwareComponents.arxml
/// holds 18 elements across 9 different tags, so the walk must see more than
/// just nested packages.
#[test]
fn elements_are_ingested_through_the_registry() {
    let file = Path::new(FIXTURE_DIR).join("SoftwareComponents.arxml");
    let mut document = Document::new();
    ARXMLParser::new(default_options())
        .load(&file, &mut document)
        .expect("SoftwareComponents.arxml parses");

    fn walk(document: &Document, packages: &[ARPackageId]) -> (usize, usize) {
        let mut total = 0;
        let mut non_package = 0;
        for package_id in packages {
            let package = document
                .get_ar_package(*package_id)
                .expect("package resolves");
            for element in package.get_elements() {
                total += 1;
                match element {
                    ElementRef::ARPackage(_) => {}
                    ElementRef::CompositionSwComponentType(id) => {
                        non_package += 1;
                        assert!(
                            document
                                .get_composition_sw_component_type(*id)
                                .expect("element resolves in its arena")
                                .get_short_name()
                                .is_some(),
                            "ingested element must carry a SHORT-NAME"
                        );
                    }
                    _ => non_package += 1,
                }
            }
            let (sub_total, sub_non_package) = walk(document, package.get_ar_packages());
            total += sub_total;
            non_package += sub_non_package;
        }
        (total, non_package)
    }

    let (total, non_package) = walk(&document, document.get_ar_packages());
    assert_eq!(total, 18, "every ELEMENTS child of the fixture is ingested");
    assert!(non_package > 0, "the fixture carries non-package elements");
}

/// Fixtures whose text still differs after a roundtrip because their payload
/// is not parsed/written yet — mirrors py-armodel's `KNOWN_ISSUES` mechanism.
/// Every entry is field-level content (LONG-NAMEs, per-class fields, ECUC
/// values) that arrives with the P2-P4 mass port; entries leave this list as
/// that work lands, and the text comparison then guards them automatically.
const P2_P4_PENDING: &[&str] = &[
    "AUTOSAR_Datatypes.arxml",
    "AUTOSAR_MOD_AISpecification_ApplicationDataType_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_ApplicationDataType_LifeCycle_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_CompuMethod_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_CompuMethod_LifeCycle_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_DataConstr_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_DataConstr_LifeCycle_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_KeywordSet_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Keyword_LifeCycle_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_PhysicalDimension_LifeCycle_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_PhysicalDimension_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_PortInterface_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_PortInterface_LifeCycle_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_PortPrototypeBlueprint_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_PortPrototypeBlueprint_LifeCycle_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_SwComponentTypes_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Unit_LifeCycle_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_Unit_Standard.arxml",
    "BswMMode.arxml",
    "BswM_Bswmd.arxml",
    "CanSystem.arxml",
    "Os_ECUC.arxml",
    "Os_ECUC_4.4.0.arxml",
    "SoftwareComponents.arxml",
    "SwRecordDemo.arxml",
];

/// py `normalize_xml_entities` + `normalize_xml_declaration_and_root`: fold
/// quote entities to characters, canonicalize the XML declaration and join a
/// root open tag whose attributes wrap over several lines. Formatting-only —
/// content is untouched.
fn normalize_arxml_text(text: &str) -> Vec<String> {
    let lines: Vec<String> = text
        .lines()
        .map(|l| l.replace("&quot;", "\"").replace("&apos;", "'"))
        .collect();
    let mut lines = lines;
    if lines
        .first()
        .is_some_and(|l| l.trim_start().starts_with("<?xml"))
    {
        lines[0] = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string();
    }

    let mut out = Vec::with_capacity(lines.len());
    let mut index = 0;
    while index < lines.len() {
        let stripped = lines[index].trim().to_string();
        if stripped.starts_with("<AUTOSAR") && !stripped.ends_with('>') {
            let mut joined = stripped;
            index += 1;
            while index < lines.len() {
                let part = lines[index].trim().to_string();
                joined.push(' ');
                joined.push_str(&part);
                if part.ends_with('>') {
                    break;
                }
                index += 1;
            }
            out.push(joined.split_whitespace().collect::<Vec<_>>().join(" "));
        } else {
            out.push(lines[index].clone());
        }
        index += 1;
    }
    out
}

/// py step 5's line-by-line comparison with two lines of context around the
/// first difference.
fn compare_lines(original: &[String], written: &[String]) -> Result<(), String> {
    if original.len() != written.len() {
        return Err(format!(
            "line count mismatch: original {} vs written {}",
            original.len(),
            written.len()
        ));
    }
    for (index, (expected, actual)) in original.iter().zip(written.iter()).enumerate() {
        if expected != actual {
            let start = index.saturating_sub(2);
            let end = (index + 4).min(original.len());
            let mut context = String::new();
            for i in start..end {
                context.push_str(&format!("{:>5} | {}\n", i + 1, original[i]));
                if i == index {
                    context.push_str(&format!("  vs | {}\n", written[i]));
                }
            }
            return Err(format!(
                "first text difference at line {}:\n{}",
                index + 1,
                context
            ));
        }
    }
    Ok(())
}

/// py `test_roundtrip.py` step 5: the written file must equal the original
/// line-for-line after formatting-only normalization. Files whose payload is
/// not yet parsed (P2-P4) are skipped via `P2_P4_PENDING`.
#[test]
fn written_output_matches_the_original_text_for_supported_files() {
    let files = fixture_files();
    let names: Vec<&str> = files
        .iter()
        .map(|file| file.file_name().unwrap().to_str().unwrap())
        .collect();
    for pending in P2_P4_PENDING {
        assert!(
            names.contains(pending),
            "stale P2_P4_PENDING entry (no such fixture): {pending}"
        );
    }

    let mut failures = Vec::new();
    for file in &files {
        let name = file.file_name().unwrap().to_str().unwrap();
        if P2_P4_PENDING.contains(&name) {
            continue;
        }
        let mut document = Document::new();
        ARXMLParser::new(default_options())
            .load(file, &mut document)
            .unwrap_or_else(|error| panic!("{name}: {error:?}"));
        let written = std::env::temp_dir().join(name);
        ARXMLWriter::new()
            .save(&written, &document)
            .unwrap_or_else(|error| panic!("{name}: {error:?}"));

        let original = normalize_arxml_text(&std::fs::read_to_string(file).unwrap());
        let generated = normalize_arxml_text(&std::fs::read_to_string(&written).unwrap());
        if let Err(message) = compare_lines(&original, &generated) {
            failures.push(format!("{name}: {message}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} fixture(s) differ textually:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn every_py_armodel_fixture_parses_and_round_trips() {
    let files = fixture_files();
    assert_eq!(
        files.len(),
        FIXTURE_COUNT,
        "fixture set drifted from the pinned py-armodel revision"
    );

    // one ok-line per fixture: `cargo test --test all_fixtures -- --nocapture`
    // (or --show-output) prints exactly which files were verified
    let failures: Vec<String> = files
        .iter()
        .filter_map(|file| match round_trip(file) {
            Ok(()) => {
                println!("ok: {}", file.display());
                None
            }
            Err(message) => Some(message),
        })
        .collect();
    assert!(
        failures.is_empty(),
        "{} of {} fixtures failed:\n{}",
        failures.len(),
        files.len(),
        failures.join("\n")
    );
}

/// Fixtures listed in WARNING_FREE_SOURCES must parse without a single
/// "unsupported element"-class warning — the parser-coverage gate (roadmap
/// spec §5.3). Everything else still round-trips at model level via the
/// warnings-and-skip path.
#[test]
fn warning_free_sources_parse_without_warnings() {
    let names: Vec<String> = fixture_files()
        .iter()
        .map(|file| file.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    for source in WARNING_FREE_SOURCES {
        assert!(
            names.contains(&source.to_string()),
            "stale WARNING_FREE_SOURCES entry (no such fixture): {source}"
        );
        let path = Path::new(FIXTURE_DIR).join(source);
        let mut document = Document::new();
        let mut parser = ARXMLParser::new(default_options());
        parser
            .load(&path, &mut document)
            .unwrap_or_else(|error| panic!("{source}: parse failed: {error}"));
        assert!(
            parser.get_warnings().is_empty(),
            "{source}: expected zero warnings, got: {:?}",
            parser.get_warnings()
        );
    }
}
