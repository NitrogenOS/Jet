use flate2::{read, write, Compression};
use std::{fs::File, path::{Path, PathBuf}};
use tar::Archive;

pub struct JetPackageFormat {
    input: String,
    output: String,
}

impl JetPackageFormat {
    pub fn new_package(io: (String, String)) -> Self {
        Self {
            input: io.0,
            output: io.1,
        }
    }

    pub fn compress(&self) {
        let jpk = File::create(format!("{}.jpk", self.output)).unwrap();
        let mut enc = write::GzEncoder::new(jpk, Compression::default());
        let mut tar = tar::Builder::new(&mut enc);
        tar.append_dir_all(".", Path::new(&self.input))
            .unwrap();
        tar.finish().unwrap();
    }

    pub fn decompres_file(path: &str, dst: &PathBuf) {
        let jpk = File::open(path).unwrap();
        let tar = read::GzDecoder::new(jpk);
        let mut archive = Archive::new(tar);
        archive.unpack(dst).unwrap();
    }

    pub async fn decompres(&self, dest: Option<&str>) {
        let jpk = File::open(format!("{}.jpk", self.output)).unwrap();
        let tar = read::GzDecoder::new(jpk);
        let mut archive = Archive::new(tar);
        let output = match dest {
            Some(output) => output.to_string(),
            None => self.input.clone(),
        };
        archive.unpack(output).unwrap();
    }
}
