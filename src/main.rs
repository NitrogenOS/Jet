use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    description: String,
    version: String,
    author: String,
    license: String,
}

fn main() {
    let matches = cli().get_matches();
    if let Some(sub_m) = matches.subcommand_matches("install") {
        let packages: Vec<&str> = sub_m
            .get_many::<String>("package")
            .expect("is present")
            .map(|s| s.as_str())
            .collect();
        for package in packages {
            println!("{}", package)
        }
    }
    if let Some(_) = matches.subcommand_matches("tempcmd") {
        let cfg = parse_config();
        println!("{:?}", cfg)
    }
}

fn cli() -> Command {
    let cmd = Command::new("jet")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("install")
                .about("install packages from repos")
                .args(&[Arg::new("package")
                    .help("package you would like to intsall")
                    .num_args(1..)]),
        )
        .subcommand(Command::new("update").about("updates packages from repos"))
        .subcommand(Command::new("upgrade").about("upgrades repos"))
        .subcommand(
            Command::new("search")
                .about("search all repo indexes for a package")
                .args(&[Arg::new("package")
                    .help("package you would like to find")
                    .num_args(1)]),
        )
        .subcommand(Command::new("tempcmd").about(""));

    return cmd;
}

fn parse_config() -> Config {
    let rawt_toml = fs::read_to_string("./package.toml").expect("couldn't read package.toml");

    let toml: Config = toml::from_str(&rawt_toml).expect("that shit aint work :(");
    return toml;
}
