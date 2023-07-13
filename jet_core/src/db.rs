use std::{collections::HashMap, fs::{self, File}, path::Path};

use serde::{Deserialize, Serialize};

use crate::{Config, JET_DB};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDatabase {
    pub packages: HashMap<String, Vec<String>>,
}

impl Config for PackageDatabase {
    fn save(&self, path: std::path::PathBuf) {
        let bytes = bincode::serialize(&self).unwrap();

        fs::write(path, bytes).unwrap();
    }

    fn load(path: std::path::PathBuf) -> Self {
        let file_contents = fs::read(path).unwrap();
        let db: Self = match bincode::deserialize(&file_contents.as_slice()) {
            Ok(config) => config,
            Err(err) => panic!("Failed to deserialize file: {err}"),
        };

        db
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Database {
    pub installed_packages: HashMap<String, PackageData>
}

#[derive(Debug,Serialize, Deserialize)]
pub struct PackageData {
    pub version: String,
    pub files: Vec<String>,
    pub dependencies: Vec<(String, String)>,
    pub conflicts: Vec<(String, String)>
}

impl Database {
    pub fn save(&self) {
        let bytes = bincode::serialize(&self).unwrap();
        fs::write(JET_DB, bytes).unwrap();
    }

    pub fn load() -> Self {
        let db = Path::new(JET_DB);
        if !db.exists() {
            File::create(db).unwrap();
            let bytes = bincode::serialize(&Database::default()).unwrap();
            fs::write(db, bytes).unwrap();
        }
        let bytes: Vec<u8> = fs::read(db).unwrap();
        bincode::deserialize(bytes.as_slice()).unwrap()
    }
}