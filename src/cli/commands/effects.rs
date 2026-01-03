use clap::{Parser, Subcommand};

use crate::cli::context::Context;
use wled::error::Result;

/// List all available effects
#[derive(Parser, Debug)]
#[clap(alias = "fx")]
pub(crate) struct Effects {
    #[clap(subcommand)]
    subcommands: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    List,
    Get,
    Set { effects: Vec<String> },
}

impl Effects {
    pub(crate) fn execute(&self, ctx: &Context) -> Result<()> {
        match &self.subcommands {
            Some(Subcommands::List) => self.list_effects(ctx),
            Some(Subcommands::Get) => self.get_effects(ctx),
            Some(Subcommands::Set { effects }) => self.set_effects(effects, ctx),
            None => self.get_effects(ctx),
        }
    }

    /// Get a list of all available effects
    fn list_effects(&self, ctx: &Context) -> Result<()> {
        let effects = ctx.client.list_effects()?;
        for (i, fx) in effects.iter().enumerate() {
            println!("{i:>3} {fx}");
        }
        Ok(())
    }

    fn get_effects(&self, ctx: &Context) -> Result<()> {
        let state = ctx.client.get_state()?;
        if let Some(segments) = state.seg {
            for (idx, segment) in segments.iter().enumerate() {
                if let Some(fx) = &segment.fx {
                    println!("Segment[{}]: {}", segment.id.unwrap_or(idx as u8), fx);
                }
            }
        }
        Ok(())
    }

    fn set_effects(&self, effects: &[String], ctx: &Context) -> Result<()> {
        ctx.client.set_effects(effects)?;
        println!("Effects set.");
        Ok(())
    }
}
