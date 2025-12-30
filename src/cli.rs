use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,

    #[arg(long)]
    pub(crate) host: Option<String>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    Power(commands::Power),
    Brightness,
    Preset,
    Effect,
    Status,
    Info,
    Device,
}

impl Command {
    pub(crate) fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Power(cmd) => cmd.execute(),
            Command::Brightness => todo!(),
            Command::Preset => todo!(),
            Command::Effect => todo!(),
            Command::Status => todo!(),
            Command::Info => todo!(),
            Command::Device => todo!(),
        }
    }
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}
