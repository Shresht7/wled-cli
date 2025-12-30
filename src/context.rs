pub(crate) struct Context {
    pub(crate) host: String,
}

impl Context {
    pub(crate) fn new(host: String) -> Self {
        Context { host }
    }
}
