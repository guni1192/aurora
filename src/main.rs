extern crate time;

use std::env;
use std::path::Path;
use std::process::{Command, exit};

fn help() {
    println!("How to use Aurora");
}

// TODO: cURL(外部コマンド)無しで実装する。
fn download_tarball(package_name: &str) {
    /*
    let now = time::now();
    let now_formated = time::strftime("%Y-%m-%d-%H-%M-%S", &now).unwrap();
    */
    let mut curl_cmds = match Command::new("curl")
        .arg("--output")
        .arg(&format!("/tmp/aurora/{}.tar.gz", package_name))
        .arg("-L")
        .arg("-O")
        .arg(&format!("https://aur.archlinux.org/cgit/aur.git/snapshot/{}.tar.gz",
                 package_name))
        .spawn() {
            Ok(p) => p,
            Err(e) => panic!("faild tot exectute: {}", e),
        };
    curl_cmds.wait();
}

fn unzip_targz(in_path: &str) {
    let pkg_path = Path::new("/tmp/aurora/");
    match env::set_current_dir(&pkg_path) {
        Ok(p) => println!("{}", pkg_path.display()),
        Err(e) => panic!("Could not change directory: {}", e),
    };

    let mut tar_cmd = match Command::new("tar")
        .arg("zxvf")
        .arg(in_path)
        .spawn() {
            Ok(p) => println!("Successful unzip tar.gz file: {:?}", p),
            Err(e) => panic!("Failed unzip tar.gz: {}", e),
        };
    println!("{:?}", tar_cmd);
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
    if args.len() < 2 { help(); exit(0); }

    match args[1].as_str() {
        "-S" => println!("Install package"),
        _    => println!("Unexpected arguments"),
    }

    for package in &args[2..] {
        download_tarball(package);
        unzip_targz(&format!("/tmp/aurora/{}.tar.gz", package.to_string()));
        makepkg(&format!("/tmp/aurora/{}", package.to_string()));
    }
}
