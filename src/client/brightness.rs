use crate::api::{endpoints::Endpoint, state::State};

impl super::WLEDClient {
    /// Get the current brightness level
    pub fn get_brightness(&self) -> Result<Option<u8>, Box<dyn std::error::Error>> {
        let url = Endpoint::State.url(&self.host);
        let response: State = self.client.get(url).send()?.json()?;
        Ok(response.bri)
    }

    /// Set the brightness level
    pub fn set_brightness(&self, brightness: u8) -> Result<(), Box<dyn std::error::Error>> {
        let url = Endpoint::State.url(&self.host);
        let payload = State {
            bri: Some(brightness),
            ..Default::default()
        };
        let response = self.client.post(url).json(&payload).send()?;
        Ok(())
    }
}
