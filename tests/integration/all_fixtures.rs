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

#[test]
fn every_py_armodel_fixture_parses_and_round_trips() {
    let files = fixture_files();
    assert_eq!(
        files.len(),
        FIXTURE_COUNT,
        "fixture set drifted from the pinned py-armodel revision"
    );

    let failures: Vec<String> = files
        .iter()
        .filter_map(|file| round_trip(file).err())
        .collect();
    assert!(
        failures.is_empty(),
        "{} of {} fixtures failed:\n{}",
        failures.len(),
        files.len(),
        failures.join("\n")
    );
}
