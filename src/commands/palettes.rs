use clap::Parser;
use serde::Deserialize;

use crate::context::Context;

#[derive(Debug, Parser, Deserialize)]
pub(crate) struct Palettes {}

impl Palettes {
    pub(crate) fn execute(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let palettes = ctx.client.get_palettes()?;
        for (i, palette) in palettes.iter().enumerate() {
            println!("{i:>3} {palette}");
        }
        Ok(())
    }
}
