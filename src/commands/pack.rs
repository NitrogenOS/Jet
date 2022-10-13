use std::{path::Path, fs::File};

use archiver_rs::Archive;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

use crate::package::Config;

struct Packer {}

impl Packer {
    pub fn new() -> &'static Self {
        &Self {}
    }

    pub fn unpack(&self, file: &str) {
        let mut tar = archiver_rs::Gzip::open(Path::new(file)).unwrap();
        let f = tar.files().unwrap();
        for file in f {
            println!("{}", file)
        }
    }

    pub fn pack(&self, dir: &str, out_file: &str) {
        let compressed_file = File::create(out_file).unwrap();
        let mut encoder = GzEncoder::new(compressed_file, Compression::default());

        {
            // Create tar archive and compress files
            let mut archive = Builder::new(&mut encoder);
            archive.append_dir_all(dir, dir);
        }

        // Finish Gzip file
        encoder.finish();
    }
}

pub fn pack(mut dir: &str) {
    if dir.ends_with("/") {
        dir = dir.strip_suffix("/").unwrap()
    }
    let cfg = Config::load_from_file(&format!("{}/package.toml", dir));

    let file = format!("{}-{}.jpk", cfg.name, cfg.version);

    let packer = Packer::new();
    packer.unpack(&file);
    // packer.pack(dir, &file);
}
