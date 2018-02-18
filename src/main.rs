extern crate time;

use std::env;
use std::path::Path;
use std::process::Command;

fn help() {
    println!("How to use Aurora");
}

fn struct_cmd(cmd: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(cmd);
    for arg in args {
        cmd.arg(arg);
    }
    cmd
}

fn download_tarball(package_name: &str) {
    let now = time::now();
    let now_formated = time::strftime("%Y-%m-%d-%H-%M-%S", &now).unwrap();
    let curl_args = [
        "curl",
        "--output",
        &format!("/tmp/aurora/{0}_{1}.tar.gz", package_name, &now_formated),
        "-L",
        "-O",
        &format!("https://aur.archlinux.org/cgit/aur.git/snapshot/{}.tar.gz",
                 package_name),
    ];
    let mut curl_cmds = match struct_cmd(curl_args[0], &curl_args[1..]).spawn() {
        Ok(p) => p,
        Err(e) => panic!("faild tot exectute: {}", e),
    };
    curl_cmds.wait();
}

fn makepkg(path: &str) {
    let pkg_path = Path::new(path);
    match env::set_current_dir(&pkg_path) {
        Ok(p) => println!("{}", pkg_path.display()),
        Err(e) => panic!("Could not change directory: {}", e),
    };

    let mut cmd = match Command::new("makepkg").arg("-si").spawn() {
        Ok(p) => p,
        Err(e) => panic!("Failed while Building: {}", e),
    };
    cmd.wait();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { help(); }
    for package in &args[1..] {
        download_tarball(package);
        // TODO: unzip tar.gz
        //  pgkbuild_path = unzip_targz(tg_path)
        // makepkg(pgkbuild_path);
    }
}
