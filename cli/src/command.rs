use clap::{Arg, Command, ArgAction};

pub fn bundle() -> Command {
    Command::new("bundle")
        .about("Bundle directory containing package.toml into a jet package file")
        .arg(Arg::new("dir").num_args(1).required(true))
        .arg(
            Arg::new("arch")
                .short('a')
                .long("arch")
                .required(true)
                .help("arch to bundle for"),
        )
}

pub fn sync() -> Command {
    Command::new("sync")
        .about("get lastest database from repo")
        .alias("upgrade")
        .arg(
            Arg::new("only")
                .short('o')
                .long("only")
                .required(false)
                .help("Only get lastest database from specified repo"),
        )
}

pub fn install() -> Command {
    Command::new("install")
        .about("install a system package")
        .alias("add")
        .args([
            Arg::new("packages")
                .help("Packages you would like to install from repos")
                .num_args(1..)
                .required_unless_present("path"),
            Arg::new("path")
                .short('p')
                .long("path")
                .required(false)
                .help("install a package from file")
                .required_unless_present("packages"),
        ])
}

pub fn uninstall() -> Command {
    Command::new("uninstall")
        .about("remove a system package")
        .alias("remove")
        .args([Arg::new("packages")
            .help("Packages you would like to install from repos")
            .num_args(1..)
            .required(true)])
}

pub fn installed_packages() -> Command {
    Command::new("installed").alias("i")
}

pub fn query_packages() -> Command {
    Command::new("query").alias("q")
}

pub fn fix() -> Command {
    Command::new("fix").about("Fix missing files/directory if any are missings")
}

pub fn check() -> Command {
    Command::new("check")
        .about("Check for missing files/directory if any")
        .arg(
            Arg::new("fix")
                .long("fix")
                .short('f')
                .action(ArgAction::SetTrue)
                .help("after checking fix all the missing file/directories if any found"),
        )
}
