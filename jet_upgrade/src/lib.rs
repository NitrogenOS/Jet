use std::path::Path;

use futures::future;
use indicatif::MultiProgress;
use jet_core::{repo, Config, RepoConfig, REPO_CONFIG_PATH};
use tokio::task::JoinHandle;
mod utils;

pub async fn upgrade_all() {
    let repos = load_config();
    let mut tasks: Vec<JoinHandle<Result<(), ()>>> = vec![];
    let mb = MultiProgress::new();

    for repo in repos {
        let repo = repo.clone();
        let mut mb_clone = mb.clone();
        
        match repo.r#type {
            jet_core::RepoType::Git => todo!(),
            jet_core::RepoType::Jetlag => tasks.push(tokio::spawn(async move {
                utils::test_download(&mut mb_clone, repo.name).await;
                Ok(())
            })),
        }
    }

    future::join_all(tasks).await;
}

pub async fn upgrade_one(repo_name: String) {
    let repos: Vec<jet_core::Repo> = load_config()
        .into_iter()
        .filter(|x| x.name == repo_name)
        .collect();
    println!("only syncing {:?}", repos[0].name)
}

fn load_config() -> Vec<repo::Repo> {
    RepoConfig::load(Path::new(REPO_CONFIG_PATH).to_path_buf()).repo
}
