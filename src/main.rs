pub mod cli;

// The main entrypoint of the application
fn main() {
    if let Err(err) = cli::run() {
        eprintln!("{err}");
        std::process::exit(1)
    }
}
