use std::env;
use std::path::Path;
use std::process::Command;
use std::process::exit;
mod query;


fn help() {
    println!("How to use Aurora");
}

fn download_tarball(package_name: &str) {
    let aur_url = &format!("https://aur.archlinux.org/{}.git", package_name);
    let clone_path = &format!("/tmp/aurora/{}", package_name);
    let mut git_clone = Command::new("git")
        .arg("clone").arg(aur_url).arg(clone_path)
        .spawn()
        .unwrap();
    git_clone.wait();
}

fn makepkg(path: &str) {
    let pkg_path = Path::new(path);
    match env::set_current_dir(&pkg_path) {
        Ok(p) => p,
        Err(e) => panic!("Could not change directory: {}", e),
    };

    let mut cmd = match Command::new("makepkg").arg("-si").spawn() {
        Ok(p) => p,
        Err(e) => panic!("Failed while Building: {}", e),
    };
    cmd.wait();
}

fn sync(options: &[String]) {
    for package in options {
        download_tarball(package);
        makepkg(&format!("/tmp/aurora/{}", package.to_string()));
    }
}

fn search(options: &[String]) {

    query::get_query(options[0])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { help(); exit(0); }

    match &args[1][..] {
        "-S" => sync(&args[2..]),
        "-Ss" => search(&args[2..]),
        _ => help(),
    }
}
