use reqwest::blocking::Client;

/// The application context
pub(crate) struct Context {
    /// The IP address of the WLED device
    pub(crate) host: String,
    /// HTTP Client that is used to communicate with the WLED device
    pub(crate) client: Client,
}

impl Context {
    /// Create a new application context
    pub(crate) fn new(host: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .expect("failed to build http client");
        Context { host, client }
    }
}
