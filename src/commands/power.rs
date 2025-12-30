use clap::Parser;
use serde_json::json;

use crate::context::Context;

#[derive(Parser, Debug)]
pub(crate) struct Power {
    #[arg(long)]
    on: bool,
    #[arg(long)]
    off: bool,
}

impl Power {
    pub(crate) fn execute(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}/json/state", ctx.host);
        let on = self.on && !self.off;
        let payload = json!({ "on": on });
        ctx.client.post(url).json(&payload).send()?;
        Ok(())
    }
}
