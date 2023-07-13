mod configs;
use std::path::Path;

pub use configs::package::*;
pub use configs::repo::*;
pub use configs::*;
mod jpk;
pub use jpk::*;
mod db;
pub use db::*;
mod utils;
pub use utils::*;

pub const REPO_CONFIG_PATH: &str = "/etc/jet/repo_list.cfg";
pub const JET_CONFIG_PATH: &str = "/etc/jet/jet.cfg";
pub const JET_DB: &str = "/etc/jet/jet.db";
pub const REPO_DB_PATH: &str = "/var/jet/db";
pub const PKG_ARCHIVE_PATH: &str = "/var/jet/pkg";

pub fn fs_copy(from: String, to: String) {
    let path_0 = Path::new(&from);
    let path_1 = Path::new(&to);
    std::fs::copy(path_0, path_1).unwrap();
}

pub fn rs_remove(file: String) {
    let path = Path::new(&file);

    if path.is_dir() {
        std::fs::remove_dir_all(path).unwrap();
    } else if path.is_file() {
        std::fs::remove_file(path).unwrap();
    }
}
