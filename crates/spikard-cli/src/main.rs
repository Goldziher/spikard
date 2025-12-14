fn main() {
    if let Err(err) = spikard_cli::cli::run_from_env() {
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}
