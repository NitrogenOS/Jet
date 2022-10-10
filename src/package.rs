use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    name: String,
    description: String,
    version: String,
    author: String,
    license: String,
    dependencies: Vec<String>,
}

impl Config {
    pub fn new(
        name: String,
        description: String,
        version: String,
        author: String,
        license: String,
        dependencies: Vec<String>,
    ) -> Config {
        return Config {
            name: name,
            description: description,
            version: version,
            author: author,
            license: license,
            dependencies: dependencies,
        };
    }

    pub fn load_from_file(file: &str) -> Config {
        let rawt_toml = fs::read_to_string(file).expect("couldn't read package.toml");

        let data: Config = toml::from_str(&rawt_toml).unwrap();
        return data;
    }
}
