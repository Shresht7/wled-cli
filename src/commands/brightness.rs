use clap::Parser;
use serde_json::json;

use crate::context::Context;

#[derive(Parser, Debug)]
pub(crate) struct Brightness {
    /// Set the brightness to a specific value (0-255)
    /// If no value is provided, the get the brightness value of the WLED
    value: Option<u8>,
}

impl Brightness {
    pub(crate) fn execute(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        match self.value {
            Some(val) => self.set_brightness(val, ctx),
            None => self.get_brightness(ctx),
        }
    }

    /// Set the brightness level
    fn set_brightness(self, val: u8, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}/json/state", ctx.host);

        let payload = json!({ "bri": val });

        let response = ctx.client.post(url).json(&payload).send()?;

        if !response.status().is_success() {
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        println!("Brightness set to {}", val);

        Ok(())
    }

    /// Get the current brightness level
    fn get_brightness(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}/json/state", ctx.host);

        let response = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
        let brightness = &response["bri"];
        let power = &response["on"];

        // ? Probably should create a struct for the `State` object.

        println!("Brightness: {brightness}");
        println!("Power: {power}");

        Ok(())
    }
}
