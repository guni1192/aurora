use std::env;
use std::path::Path;
use std::process::Command;

fn download_tarball(package_name: &str) -> std::io::Result<()> {
    let aur_url = &format!("https://aur.archlinux.org/{}.git", package_name);
    let clone_path = &format!("/tmp/aurora/{}", package_name);
    let mut git_clone = Command::new("git")
        .arg("clone")
        .arg(aur_url)
        .arg(clone_path)
        .spawn()?;

    git_clone.wait()?;
    Ok(())
}

fn makepkg(package_name: &str) -> std::io::Result<()> {
    let path = &format!("/tmp/aurora/{}", package_name.to_string());
    let pkg_path = Path::new(path);

    env::set_current_dir(&pkg_path).expect("Could not change directory: ");

    let mut cmd = Command::new("makepkg").arg("-si").spawn()?;
    cmd.wait()?;

    Ok(())
}

pub fn sync(package_name: &str) {
    download_tarball(package_name).expect("Could not download aur tar: ");
    makepkg(package_name).expect("Could not makepkg: ");
}

// fn search(options: &[String]) {
//     query::get_query(options[0])
// }
