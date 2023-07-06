use std::path::Path;

use jet_core::{Config, JetPackageFormat, PackageArch, PackageConfig};

pub fn bundle(dir: &str, arch: &str) {
    let pkg_config: PackageConfig = PackageConfig::load(Path::new(dir).join("package.toml"));
    is_arch_supported(&pkg_config, arch.to_string());
    let output_file = create_package_name(pkg_config.name, pkg_config.version, arch.to_string());
    create_and_compress_jetpack(dir.to_string(), output_file)
}

fn is_arch_supported(cfg: &PackageConfig, arch: String) {
    let arch_supported = cfg.arch.contains(&PackageArch::from(arch.clone()));
    if arch_supported == false {
        panic!("This arch {arch} is not supported by this package")
    }
}

fn create_package_name(name: String, version: String, arch: String) -> String {
    format!("{}-{}-{}", name, version, arch)
}

fn create_and_compress_jetpack(input_dir: String, output_file: String) {
    let package = JetPackageFormat::new_package((input_dir.to_string(), output_file));
    package.compress();
}
