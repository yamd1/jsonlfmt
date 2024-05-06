use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("jsonlfmt")
        .version("0.1.0")
        .author("yamd1")
        .about("Json Lines formatter")
        .arg(
            Arg::new("in_file")
                .help("")
                .value_name("IN_FILE")
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .help("")
                .value_name("OUT_FILE")
                .action(ArgAction::Append)
                .default_value(None)
                .last(true),
        )
        .get_matches();

    let in_file = matches.get_one::<String>("in_file").unwrap().to_owned();
    let out_file = matches.get_one::<String>("out_file").to_owned().cloned();

    Ok(Config { in_file, out_file })
}

#[test]
pub fn stdio() {}

type ResultA<T> = Result<Option<T>, Box<dyn Error>>;
