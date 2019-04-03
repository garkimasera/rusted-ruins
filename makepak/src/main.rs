#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_geom as geom;

mod verbose;
#[macro_use]
mod tomlinput;
mod buildobj;
mod compile;
mod dir;
mod error;
mod rrscript;

fn main() {
    let matches = create_matches();

    // Input files
    let files: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    if files.is_empty() {
        return;
    }

    // Verbose mode
    if matches.is_present("verbose") {
        verbose::set_verbose(true);
    }

    // Print infomation of pak files
    if matches.is_present("info") {
        print_info(&files);
        return;
    }

    let output_file: String = if let Some(f) = matches.value_of("output") {
        f.to_owned()
    } else {
        let mut f = files[0].to_string();
        f.push_str(".pak");
        f
    };

    compile::compile(&files, &output_file);
}

fn print_info(files: &[&str]) {
    println!("Infomation of {:?} will be printed", files);
}

fn create_matches() -> clap::ArgMatches<'static> {
    use clap::{App, Arg};

    App::new("rusted-ruins-makepak")
        .about("Pak file maker for Rusted Ruins")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Verbose mode"),
        )
        .arg(
            Arg::with_name("info")
                .short("i")
                .long("info")
                .help("Print given pak file information"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Set output pakage file name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input toml files")
                .index(1)
                .multiple(true)
                .required(true),
        )
        .get_matches()
}
