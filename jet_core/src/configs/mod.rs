use std::path::PathBuf;

pub mod repo;
pub mod package;

pub trait Config {
    fn save(&self, path: PathBuf);
    fn load(path: PathBuf) -> Self;
}