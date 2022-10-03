pub fn install(packages: Vec<&str>) {
    println!("Installing... ");
    for package in packages {
        println!("{}", package)
    }
}