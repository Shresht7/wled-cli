use crate::{
    api::{
        eff::{self, EffectsList},
        endpoints::Endpoint,
        response::APIResponse,
        state::State,
    },
    error::{Result, WledError},
};

impl super::WLEDClient {
    /// Get a list of all available effects
    pub fn list_effects(&self) -> Result<EffectsList> {
        let url = Endpoint::Eff.url(&self.host);
        let response = self.client.get(url).send()?.json()?;
        Ok(response)
    }

    /// Set the effects for the segments
    pub fn set_effects(&self, effects: &[String]) -> Result<()> {
        let segments = eff::parse_into_segments(effects)?;
        let url = Endpoint::State.url(&self.host);
        let payload = State {
            seg: Some(segments),
            ..Default::default()
        };
        let response: APIResponse = self.client.post(url).json(&payload).send()?.json()?;

        match response.success {
            true => Ok(()),
            false => Err(WledError::ApiError("Failed to set effects".to_string())),
        }
    }
}
