use reqwest::blocking::Client;

pub(crate) struct Context {
    pub(crate) host: String,
    pub(crate) client: Client,
}

impl Context {
    pub(crate) fn new(host: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .expect("failed to build http client");
        Context { host, client }
    }
}
