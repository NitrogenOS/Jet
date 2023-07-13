use std::{
    cmp::min,
    fs::{self, File},
    io::Write,
    path::Path,
};

use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use jet_core::PKG_ARCHIVE_PATH;

pub async fn download(
    mb: &mut MultiProgress,
    url: String,
    name: String,
    version: String,
    arch: String,
) -> String {
    let client = reqwest::Client::new();
    let path = Path::new(PKG_ARCHIVE_PATH).join(format!("{name}-{version}-{arch}.jpk"));

    if path.exists() {
        return String::from(path.to_str().unwrap());
    }

    let resp = client
        .get(&format!("{url}/{name}/{version}?arch={arch}"))
        .send()
        .await
        .expect("failed to download package");

    let total_size = resp.content_length().unwrap();

    let pb = mb.add(ProgressBar::new(total_size));
    pb.set_style(ProgressStyle::default_bar()
                    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
                    .progress_chars("=>-"));
    pb.set_message(format!("Syncing {}@{}", name, version));

    let mut downloaded: u64 = 0;
    let mut stream = resp.bytes_stream();

    let file = File::create(&path);

    if !Path::new(PKG_ARCHIVE_PATH).exists() {
        fs::create_dir_all(Path::new(PKG_ARCHIVE_PATH)).unwrap();
    }

    if let Err(err) = file {
        panic!("error {err}")
    } else if let Ok(mut file) = file {
        while let Some(item) = stream.next().await {
            let chunk = item
                .or(Err(format!("Error while downloading file")))
                .unwrap();
            file.write_all(&chunk)
                .or(Err(format!("Error while writing to file")))
                .unwrap();
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }
        println!("Finished downloading")
    }

    String::from(path.to_str().unwrap())
}
