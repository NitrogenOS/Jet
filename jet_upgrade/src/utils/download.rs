use std::{cmp::min, fs::{File, self}, io::Write, path::Path};

use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use jet_core::REPO_DB_PATH;

pub async fn download(mb: &mut MultiProgress, name: String, url: String) {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/db/fetch", url))
        .send()
        .await
        .expect("failed to download package");

    let total_size = resp.content_length().unwrap();

    let pb = mb.add(ProgressBar::new(total_size));
    pb.set_style(ProgressStyle::default_bar()
                    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
                    .progress_chars("=>-"));
    pb.set_message(format!("Syncing {}", name));

    let mut downloaded: u64 = 0;
    let mut stream = resp.bytes_stream();

    if !Path::new(REPO_DB_PATH).exists() {
        fs::create_dir_all(Path::new(REPO_DB_PATH)).unwrap();
    }

    let file = File::create(Path::new(REPO_DB_PATH).join(format!("{name}.db")));

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
    }
}
