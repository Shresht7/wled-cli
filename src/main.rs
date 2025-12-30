mod cli;
mod commands;
mod context;

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
    let ctx = context::Context::new(args.host.expect("a wled ip address"));
    args.command.execute(&ctx)
}
