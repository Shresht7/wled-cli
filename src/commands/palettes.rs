use clap::Parser;
use serde::Deserialize;

use crate::{
    api::{endpoints::Endpoint, pal::PaletteList},
    context::Context,
};

#[derive(Debug, Parser, Deserialize)]
pub(crate) struct Palettes {}

impl Palettes {
    pub(crate) fn execute(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = Endpoint::Pal.url(&ctx.host);

        let response = ctx.client.get(url).send()?;

        if !response.status().is_success() {
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        let palettes: PaletteList = response.json()?;

        for (i, palette) in palettes.iter().enumerate() {
            println!("{i:>3} {palette}");
        }

        Ok(())
    }
}
