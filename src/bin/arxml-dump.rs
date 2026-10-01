use std::env;
use std::path::Path;
use std::process;

use getopts::Options;

use armodel::parser::arxml_parser::{default_options, ARXMLParser};
use armodel::Document;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {program} [options]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "arxml", "Set arxml file name", "NAME");
    opts.optflag("h", "help", "Show this help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{f}");
            process::exit(1);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    let Some(path) = matches.opt_str("a") else {
        print_usage(&program, &opts);
        return;
    };

    let mut document = Document::new();
    if let Err(error) = ARXMLParser::new(default_options()).load(Path::new(&path), &mut document) {
        eprintln!("Failed to parse {path}: {error}");
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
