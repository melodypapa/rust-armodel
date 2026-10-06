//! `arxml-validator` — validate an AUTOSAR ARXML file against the bundled
//! AUTOSAR XSD for its release. Standalone verification only; it never
//! builds the model.
//!
//! Exit codes: 0 = valid · 1 = schema violations · 2 = unsupported (no
//! bundled schema for the detected/explicit release).

use std::path::PathBuf;
use std::process;

use clap::Parser;

use armodel::validation::{detect_schema_file, ARXMLValidator};

/// Validate an AUTOSAR ARXML file against the bundled AUTOSAR XSD.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Validate against this bundled release (e.g. R23-11) instead of the
    /// release detected from the file's xsi:schemaLocation.
    #[arg(long)]
    release: Option<String>,

    /// The path of the AUTOSAR ARXML file
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let data = match std::fs::read(&args.input) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", args.input.display());
            process::exit(1);
        }
    };

    let validator = match &args.release {
        Some(release) => ARXMLValidator::for_release(release),
        None => ARXMLValidator::for_document(&data),
    };
    let Some(validator) = validator else {
        // Unsupported schema is a FAILURE here (unlike the reader/writer
        // gates, which warn and continue unvalidated): a validation tool
        // that cannot resolve a schema must not report success.
        let detected = detect_schema_file(&data)
            .map(|f| String::from_utf8_lossy(f).into_owned())
            .unwrap_or_else(|| "<no xsi:schemaLocation>".to_string());
        eprintln!(
            "error: unsupported: no bundled XSD schema (requested: {detected}; bundled: R23-11, R4.4.0, R4.3.1, R3.2.3)"
        );
        process::exit(2);
    };

    let errors = validator.validate(&data);
    if errors.is_empty() {
        println!(
            "{}: valid against the {} schema",
            args.input.display(),
            validator.release()
        );
        return;
    }
    for error in &errors {
        eprintln!(
            "{}: line {}, col {}: {}",
            args.input.display(),
            error.line,
            error.column,
            error.message
        );
    }
    eprintln!(
        "{}: failed {} schema validation with {} error(s)",
        args.input.display(),
        validator.release(),
        errors.len()
    );
    process::exit(1);
}
