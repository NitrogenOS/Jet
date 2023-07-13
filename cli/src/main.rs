use clap::{ArgMatches, Command};
use nix::unistd::Uid;

mod command;

#[tokio::main]
async fn main() {
    let cli: ArgMatches = cli().get_matches();

    if let Some(args) = cli.subcommand_matches("bundle") {
        let dir = args.get_one::<String>("dir").unwrap();
        let arch = args.get_one::<String>("arch").unwrap();
        jet_build::bundle(dir, arch);
    } else if let Some(args) = cli.subcommand_matches("sync") {
        if !Uid::effective().is_root() {
            panic!("You must run this executable with root permissions");
        }

        match args.get_one::<String>("only") {
            Some(repo) => jet_upgrade::upgrade_one(repo.to_owned()).await,
            None => jet_upgrade::upgrade_all().await,
        }
    } else if let Some(args) = cli.subcommand_matches("install") {
        if !Uid::effective().is_root() {
            panic!("You must run this executable with root permissions");
        }

        match args.get_one::<String>("path") {
            Some(path) => jet_install::install_file(path),
            None => {
                let packages: Vec<&String> = args.get_many::<String>("packages").unwrap().collect();
                jet_install::install(packages).await;
            }
        }
    } else if let Some(cmds) = cli.subcommand_matches("packages") {
        if !Uid::effective().is_root() {
            panic!("You must run this executable with root permissions");
        }

        if let Some(_) = cmds.subcommand_matches("installed") {
            let jet_db = jet_core::Database::load();

            if jet_db.installed_packages.is_empty() {
                println!("No packages installed")
            } else {
                println!("installed packages:");
                for key in jet_db.installed_packages.keys() {
                    println!(
                        "{key}@{}",
                        jet_db.installed_packages.get(key).unwrap().version
                    )
                }
            }
        } else if let Some(_args) = cmds.subcommand_matches("query") {
        }
    } else if let Some(args) = cli.subcommand_matches("uninstall") {
        if !Uid::effective().is_root() {
            panic!("You must run this executable with root permissions");
        }

        let packages: Vec<&String> = args.get_many::<String>("packages").unwrap().collect();

        for package in packages {
            jet_uninstall::uninstall(package)
        }
    } else if let Some(_args) = cli.subcommand_matches("fix") {
        jet_core::fix();
    } else if let Some(args) = cli.subcommand_matches("check") {
        let fix = args.get_one::<bool>("fix").unwrap();

        jet_core::check();

        if fix.to_owned() {
            jet_core::fix()
        }
    }
}

fn cli() -> clap::Command {
    Command::new("jet")
        .about("A blazingly fast package manager for NitrogenOS")
        .subcommand(command::bundle())
        .subcommand(command::sync())
        .subcommand(command::install())
        .subcommand(command::uninstall())
        .subcommand(
            Command::new("packages")
                .alias("pkg")
                .subcommand(command::installed_packages())
                .subcommand(command::query_packages()),
        )
        .subcommand(command::fix())
        .subcommand(command::check())
}
