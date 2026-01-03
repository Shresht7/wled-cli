use crate::{
    api::state::{Fx, Segment},
    error::Result,
};

pub type EffectsList = Vec<String>;

pub fn parse_into_segments(effects: &[String]) -> Result<Vec<Segment>> {
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
