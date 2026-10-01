//! P0 acceptance (P0 design §2, §8):
//! parse → write → re-parse → assert_structurally_equal, plus the
//! whitespace-preserving SD text assertion.

use std::path::Path;

use armodel::m2::msr::documentation::text_model::language_data_model::XmlSpace;
use armodel::parser::arxml_parser::{default_options, ARXMLParser};
use armodel::writer::arxml_writer::ARXMLWriter;
use armodel::Document;

#[test]
fn admin_data_whitespace_roundtrip() {
    let source = Path::new("tests/integration/test_files/AdminDataWhitespace.arxml");

    // 1. parse into a Document
    let mut document = Document::new();
    ARXMLParser::new(default_options())
        .load(source, &mut document)
        .unwrap();

    // release detection: AUTOSAR_00050.xsd → R21-11
    assert_eq!(document.get_ar_release(), "R21-11");

    // 2. write to a temp file
    let output = std::env::temp_dir().join("armodel_roundtrip_AdminDataWhitespace.arxml");
    ARXMLWriter::new().save(&output, &document).unwrap();

    // 3. re-parse
    let mut reparsed = Document::new();
    ARXMLParser::new(default_options())
        .load(&output, &mut reparsed)
        .unwrap();

    // 4. structural equality (model equality only — no written-text diff in P0)
    document.assert_structurally_equal(&reparsed).unwrap();

    // 5. the whitespace-preserving field round-trips exactly
    let admin_data = document.get_admin_data().unwrap();
    let sdg = document.get_sdg(admin_data.get_sdgs()[0]).unwrap();
    let contents = document
        .get_sdg_contents(sdg.get_sdg_contents_type().unwrap())
        .unwrap();
    let sd = document.get_sd(contents.get_sd()[0]).unwrap();
    assert_eq!(sd.get_value(), Some("special   data"));
    assert_eq!(sd.get_xml_space(), Some(XmlSpace::Preserve));

    // and the used-languages entry too
    let paragraph = document
        .get_multi_language_plain_text(admin_data.get_used_languages().unwrap())
        .unwrap();
    let l10 = document.get_l_plain_text(paragraph.get_l10s()[0]).unwrap();
    assert_eq!(l10.get_value(), Some("English"));

    let _ = std::fs::remove_file(&output);
}
