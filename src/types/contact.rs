use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contact information known to a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactRecord {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, rename = "pushName", skip_serializing_if = "Option::is_none")]
    pub push_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, rename = "isBusiness")]
    pub is_business: bool,
    #[serde(default, rename = "isGroup")]
    pub is_group: bool,
    #[serde(default, rename = "isBlocked")]
    pub is_blocked: bool,
}

/// Query parameters for listing contacts.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListContactsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Response checking if a phone number is registered on WhatsApp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckNumberResponse {
    pub exists: bool,
    #[serde(
        default,
        rename = "whatsappId",
        skip_serializing_if = "Option::is_none"
    )]
    pub whatsapp_id: Option<String>,
}

/// Single profile picture response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePictureResponse {
    #[serde(default, rename = "profilePictureUrl")]
    pub profile_picture_url: Option<String>,
}

/// Batch profile pictures response for up to 50 contacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePicturesResponse {
    #[serde(default)]
    pub pictures: HashMap<String, Option<String>>,
}

/// Response resolving a contact privacy ID (@lid) to an MSISDN phone number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPhoneResponse {
    pub phone: Option<String>,
}

/// Request to create or update a contact in the account address book.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertContactRequest {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(default, rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl UpsertContactRequest {
    pub fn new(first_name: impl Into<String>) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: None,
        }
    }
}
