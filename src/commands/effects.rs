use clap::Parser;

use crate::api::eff::EffectsList;
use crate::context::Context;

/// List all available effects
#[derive(Parser, Debug)]
#[clap(alias = "fx")]
pub(crate) struct Effects {}

impl Effects {
    pub(crate) fn execute(&self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}/json/eff", ctx.host);

        let response = ctx.client.get(url).send()?;

        if !response.status().is_success() {
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        let effects: EffectsList = response.json()?;

        for (i, fx) in effects.iter().enumerate() {
            println!("{i:>3} {fx}");
        }

        Ok(())
    }
}
