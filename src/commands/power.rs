use clap::{Parser, Subcommand};

use crate::api::endpoints::Endpoint;
use crate::api::response;
use crate::api::state::{self, State};
use crate::context::Context;

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum PowerSubcommand {
    /// Turn the device on
    On,
    /// Turn the device off
    Off,
    /// Toggle the device on or off
    Toggle,
}

impl From<PowerSubcommand> for state::PowerState {
    fn from(value: PowerSubcommand) -> Self {
        match value {
            PowerSubcommand::On => state::PowerState::On,
            PowerSubcommand::Off => state::PowerState::Off,
            PowerSubcommand::Toggle => state::PowerState::Toggle,
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
    pub(crate) fn execute(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        match self.subcommand {
            Some(state) => self.set_power(state, ctx),
            None => self.get_power(ctx),
        }
    }

    fn set_power(
        &self,
        subcmd: PowerSubcommand,
        ctx: &Context,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = subcmd.into();
        ctx.client.set_power(state)?;
        Ok(())
    }

    fn get_power(&self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let power = ctx.client.get_power()?;
        if let Some(power) = power {
            println!("Power: {power}");
        }
        Ok(())
    }
}
