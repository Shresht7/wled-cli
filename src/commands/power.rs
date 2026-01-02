use clap::Parser;
use serde_json::json;

use crate::api::{response, state};
use crate::context::Context;

/// Power on or off the WLED device
#[derive(Parser, Debug)]
#[clap(alias = "switch")]
pub(crate) struct Power {
    #[clap(subcommand)]
    subcommand: state::PowerState,
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

        let json_response: response::APIResponse = response.json()?;

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
