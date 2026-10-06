//! `arxml-format` — load an AUTOSAR ARXML file and write it back formatted
//! (mirrors py-armodel's `arxml_format_cli.py`).

use std::path::PathBuf;
use std::process;

use clap::Parser;

use armodel::reader::arxml_reader::{ARXMLReader, ReaderOptions};
use armodel::writer::arxml_writer::{ARXMLWriter, WriterOptions};
use armodel::{AdminDataTransformer, Document};

/// Load an AUTOSAR ARXML file and write it back formatted.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Print debug information
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Log all information to file
    #[arg(long, value_name = "FILE")]
    log: Option<PathBuf>,

    /// Skip the error and report it as warning message
    #[arg(short = 'w', long)]
    warning: bool,

    /// Remove all the AdminData
    #[arg(long = "remove-admin-data")]
    remove_admin_data: bool,

    /// Unescape XML quote entities (quot, apos) in output
    #[arg(long = "unescape-entities")]
    unescape_entities: bool,

    /// Disable XSD schema validation on load and save.
    /// Legacy escape hatch for files without a bundled schema or with known deviations.
    #[arg(long = "no-validate")]
    no_validate: bool,

    /// The path of AUTOSAR ARXML file
    input: PathBuf,

    /// The path of output ARXML file
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    // py passes warning through to the parser; without -w the parser fails
    // on the first problem (py's AbstractARXMLParser default is warning=False).
    let mut document = Document::new();
    if let Err(error) = ARXMLReader::new(ReaderOptions {
        warning: args.warning,
        validate: !args.no_validate,
    })
    .load(&args.input, &mut document)
    {
        eprintln!("[ERROR] : {error}");
        process::exit(1);
    }

    if args.remove_admin_data {
        AdminDataTransformer::new().remove(&mut document);
    }

    // py's verbose mode attaches a file handler (creating the log next to the
    // output, or at --log) before saving; the debug message capture itself is
    // wired when the parser/writer port reports messages.
    if args.verbose {
        let log_path = args.log.clone().unwrap_or_else(|| {
            args.output
                .parent()
                .filter(|dir| !dir.as_os_str().is_empty())
                .map(|dir| dir.join("arxml_format.log"))
                .unwrap_or_else(|| PathBuf::from("arxml_format.log"))
        });
        if let Err(error) = std::fs::File::create(&log_path) {
            eprintln!("[ERROR] : {error}");
            process::exit(1);
        }
    }

    let writer = ARXMLWriter::with_options(WriterOptions {
        unescape_entities: args.unescape_entities,
        validate: !args.no_validate,
    });
    if let Err(error) = writer.save(&args.output, &document) {
        eprintln!("[ERROR] : {error}");
        process::exit(1);
    }
}
