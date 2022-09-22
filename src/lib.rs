use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type OpsResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
}

pub struct Wallet {}

pub struct Contract {}

pub fn run(config: Config) -> OpsResult<()> {
    Ok(())
}

pub fn get_args() -> OpsResult<Config> {
    let matches = App::new("lunaire")
        .version("0.1.0")
        .author("Paul Vidal <u1d4lp@gmail.com>")
        .about("Wallets for DotSama networks with support for Moonbeam and Moonriver smart contracts")
        .arg(
            Arg::with_name("addresses")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .max_values(2)
        )
        .arg(
            Arg::with_name("network")
                .short('n')
                .long("network")
                .help("Chose network")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("balance")
                .short('b')
                .long("balance")
                .help("Fetch balance")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
    })
}

pub fn connect() {}

pub fn create_wallet() {}

pub fn create_vanity_address() {}