use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub arch: Vec<PackageArch>,

    pub install: HashMap<String, InstallOpts>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum PackageArch {
    #[serde(rename = "x64")]
    X86,
    #[serde(rename = "x86")]
    X64,
}

impl From<String> for PackageArch {
    fn from(value: String) -> Self {
        match value.as_str() {
            "x64" => Self::X64,
            "x86" => Self::X86,
            _ => panic!("unknown arch"),
        }
    }
}

impl Into<String> for PackageArch {
    fn into(self) -> String {
        match self {
            PackageArch::X86 => "x86".to_string(),
            PackageArch::X64 => "x64".to_string(),
        }
    }
}

impl ToString for PackageArch {
    fn to_string(&self) -> String {
        match self {
            PackageArch::X86 => "x86".to_string(),
            PackageArch::X64 => "x64".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallOpts {
    pub path: String,
    pub executable: bool,
}

impl Config for PackageConfig {
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
