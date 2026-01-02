use clap::{Parser, Subcommand};
use serde_json::json;

use crate::api::eff::EffectsList;
use crate::api::state::{Fx, Segment, State};
use crate::context::Context;

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
    pub(crate) fn execute(&self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        match &self.subcommands {
            Some(Subcommands::List) => self.list_effects(ctx),
            Some(Subcommands::Get) => self.get_effects(ctx),
            Some(Subcommands::Set { effects }) => self.set_effects(effects, ctx),
            None => self.get_effects(ctx),
        }
    }

    /// Get a list of all available effects
    fn list_effects(&self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
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

    fn get_effects(&self, ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://{}/json/state", ctx.host);

        let response = ctx.client.get(url).send()?;

        if !response.status().is_success() {
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        let state: State = response.json()?;

        if let Some(segments) = state.seg {
            for (idx, segment) in segments.iter().enumerate() {
                if let Some(fx) = &segment.fx {
                    println!("Segment[{}]: {}", segment.id.unwrap_or(idx as u8), fx);
                }
            }
        }

        Ok(())
    }

    fn set_effects(
        &self,
        effects: &[String],
        ctx: &Context,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let segments = Self::parse_into_segments(effects)?;

        let url = format!("http://{}/json/state", ctx.host);

        let payload = json!({
            "seg": segments
        });

        let response = ctx.client.put(url).json(&payload).send()?;

        if !response.status().is_success() {
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        for (idx, segment) in segments.iter().enumerate() {
            if let Some(fx) = &segment.fx {
                println!("Segment[{}]: {}", segment.id.unwrap_or(idx as u8), fx);
            }
        }

        Ok(())
    }

    fn parse_into_segments(effects: &[String]) -> Result<Vec<Segment>, Box<dyn std::error::Error>> {
        let mut segments: Vec<Segment> = Vec::new();
        for (idx, fx) in effects.iter().enumerate() {
            let segment = if fx.contains(":") {
                let parts = fx.split(":").collect::<Vec<&str>>();
                let id = parts[0].parse::<u8>()?;
                let fx = parts[1].parse::<Fx>()?;
                Segment::new().id(id).fx(fx)
            } else {
                Segment::new().id(idx as u8).fx(fx.parse()?)
            };
            segments.push(segment);
        }
        Ok(segments)
    }
}

// !! Holy hell this needs a refactor. Need to wrap all API client logic up in a Client struct. Decouple api logic from command logic. etc.
