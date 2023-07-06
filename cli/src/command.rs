use clap::{Arg, Command};

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
