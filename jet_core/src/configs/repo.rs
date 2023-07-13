use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub repo: Vec<Repo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub r#type: RepoType,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RepoType {
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "jet")]
    Jetlag,
}

impl Config for RepoConfig {
    fn save(&self, path: PathBuf) {
        let config_string = toml::to_string(self).unwrap();
        fs::write(path, config_string).unwrap();
    }

    fn load(path: PathBuf) -> Self {
        let file_contents = fs::read_to_string(path).unwrap();
        let cfg: Self = match toml::from_str(&file_contents) {
            Ok(config) => config,
            Err(err) => panic!("Failed to deserialize file: {err}"),
        };

        cfg
    }
}
