#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

#[macro_use]
extern crate serde_derive;
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_geom as geom;

mod verbose;
#[macro_use]
mod input;
mod buildobj;
mod compile;
mod dir;
mod error;
mod pyscript;

use clap::{IntoApp, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    input_files: Vec<PathBuf>,
    #[clap(short, long)]
    output: Option<PathBuf>,
    #[clap(long)]
    info: bool,
    #[clap(long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    verbose::set_verbose(args.verbose);

    if args.input_files.is_empty() {
        let _ = <Args as IntoApp>::into_app().print_help();
        return;
    }

    // Print information of pak files
    if args.info {
        print_info(&args.input_files);
    }

    let output_file = args.output.unwrap_or_else(|| {
        let mut path = args.input_files[0].clone();
        path.set_extension("pak");
        path
    });

    compile::compile(&args.input_files, &output_file);
}

fn print_info(files: &[PathBuf]) {
    println!("Information of {:?} will be printed", files);
}
