use clap::{Arg, Command};
mod commands;
use commands::{install, update, upgrade};
mod package;
use package::{Config};


fn main() {
    let matches = cli().get_matches();
    // Install
    if let Some(sub_m) = matches.subcommand_matches("install") {
        let packages: Vec<&str> = sub_m
            .get_many::<String>("package")
            .expect("is present")
            .map(|s| s.as_str())
            .collect();
        install::install(packages)
    }
    // Update
    if let Some(_) = matches.subcommand_matches("update") {
        update::update()
    }
    // Upgrade
    if let Some(_) = matches.subcommand_matches("update") {
        upgrade::upgrade()
    }
    // Tempcmd
    if let Some(_) = matches.subcommand_matches("tempcmd") {
        let cfg = Config::load_from_file("./package.toml");
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

