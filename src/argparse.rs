use super::api::Lang;
use super::api::PapagoReq;
use super::api::{NMT_URL, SMT_URL};
use clap::{App, Arg};

const LANG_PAPAGO: [&str; 9] = ["ko", "en", "cn", "tw", "es", "fr", "vi", "th", "id"];

pub struct CLIArgs {
    pub payload: PapagoReq,
    pub api: &'static str,
}

/// get_args returns command line arguments
pub fn get_args() -> CLIArgs {
    let matches = App::new("Papago API")
        .arg(
            Arg::with_name("TEXT")
                .help("Text to translate")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .takes_value(true)
                .default_value("en")
                .possible_values(&LANG_PAPAGO)
                .help("Source language"),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .default_value("ko")
                .possible_values(&LANG_PAPAGO)
                .help("Target language"),
        )
        .arg(
            Arg::with_name("nmt")
                .long("nmt")
                .help("USE NMT API (default)")
                .conflicts_with("smt"),
        )
        .arg(
            Arg::with_name("smt")
            .long("smt")
                .help("USE SMT API")
                .conflicts_with("nmt"),
        )
        .get_matches();

    let payload = PapagoReq {
        source: parse_lang(matches.value_of("source").unwrap()),
        target: parse_lang(matches.value_of("target").unwrap()),
        text: matches.value_of("TEXT").unwrap().to_owned(),
    };

    let api = if matches.is_present("smt") {
        SMT_URL
    } else {
        NMT_URL
    };

    CLIArgs { payload, api }
}

fn parse_lang(possible_lang: &str) -> Lang {
    match possible_lang.to_lowercase().as_str() {
        "ko" => Lang::Ko,
        "en" => Lang::En,
        "cn" => Lang::Cn,
        "tw" => Lang::Tw,
        "es" => Lang::Es,
        "fr" => Lang::Fr,
        "vi" => Lang::Vi,
        "th" => Lang::Th,
        "id" => Lang::Id,
        _ => panic!("WRONG INPUT BUT YOU SHOULD NEVER SEE THIS"),
    }
}
