//! Byte-level format check (py `test_roundtrip.py` step 5): `arxml-format`
//! reads a source file from `tests/integration/test_files/`, writes it to
//! `data/`, and the two files must be identical in binary mode.
//!
//! `FORMAT_SOURCES` is the configuration: a file only earns a place here once
//! the writer reproduces it byte-for-byte, which today requires full parser/
//! writer coverage of every element it uses. The P0-scope writer keeps only
//! the common Identifiable parts, so the list starts with the one fixture
//! that round-trips exactly; entries are added as the P2–P4 port extends
//! coverage (the model-level equivalent over all 32 fixtures lives in
//! `all_fixtures.rs`).

use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURE_DIR: &str = "tests/integration/test_files";
const OUTPUT_DIR: &str = "data";

/// Files that must survive `arxml-format` byte-identically.
const FORMAT_SOURCES: &[&str] = &[
    "AdminDataWhitespace.arxml",
    "AUTOSAR_MOD_AISpecification_BaseTypes_Standard.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_Body_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_Chassis_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_MmedTelmHmi_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_OccptPedSfty_Blueprint.arxml",
    "AUTOSAR_MOD_AISpecification_Collection_Pt_Blueprint.arxml",
];

fn first_difference(expected: &[u8], actual: &[u8]) -> Option<String> {
    let offset = expected
        .iter()
        .zip(actual.iter())
        .position(|(a, b)| a != b)
        .unwrap_or_else(|| expected.len().min(actual.len()));
    let context = |bytes: &[u8]| {
        let start = offset.saturating_sub(16);
        let end = (offset + 48).min(bytes.len());
        String::from_utf8_lossy(&bytes[start..end]).into_owned()
    };
    Some(format!(
        "first difference at byte {offset}: expected {:?}..., got {:?}...",
        context(expected),
        context(actual)
    ))
}

#[test]
fn format_output_is_byte_identical_for_configured_sources() {
    let binary = env!("CARGO_BIN_EXE_arxml-format");
    std::fs::create_dir_all(OUTPUT_DIR).expect("output dir can be created");

    let mut failures: Vec<String> = Vec::new();
    for source in FORMAT_SOURCES {
        let input = Path::new(FIXTURE_DIR).join(source);
        let output: PathBuf = Path::new(OUTPUT_DIR).join(source);

        let status = Command::new(binary)
            .arg(&input)
            .arg(&output)
            .status()
            .expect("arxml-format binary runs");
        if !status.success() {
            failures.push(format!("{source}: arxml-format exited with {status}"));
            continue;
        }

        let expected = std::fs::read(&input).expect("source file is readable");
        let actual = std::fs::read(&output).expect("formatted file is readable");
        if expected != actual {
            let detail = if expected.len() == actual.len() {
                first_difference(&expected, &actual).unwrap()
            } else {
                format!(
                    "size mismatch: source is {} bytes, formatted is {} bytes ({})",
                    expected.len(),
                    actual.len(),
                    first_difference(&expected, &actual)
                        .unwrap_or_else(|| "prefix is identical".into())
                )
            };
            failures.push(format!("{source}: {detail}"));
        }
    }

    assert!(
        failures.is_empty(),
        "byte comparison failed for {} of {} file(s):\n{}",
        failures.len(),
        FORMAT_SOURCES.len(),
        failures.join("\n")
    );
}
