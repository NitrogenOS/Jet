use crate::lib::download;

pub async fn install(packages: Vec<&str>) {
    for package in packages {
        print!("{}, ", package);
        download::download(package).await
    }

}