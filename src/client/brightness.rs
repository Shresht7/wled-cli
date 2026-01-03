use crate::{
    api::{endpoints::Endpoint, response::APIResponse, state::State},
    error::{Result, WledError},
};

impl super::WLEDClient {
    /// Get the current brightness level
    pub fn get_brightness(&self) -> Result<Option<u8>> {
        let url = Endpoint::State.url(&self.host);
        let response: State = self.client.get(url).send()?.json()?;
        Ok(response.bri)
    }

    /// Set the brightness level
    pub fn set_brightness(&self, brightness: u8) -> Result<()> {
        let url = Endpoint::State.url(&self.host);
        let payload = State {
            bri: Some(brightness),
            ..Default::default()
        };
        let response: APIResponse = self.client.post(url).json(&payload).send()?.json()?;
        match response.success {
            true => Ok(()),
            false => Err(WledError::ApiError("Failed to set brightness".to_string())),
        }
    }
}
