use clap::{App, Arg, SubCommand};
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
        .about("Wallets for DotSama networks with support for Moonbeam and Moonriver smart contracts, and vanity addresses generation")
        .subcommand(SubCommand::with_name("create")).about("Creates a new wallet")
        .arg(
            Arg::with_name("network")
                .short('n')
                .long("network")
                .help("Chose network")
                .required(true)
                .takes_value(true)
        )
        .subcommand(SubCommand::with_name("connect")).about("Connect to a wallet")
        .subcommand(SubCommand::with_name("import")).about("Imports a wallet from a mnemonic phrase")
        .subcommand(SubCommand::with_name("export")).about("Exports a wallet to a mnemonic phrase")
        .subcommand(SubCommand::with_name("balance")).about("Gets the balance of a wallet")
        .arg(
            Arg::with_name("network")
                .short('n')
                .long("network")
                .help("Chose network")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("address")
                .value_name("address")
                .help("Input file(s)")
                .multiple_occurrences(false)
        )
        .subcommand(SubCommand::with_name("transfer")).about("Transfer tokens from a wallet to another")
        .subcommand(SubCommand::with_name("contract")).about("Interact with a smart contract")
        .subcommand(SubCommand::with_name("vanity")).about("Generates a vanity address")
        .get_matches();

    Ok(Config {
    })
}

pub fn connect() {}

pub fn create_wallet() {}

pub fn create_vanity_address() {}