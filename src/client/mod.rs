use reqwest::blocking::Client;

use crate::api::{endpoints::Endpoint, state::State};

mod effects;
mod palettes;
mod power;
mod state;

#[derive(Debug, Default)]
pub struct WLEDClient {
    /// The IP address of the WLED device
    pub(crate) host: String,
    /// HTTP Client that is used to communicate with the WLED device
    pub(crate) client: Client,
}

impl WLEDClient {
    pub fn new(host: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .expect("failed to build http client");
        Self {
            host,
            client,
            ..Default::default()
        }
    }

    pub fn get_state(&self) -> Result<State, Box<dyn std::error::Error>> {
        let url = Endpoint::State.url(&self.host);
        let response = self.client.get(url).send()?.json()?;
        Ok(response)
    }
}
