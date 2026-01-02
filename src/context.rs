use crate::client::WLEDClient;

/// The application context
pub(crate) struct Context {
    /// The WLED API Client
    pub(crate) client: WLEDClient,
}

impl Context {
    /// Create a new application context
    pub(crate) fn new(host: String) -> Self {
        let client = WLEDClient::new(host);
        Context { client }
    }
}
