use crate::config::Config;
use clap::{App, Arg, ArgMatches};

fn get_matches() -> ArgMatches<'static> {
    App::new("Rusted Ruins")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("fix-rand")
                .long("fix-rand")
                .help("Fixes the state of RNG when game start"),
        )
        .get_matches()
}

pub fn modify_config_by_args(mut config: Config) -> Config {
    let matches = get_matches();

    if matches.is_present("fix-rand") {
        config.fix_rand = true;
    }

    config
}
