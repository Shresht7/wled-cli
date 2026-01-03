use crate::{
    api::{endpoints::Endpoint, response::APIResponse, state::State},
    error::{Result, WledError},
};

impl super::WLEDClient {
    pub fn get_state(&self) -> Result<State> {
        let url = Endpoint::State.url(&self.host);
        let response = self.client.get(url).send()?.json()?;
        Ok(response)
    }

    pub fn set_state(&self, state: State) -> Result<()> {
        let url = Endpoint::State.url(&self.host);
        let response: APIResponse = self.client.post(url).json(&state).send()?.json()?;
        match response.success {
            true => Ok(()),
            false => Err(WledError::ApiError("Failed to set state".to_string())),
        }
    }
}
