use serde::{Deserialize, Serialize};

/// Request to set account display name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetProfileNameRequest {
    pub name: String,
}

/// Request to set account about / status text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetProfileStatusRequest {
    pub status: String,
}

/// Request to set account profile picture.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetProfilePictureRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}
