use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: bool,
}

#[derive(Serialize)]
pub struct VersionResponse {
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct PowerRequest {
    pub timeout: Option<u16>,
}

#[derive(Serialize)]
pub struct PowerResponse {
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Default for PowerResponse {
    fn default() -> Self {
        PowerResponse {
            status: true,
            error: None,
        }
    }
}
