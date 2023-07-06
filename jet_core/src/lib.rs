mod configs;
pub use configs::package::*;
pub use configs::repo::*;
pub use configs::*;
mod jpk;
pub use jpk::*;

pub const REPO_CONFIG_PATH: &str = "/etc/jet/repo_list.cfg";
pub const JET_CONFIG_PATH: &str = "/etc/jet/jet.cfg";
pub const REPO_DB_PATH: &str = "/var/jet/db";