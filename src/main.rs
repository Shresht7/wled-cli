mod cli;

// The main entrypoint of the application
fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1)
    }
}

// The main run logic
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse();
    args.command.execute()
}
