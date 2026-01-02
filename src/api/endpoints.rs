/// API Endpoints
pub enum Endpoint {
    State,
    Info,
    SI,
    Nodes,
    Eff,
    Palx,
    FxData,
    Net,
    Live,
    Pal,
    Cfg,
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endpoint::State => write!(f, "json/state"),
            Endpoint::Info => write!(f, "json/info"),
            Endpoint::SI => write!(f, "json/si"),
            Endpoint::Nodes => write!(f, "json/nodes"),
            Endpoint::Eff => write!(f, "json/eff"),
            Endpoint::Palx => write!(f, "json/palx"),
            Endpoint::FxData => write!(f, "json/fxdata"),
            Endpoint::Net => write!(f, "json/net"),
            Endpoint::Live => write!(f, "json/live"),
            Endpoint::Pal => write!(f, "json/pal"),
            Endpoint::Cfg => write!(f, "json/cfg"),
        }
    }
}

impl Endpoint {
    pub fn url(&self, host: &str) -> String {
        let protocol = "http";
        format!("{protocol}://{host}/{self}")
    }
}
