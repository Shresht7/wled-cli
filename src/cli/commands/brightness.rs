use clap::Parser;

use crate::cli::context::Context;
use wled::error::Result;

#[derive(Parser, Debug)]
pub(crate) struct Brightness {
    /// Set the brightness to a specific value (0-255)
    /// If no value is provided, the get the brightness value of the WLED
    value: Option<u8>,
}

impl Brightness {
    pub(crate) fn execute(self, ctx: &Context) -> Result<()> {
        match self.value {
            Some(val) => self.set_brightness(val, ctx),
            None => self.get_brightness(ctx),
        }
    }

    /// Set the brightness level
    fn set_brightness(self, val: u8, ctx: &Context) -> Result<()> {
        ctx.client.set_brightness(val)?;
        println!("Brightness set to {}", val);
        Ok(())
    }

    /// Get the current brightness level
    fn get_brightness(self, ctx: &Context) -> Result<()> {
        let state = ctx.client.get_state()?;
        if let Some(bri) = state.bri {
            println!("Brightness: {bri}");
        }
        if let Some(on) = state.on {
            println!("Power: {on}");
        }
        Ok(())
    }
}
