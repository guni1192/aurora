use std::env;
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
    let curl_args = [
        "curl",
        "--output",
        &format!("/tmp/aurora/{}.tar.gz", package_name),
        "-L",
        "-O",
        &format!("https://aur.archlinux.org/cgit/aur.git/snapshot/{}.tar.gz",
                 package_name),
    ];
    let mut curl_cmds = struct_cmd(curl_args[0], &curl_args[1..]);
    match curl_cmds.spawn() {
        Ok(p) => p,
        Err(e) => panic!("faild tot exectute: {}", e),
    };
}


fn main() {
    let packages: Vec<String> = env::args().collect();
    if packages.len() < 2 { help(); }
    for package in &packages[1..] {
        download_tarball(package);
    }
}
