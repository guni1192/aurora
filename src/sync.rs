use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

use crate::config;

fn download_tarball(package_name: &str) -> io::Result<()> {
    let aur_url = &format!("{}/{}.git", config::AUR_GIT_URL, package_name);
    let clone_path = &format!("{}/{}", config::DOWNLOAD_DIR, package_name);
    let mut git_clone = Command::new("git")
        .arg("clone")
        .arg(aur_url)
        .arg(clone_path)
        .spawn()?;

    git_clone.wait()?;
    Ok(())
}

fn makepkg(package_name: &str) -> io::Result<()> {
    let path = &format!("{}/{}", config::DOWNLOAD_DIR, package_name.to_string());
    let pkg_path = Path::new(path);

    env::set_current_dir(&pkg_path).expect("Could not change directory: ");

    let mut cmd = Command::new("makepkg").arg("-si").spawn()?;
    cmd.wait()?;

    Ok(())
}

pub fn sync(package_names: Vec<&str>) {
    for package_name in package_names {
        println!("Install {:?} ...", package_name);
        download_tarball(package_name).expect("Could not download aur tar: ");
        makepkg(package_name).expect("Could not makepkg: ");
    }
}

// fn search(options: &[String]) {
//     query::get_query(options[0])
// }
