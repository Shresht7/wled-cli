use serde::Deserialize;

#[derive(Deserialize)]
pub struct APIResponse {
    pub success: bool,
}
