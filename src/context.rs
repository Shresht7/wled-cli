use reqwest::blocking::Client;

pub(crate) struct Context {
    pub(crate) host: String,
    pub(crate) client: Client,
}

impl Context {
    pub(crate) fn new(host: String) -> Self {
        Context {
            host,
            client: reqwest::blocking::Client::new(),
        }
    }
}
