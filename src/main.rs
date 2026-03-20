use std::process;

use nkf_rust::cli;
use nkf_rust::pipeline;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let options = match cli::parse_args(args) {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("nkf: {}", e);
            process::exit(1);
        }
    };

    if options.show_help {
        cli::print_help();
        return;
    }

    if options.show_version {
        cli::print_version();
        return;
    }

    if let Err(e) = pipeline::run(&options) {
        eprintln!("nkf: {}", e);
        process::exit(1);
    }
}
