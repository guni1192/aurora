mod sync;
// mod query;
mod cli;

fn main() {
    let matches = cli::init_config().get_matches();

    if let Some(s) = matches.values_of("sync") {
        sync::sync(s.collect());
    }
}
