use indicatif::MultiProgress;
use jet_core::{
    Config, JetPackageFormat, PackageConfig, PackageData, PackageDatabase, RepoConfig,
    REPO_CONFIG_PATH, REPO_DB_PATH,
};
use nanoid::nanoid;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs, path::Path};

mod download;

pub async fn install(packages: Vec<&String>) {
    let repo_dir = fs::read_dir(REPO_DB_PATH).unwrap();
    let mut repos: HashMap<String, PackageDatabase> = HashMap::new();
    let mut mb = MultiProgress::new();
    let repo_cfg = RepoConfig::load(REPO_CONFIG_PATH.into());

    for repo in repo_dir {
        let repo = repo.unwrap();

        if repo.file_type().unwrap().is_file() {
            let file_name = repo.file_name();
            let file_name = file_name.to_str().unwrap();

            let pdb = PackageDatabase::load(Path::new(REPO_DB_PATH).join(&file_name));
            repos.insert(file_name.to_string(), pdb);
        }
    }

    let mut in_repos = vec![];

    for package in packages {
        println!("get which repo package is in");
        for (name, repo) in &repos {
            if repo.packages.contains_key(package).clone() {
                in_repos.push(name)
            }
        }

        println!("get repo name");
        let repo_name = in_repos[0];
        let repo = repos.get(repo_name).unwrap();

        println!("get latest verion of package");
        let pkg = repo.packages.get(&package.clone()).unwrap();
        let version = pkg.get(pkg.len() - 1).unwrap().to_owned();

        println!("get latest repo of the package url");
        let mut repos = repo_cfg.repo.clone().into_iter();
        let url = repos
            .find(|r| r.name == repo_name.strip_suffix(".db").unwrap().clone())
            .unwrap()
            .url;

        println!("download package: {}:{} for {} from {}", package.to_string(), version, env::consts::ARCH, url);
        let pkg_path =
            download::download(&mut mb, url, package.to_string(), version, String::from(env::consts::ARCH)).await;

        println!("installing {pkg_path}");
        install_file(&pkg_path)
    }
}

pub fn install_file(path: &str) {
    let temp_dir = env::temp_dir().join("jetpkg");
    let package_temp_dir = temp_dir.join(nanoid!());

    if !temp_dir.exists() {
        fs::create_dir(temp_dir).unwrap();
    }

    fs::create_dir(&package_temp_dir).unwrap();

    JetPackageFormat::decompres_file(path, &package_temp_dir);

    env::set_current_dir(&package_temp_dir).unwrap();

    let cfg = PackageConfig::load(Path::new("./package.toml").to_path_buf());
    let mut jet_db = jet_core::Database::load();

    let mut files = vec![];

    let dependencies = match cfg.dependencies {
        Some(ref deps) => {
            let mut dependencies = vec![];
            for (pkg, ver) in deps {
                dependencies.push((pkg.clone(), ver.clone()))
            }
            dependencies
        }
        None => vec![],
    };

    let conflicts = match cfg.dependencies {
        Some(ref conflict) => {
            let mut conflicts = vec![];
            for (ref pkg, ver) in conflict {
                conflicts.push((pkg.clone().to_owned(), ver.clone()))
            }
            conflicts
        }
        None => vec![],
    };

    for (file, install_opts) in cfg.install {
        if install_opts.executable {
            let mut perms = fs::metadata(&file).unwrap().permissions();
            perms.set_mode(0o777);
            fs::set_permissions(&file, perms).unwrap();
        }

        let parant = Path::new(&install_opts.path).parent().unwrap();

        if !parant.exists() {
            fs::create_dir_all(parant).unwrap()
        }

        jet_core::fs_copy(file.clone(), install_opts.path.clone());
        jet_core::rs_remove(file);

        files.push(install_opts.path.clone())
    }

    jet_db.installed_packages.insert(
        cfg.name,
        PackageData {
            version: cfg.version,
            files,
            dependencies,
            conflicts,
        },
    );

    jet_db.save();

    fs::remove_dir_all(&package_temp_dir).unwrap();
}
