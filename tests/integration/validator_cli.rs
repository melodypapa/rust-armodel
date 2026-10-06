//! End-to-end tests for the `arxml-validator` CLI exit-code contract.

use std::process::Command;

fn run(args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_arxml-validator"))
        .args(args)
        .output()
        .expect("binary runs");
    (
        output.status.code().unwrap_or(-1),
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    )
}

#[test]
fn valid_fixture_exits_zero() {
    let (code, out) = run(&["tests/integration/test_files/Os_ECUC_4.4.0.arxml"]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("valid against the R4.4.0 schema"), "{out}");
}

#[test]
fn unresolvable_schema_exits_two_with_unsupported_message() {
    // AUTOSAR_Datatypes.arxml references AUTOSAR_4-0-3.xsd — not bundled.
    let (code, err) = run(&["tests/integration/test_files/AUTOSAR_Datatypes.arxml"]);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("unsupported"), "{err}");
    assert!(err.contains("AUTOSAR_4-0-3.xsd"), "{err}");
}

#[test]
fn explicit_release_override_downgrades_to_violations_or_valid() {
    // --release forces a bundled schema even when the file asks for another;
    // the run then reports real violations (1) or validity (0) — never 2.
    let (code, _) = run(&[
        "--release",
        "R4.3.1",
        "tests/integration/test_files/Os_ECUC_4.4.0.arxml",
    ]);
    assert!(code == 0 || code == 1);
}
