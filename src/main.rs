use std::env;
use std::path::Path;
use std::process::exit;
use std::process::Command;
// mod query;

use clap::{App, Arg, SubCommand};

fn download_tarball(package_name: &str) {
    let aur_url = &format!("https://aur.archlinux.org/{}.git", package_name);
    let clone_path = &format!("/tmp/aurora/{}", package_name);
    let mut git_clone = Command::new("git")
        .arg("clone")
        .arg(aur_url)
        .arg(clone_path)
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

// fn sync(options: &[String]) {
fn sync(s: &str) {
    // for package in options {
    download_tarball(s);
    makepkg(&format!("/tmp/aurora/{}", s.to_string()));
    // }
}

// fn search(options: &[String]) {
//     query::get_query(options[0])
// }

fn main() {
    let matches = App::new("aurora")
        .version("0.1.0")
        .author("Takashi IIGUNI <guni-lab@gmail.com>")
        .about("AUR Helper By Rust")
        .arg(
            Arg::with_name("sync")
                .short("S")
                .long("sync")
                .value_name("FORMULA...")
                .help("set repository name")
                .takes_value(true),
        )
        .get_matches();

    if let Some(s) = matches.value_of("sync") {
        println!("Install {}", s);
        sync(s)
    }

    // match &args[1][..] {
    //     "-S" => sync(&args[2..]),
    //     // "-Ss" => search(&args[2..]),
    //     _ => help(),
    // }
}
