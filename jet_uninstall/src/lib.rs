use std::fs;

pub fn uninstall(package: &str) {
    let mut jet_db = jet_core::Database::load();
    let package = jet_db.installed_packages.remove(package).unwrap();
    // check if removing this causes dependicies issues

    for file in package.files {
        fs::remove_file(file).unwrap();
    }

    jet_db.save();
}
