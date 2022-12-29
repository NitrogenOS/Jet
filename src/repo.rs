use serde::{Deserialize, Serialize};

mod lib;
use lib::version;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    name: String,
    version: version::Version,
    packages: Vec<Package>
}

pub struct Package {
    name: String,
    version: version::Version,
    source: String
}