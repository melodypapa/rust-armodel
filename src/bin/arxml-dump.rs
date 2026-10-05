use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;

use armodel::reader::arxml_reader::{default_options, ARXMLReader};
use armodel::Document;

/// Dump the packages of an AUTOSAR ARXML file.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Set arxml file name
    #[arg(short = 'a', long = "arxml", value_name = "NAME")]
    arxml: PathBuf,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.arxml);

    let mut document = Document::new();
    if let Err(error) = ARXMLReader::new(default_options()).load(path, &mut document) {
        eprintln!("Failed to parse {path}: {error}", path = path.display());
        process::exit(1);
    }

    println!("AR release: {}", document.get_ar_release());
    for package_id in document.get_ar_packages() {
        match document.get_ar_package(*package_id) {
            Some(package) => {
                let name = package.get_short_name().unwrap_or("<no SHORT-NAME>");
                println!("AR-PACKAGE {name}");
            }
            None => println!("AR-PACKAGE <unresolved>"),
        }
    }
}
