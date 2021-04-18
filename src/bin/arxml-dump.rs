use armodel::ARPackage;
use getopts::Options;
use std::env;

fn main() {
    let _ = ARPackage::new();
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("a", "arxml", "Set arxml file name", "NAME");
    opts.optflag("h", "help", "Show this help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h"){
        print!("Help");
        return;
    }
}
