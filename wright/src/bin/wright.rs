//! Command line app generated by clap.
use clap::{App, AppSettings, Arg, Values};

use std::process::exit;

use wright::interpreter::{Emit, Wright, Target, DEFAULT_TARGET};

/// Create and return the Wright command line app via [clap.rs](https://clap.rs).
pub fn get_wright_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Wright")
        .setting(AppSettings::ColorAlways)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("The Wright programming language interpreter and compiler.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input wright file(s).")
                .multiple(true)
                .required_unless("INTERACTIVE")
        )
        .arg(
            Arg::with_name("RUN")
                .help("Runs wright file(s) immediately.")
                .short("r")
                .long("run")
                .conflicts_with("INTERACTIVE")
        )
        .arg(
            Arg::with_name("INTERACTIVE")
                .help("Start an interactive wright session.")
                .short("i")
                .long("interactive")
                .conflicts_with_all(&["RUN", "EMIT", "TARGET"])
        )
        .arg(
            Arg::with_name("EMIT")
                .short("e")
                .long("emit")
                .help("Prints intermediate representation(s).")
                .takes_value(true)
                .possible_values(&["ast"])
                .use_delimiter(true)
                .multiple(true)
        )
        .arg(
            Arg::with_name("TARGET")
                .help("Set the compilation target")
                .long("target")
                .takes_value(true)
                .possible_values(&["treewalk"])
        )
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .help("Prints additional information.")
        )
}

fn main() {
    let matches = get_wright_app().get_matches();

    let filenames = matches
        .values_of("INPUT")
        .unwrap_or(Values::default())
        .collect();

    let emits: Vec<Emit> = matches
        .values_of("EMIT")
        .map(|vals: Values| vals.map(|s: &str| match s {
            "ast" => Emit::AbstractSyntaxTree,
            other => panic!("{} should not be a possible emit option.", other),
        }).collect())
        .unwrap_or(Vec::new());

    let target = matches.value_of("TARGET")
        .map(|s| match s {
            "treewalk" => Target::Treewalker,
            other => panic!("{} should not be a possible target option.", other),
        })
        .unwrap_or(DEFAULT_TARGET);

    let mut wright = Wright::new();
    wright
        .verbose(matches.is_present("VERBOSE"))
        .interactive(matches.is_present("INTERACTIVE"))
        .set_target(target)
        .set_emits(emits);
    if wright.add_files(filenames).is_err() {
        exit(exitcode::NOINPUT);
    }
    exit(wright.call());
}
