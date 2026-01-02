use clap::Parser;
use serde_json::json;

use crate::context::Context;

/// Power on or off the WLED device
#[derive(Parser, Debug)]
pub(crate) struct Power {
    #[arg(long)]
    on: bool,
    #[arg(long)]
    off: bool,
}

// ? Maybe a subcommand structure would be more ergonomic. For example, `power on`, `power off` and `power toggle`.
// ? simply `power` can then be used to query the state.

impl Power {
    pub(crate) fn execute(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}/json/state", ctx.host);
        let on = self.on && !self.off;
        let payload = json!({ "on": on });
        ctx.client.post(url).json(&payload).send()?;

        // ? Probably should abstract away the API logic in a separate module

        Ok(())
    }
}
