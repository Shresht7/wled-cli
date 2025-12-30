use clap::{Parser, Subcommand};

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,

    #[arg(long)]
    pub(crate) host: Option<String>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    Power,
    Brightness,
    Preset,
    Effect,
    Status,
    Info,
    Device,
}

impl Command {
    pub(crate) fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{self:#?}");
        Ok(())
    }
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}
