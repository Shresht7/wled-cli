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

    /// Sets the power status
    fn set_power(
        &self,
        subcmd: PowerSubcommand,
        ctx: &Context,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = Endpoint::State.url(&ctx.host);

        let power_state: state::PowerState = subcmd.into();
        let payload = State {
            on: Some(power_state),
            ..Default::default()
        };

        let response = ctx.client.post(url).json(&payload).send()?;

        if !response.status().is_success() {
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        let json_response: response::APIResponse = response.json()?;

        if json_response.success {
            println!("Successfully set power state for {}", ctx.host);
        } else {
            println!("Failed to set power state for {}", ctx.host);
        }

        Ok(())
    }

    /// Gets the current power status
    fn get_power(&self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = Endpoint::State.url(&ctx.host);

        let response: State = ctx.client.get(url).send()?.json()?;

        if let Some(power) = response.on {
            println!("Power: {power:#?}");
        }

        Ok(())
    }
}
