use clap::{Parser, Subcommand};

use crate::cli::context::Context;
use wled::{api::state::PowerState, error::Result};

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum PowerSubcommand {
    /// Turn the device on
    On,
    /// Turn the device off
    Off,
    /// Toggle the device on or off
    Toggle,
}

impl From<PowerSubcommand> for PowerState {
    fn from(value: PowerSubcommand) -> Self {
        match value {
            PowerSubcommand::On => PowerState::On,
            PowerSubcommand::Off => PowerState::Off,
            PowerSubcommand::Toggle => PowerState::Toggle,
        }
    }
}

/// Power on or off the WLED device
#[derive(Parser, Debug)]
#[clap(alias = "switch")]
pub(crate) struct Power {
    #[clap(subcommand)]
    subcommand: Option<PowerSubcommand>,
}

impl Power {
    pub(crate) fn execute(self, ctx: &Context) -> Result<()> {
        match self.subcommand {
            Some(state) => self.set_power(state, ctx),
            None => self.get_power(ctx),
        }
    }

    fn set_power(&self, subcmd: PowerSubcommand, ctx: &Context) -> Result<()> {
        let state = subcmd.into();
        ctx.client.set_power(state)?;
        println!("Power set");
        Ok(())
    }

    fn get_power(&self, ctx: &Context) -> Result<()> {
        let power = ctx.client.get_power()?;
        if let Some(power) = power {
            println!("Power: {power}");
        }
        Ok(())
    }
}
