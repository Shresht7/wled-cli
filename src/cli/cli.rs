use clap::{Parser, Subcommand};

use wled::error::Result;
use super::{commands, context::Context};

#[derive(Parser)]
#[clap(version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,

    #[arg(long, global = true)]
    pub(crate) host: Option<String>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    Power(commands::Power),
    Brightness(commands::Brightness),
    Preset,
    Effects(commands::Effects),
    Palettes(commands::Palettes),
    Status,
    Info,
    Device,
}

impl Command {
    pub(crate) fn execute(self, ctx: &Context) -> Result<()> {
        match self {
            Command::Power(cmd) => cmd.execute(ctx),
            Command::Brightness(cmd) => cmd.execute(ctx),
            Command::Preset => todo!(),
            Command::Effects(cmd) => cmd.execute(ctx),
            Command::Palettes(cmd) => cmd.execute(ctx),
            Command::Status => todo!(),
            Command::Info => todo!(),
            Command::Device => todo!(),
        }
    }
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}
