use crate::api::{
    endpoints::Endpoint,
    response::APIResponse,
    state::{PowerState, State},
};

impl super::WLEDClient {
    /// Gets the current power status
    pub fn get_power(&self) -> Result<Option<PowerState>, Box<dyn std::error::Error>> {
        let url = Endpoint::State.url(&self.host);
        let response: State = self.client.get(url).send()?.json()?;
        Ok(response.on)
    }

    /// Sets the power status
    pub fn set_power(&self, power: PowerState) -> Result<(), Box<dyn std::error::Error>> {
        let url = Endpoint::State.url(&self.host);

        let payload = State {
            on: Some(power),
            ..Default::default()
        };

        let response: APIResponse = self.client.post(url).json(&payload).send()?.json()?;

        if response.success {
            Ok(())
        } else {
            // FIXME: Create APIError?
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to set power state",
            )))
        }
    }
}
