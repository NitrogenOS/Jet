use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use toml::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub license: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub copy: Option<HashMap<String, Value>>,
}

impl Config {
    pub fn new(
        name: String,
        description: String,
        version: String,
        author: String,
        license: String,
        dependencies: Vec<String>,
        copy: HashMap<String, Value>
    ) -> Config {
        return Config {
            name: name,
            description: description,
            version: version,
            author: Some(author),
            license: Some(license),
            dependencies: Some(dependencies),
            copy: Some(copy),
        };
    }

    pub fn load_from_file(file: &str) -> Config {
        let rawt_toml = fs::read_to_string(file).expect("couldn't read package.toml");

        let data: Config = toml::from_str(&rawt_toml).unwrap();
        return data;
    }
}
