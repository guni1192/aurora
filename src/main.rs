use clap::{App, Arg};

mod sync;
// mod query;

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
        println!("Install {} ...", s);
        sync::sync(s);
    }
}
