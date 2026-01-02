use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::context::Context;

/// Power on or off the WLED device
#[derive(Parser, Debug)]
#[clap(alias = "switch")]
pub(crate) struct Power {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Turn the device on
    On,
    /// Turn the device off
    Off,
    /// Toggle the device on or off
    Toggle,
}

impl Serialize for SubCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SubCommand::On => serializer.serialize_bool(true),
            SubCommand::Off => serializer.serialize_bool(false),
            SubCommand::Toggle => serializer.serialize_str("t"),
        }
    }
}

impl Power {
    pub(crate) fn execute(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}/json/state", ctx.host);

        let payload = json!({
            "on": &self.subcommand
        });

        let response = ctx.client.post(url).json(&payload).send()?;

        if !response.status().is_success() {
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        let json_response: APIResponse = response.json()?;

        if json_response.success {
            println!(
                "Successfully set power state for {} to {:?}",
                ctx.host, &self.subcommand
            );
        } else {
            println!("Failed to set power state for {}", ctx.host);
        }

        Ok(())
    }
}

#[derive(Deserialize)]
pub(crate) struct APIResponse {
    success: bool,
}
