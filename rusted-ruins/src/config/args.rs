use crate::config::Config;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Use fixed random seed
    #[clap(long)]
    fix_rand: bool,
}

pub fn modify_config_by_args(mut config: Config) -> Config {
    let args = Args::parse();

    config.fix_rand = args.fix_rand;

    config
}
