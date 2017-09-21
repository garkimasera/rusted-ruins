
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate tar;
#[macro_use]
extern crate error_chain;
extern crate getopts;
extern crate image;
extern crate rusted_ruins_common as common;

use getopts::Options;
use std::env;
use std::process::exit;

mod verbose;
#[macro_use]
mod tomlinput;
mod buildobj;
mod compile;
mod dir;
mod error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = create_opts();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            exit(1);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    let files = matches.free.clone();
    if files.is_empty() {
        print_usage(&program, opts);
        return;
    }

    // Verbose mode
    if matches.opt_present("v") {
        verbose::set_verbose(true);
    }

    // Print infomation of pak files
    if matches.opt_present("i") {
        print_info(&files);
        return;
    }

    let output_file: String = match matches.opt_str("o") {
        Some(f) => f.to_string(),
        None => {
            let mut f = files[0].clone();
            f.push_str(".pak");
            f
        },
    };

    compile::compile(&files, &output_file);

    println!("{:?}", files);
}

fn print_info(files: &Vec<String>) {
    println!("Infomation of {:?} will be printed", files);
}

fn create_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help");
    opts.optflag("v", "verbose", "Verbose mode");
    opts.optflag("i", "info", "Print given pakage file information");
    opts.optopt("o", "", "Set output pakage file name", "NAME");
    opts
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}


