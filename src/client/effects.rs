use crate::api::{eff::EffectsList, endpoints::Endpoint};

impl super::WLEDClient {
    /// Get a list of all available effects
    pub fn list_effects(&self) -> Result<EffectsList, Box<dyn std::error::Error>> {
        let url = Endpoint::Eff.url(&self.host);
        let response = self.client.get(url).send()?.json()?;
        Ok(response)
    }
}
