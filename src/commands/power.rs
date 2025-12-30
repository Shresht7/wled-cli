use clap::Parser;

use crate::context::Context;

#[derive(Parser, Debug)]
pub(crate) struct Power {
    #[arg(short, long, default_value_t = true)]
    on: bool,
}

impl Power {
    pub(crate) fn execute(self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let state = match self.on {
            true => "ON",
            false => "OFF",
        };
        println!("Powering: {state} for {}", ctx.host);
        Ok(())
    }
}
