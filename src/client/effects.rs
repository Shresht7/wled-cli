use serde_json::json;

use crate::api::{
    eff::{self, EffectsList},
    endpoints::Endpoint,
    state::Segment,
};

impl super::WLEDClient {
    /// Get a list of all available effects
    pub fn list_effects(&self) -> Result<EffectsList, Box<dyn std::error::Error>> {
        let url = Endpoint::Eff.url(&self.host);
        let response = self.client.get(url).send()?.json()?;
        Ok(response)
    }

    /// Set the effects for the segments
    pub fn set_effects(
        &self,
        effects: &[String],
    ) -> Result<Vec<Segment>, Box<dyn std::error::Error>> {
        let segments = eff::parse_into_segments(effects)?;
        let url = Endpoint::State.url(&self.host);
        let payload = json!({
            "seg": segments
        });
        let response = self.client.put(url).json(&payload).send()?;
        Ok(segments)
    }
}
