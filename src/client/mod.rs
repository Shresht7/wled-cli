use reqwest::blocking::Client;

mod palettes;
mod power;

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
        Self { host, client }
    }
}
