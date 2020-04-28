use clap::{crate_authors, crate_name, crate_version, App, Arg};

pub fn init_config<'a>() -> App<'a, 'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("AUR Helper")
        .arg(
            Arg::with_name("sync")
                .short("S")
                .long("sync")
                .value_name("FORMULA...")
                .help("set repository name")
                .multiple(true)
                .takes_value(true),
        )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sync_single_package() {
        let args = vec!["aurora", "-S", "linux-git"];
        init_config()
            .get_matches_from_safe(&args)
            .expect("validation faild:");
    }

    #[test]
    fn sync_multi_package() {
        let args = vec!["aurora", "-S", "linux-git", "vim-git"];
        init_config()
            .get_matches_from_safe(&args)
            .expect("validation faild:");
    }
}
