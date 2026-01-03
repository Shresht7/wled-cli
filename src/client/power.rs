use crate::{
    api::{
        endpoints::Endpoint,
        response::APIResponse,
        state::{PowerState, State},
    },
    error::{Result, WledError},
};

impl super::WLEDClient {
    /// Gets the current power status
    pub fn get_power(&self) -> Result<Option<PowerState>> {
        let url = Endpoint::State.url(&self.host);
        let response: State = self.client.get(url).send()?.json()?;
        Ok(response.on)
    }

    /// Sets the power status
    pub fn set_power(&self, power: PowerState) -> Result<()> {
        let url = Endpoint::State.url(&self.host);

        let payload = State {
            on: Some(power),
            ..Default::default()
        };

        let response: APIResponse = self.client.post(url).json(&payload).send()?.json()?;

        match response.success {
            true => Ok(()),
            false => Err(WledError::ApiError("Failed to set power state".to_string())),
        }
    }
}
