use crate::config::Config;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Language
    #[clap(long)]
    lang: Option<String>,
    /// Second language
    #[clap(long)]
    second_lang: Option<String>,
    /// Use fixed random seed
    #[clap(long)]
    fix_rand: bool,
}

pub fn modify_config_by_args(mut config: Config) -> Config {
    let args = Args::parse();

    config.fix_rand = args.fix_rand;

    if let Some(lang) = args.lang {
        config.lang = lang;
    }
    if let Some(second_lang) = args.second_lang {
        config.second_lang = second_lang;
    }

    config
}
