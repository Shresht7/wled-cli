use crate::{client::WLEDClient, error::Result};

/// The application context
pub(crate) struct Context {
    /// The WLED API Client
    pub(crate) client: WLEDClient,
}

impl Context {
    /// Create a new application context
    pub(crate) fn new(host: String) -> Result<Self> {
        let client = WLEDClient::new(host)?;
        Ok(Context { client })
    }
}
