use reqwest::blocking::Client;

use crate::error::Result;

mod brightness;
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
    pub fn new(host: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()?;
        Ok(Self { host, client })
    }
}
