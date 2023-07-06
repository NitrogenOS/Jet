use std::{cmp::min,fs::File, thread, time::Duration, io::Write};

use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub async fn download(url: String) {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/db/fetch", url))
        .send()
        .await
        .expect("failed to download package");

    let total_size = resp.content_length().unwrap();

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
                    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
                    .progress_chars("=>-"));
    pb.set_message(format!("Downloading {}", "download_url"));

    let mut downloaded: u64 = 0;
    let mut stream = resp.bytes_stream();

    let mut file = File::create("path")
        .or(Err(format!("Failed to create file '{}'", "./temp")))
        .unwrap();

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

pub async fn test_download(mb: &mut MultiProgress, name: String) {
    let mut downloaded = 0;
    let total_size = 231231231;

    let pb = mb.add(ProgressBar::new(total_size));

    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{msg}\n[{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
            )
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message(format!("Downloading {name}"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}
