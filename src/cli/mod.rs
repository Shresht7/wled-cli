pub mod cli;
pub mod commands;
pub mod context;

// The main run logic
pub fn run() -> wled::error::Result<()> {
    let args = cli::parse();
    let host = args.host.expect("a wled ip address");
    let ctx = context::Context::new(host)?;
    args.command.execute(&ctx)
}
