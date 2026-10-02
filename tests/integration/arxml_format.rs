//! The `arxml-format` pipeline at the library level (issue #8): parse →
//! optional transform → write, mirroring what `src/bin/arxml-format.rs`
//! performs. Written-file fidelity still tracks the P0-scope parser/writer;
//! per-class payload arrives with the P2–P4 port.

use std::path::Path;

use armodel::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackageId;
use armodel::parser::arxml_parser::{default_options, ARXMLParser};
use armodel::writer::arxml_writer::ARXMLWriter;
use armodel::{AdminDataTransformer, Document};

const FIXTURE: &str = "tests/integration/test_files/AdminDataWhitespace.arxml";

fn parse(path: &Path) -> Document {
    let mut document = Document::new();
    ARXMLParser::new(default_options())
        .load(path, &mut document)
        .unwrap();
    document
}

fn all_packages_clean(document: &Document, packages: &[ARPackageId]) -> bool {
    packages.iter().all(|id| {
        let package = document.get_ar_package(*id).unwrap();
        package.get_admin_data().is_none()
            && all_packages_clean(document, package.get_ar_packages())
    })
}

#[test]
fn format_round_trips_the_model() {
    let source = Path::new(FIXTURE);
    let document = parse(source);

    let output = tempfile::NamedTempFile::new().unwrap();
    ARXMLWriter::new().save(output.path(), &document).unwrap();

    let formatted = parse(output.path());
    document.assert_structurally_equal(&formatted).unwrap();
}

#[test]
fn remove_admin_data_strips_root_and_all_packages() {
    let mut document = parse(Path::new(FIXTURE));
    AdminDataTransformer::new().remove(&mut document);

    assert!(document.get_admin_data().is_none());
    assert!(all_packages_clean(&document, document.get_ar_packages()));

    // and the written file stays clean through a re-parse
    let output = tempfile::NamedTempFile::new().unwrap();
    ARXMLWriter::new().save(output.path(), &document).unwrap();
    let formatted = parse(output.path());
    assert!(formatted.get_admin_data().is_none());
    assert!(all_packages_clean(&formatted, formatted.get_ar_packages()));
}
