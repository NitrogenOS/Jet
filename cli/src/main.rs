use clap::{ArgMatches, Command};
use nix::unistd::Uid;

mod command;

#[tokio::main]
async fn main() {
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions");
    }

    let cli: ArgMatches = cli().get_matches();


    if let Some(args) = cli.subcommand_matches("bundle") {
        let dir = args.get_one::<String>("dir").unwrap();
        let arch = args.get_one::<String>("arch").unwrap();
        jet_build::bundle(dir, arch);
    }

    if let Some(args) = cli.subcommand_matches("sync") {
        match args.get_one::<String>("only") {
            Some(repo) => jet_upgrade::upgrade_one(repo.to_owned()).await,
            None => jet_upgrade::upgrade_all().await,
        }
    };

    // if let Some(args) = cli.subcommand_matches("fix") {
    // }

    // if let Some(args) = cli.subcommand_matches("check") {
    // }
}

fn cli() -> clap::Command {
    Command::new("jet")
        .about("A blazingly fast package manager for NitrogenOS")
        .subcommand(command::bundle())
        .subcommand(command::sync())
}
