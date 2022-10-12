use crate::package::Config;

pub fn pack(dir: &str) {
    let cfg = Config::load_from_file(&format!("{}/package.tom", dir));
}