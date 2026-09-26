use serde::{Deserialize, Serialize};

/// WhatsApp Business label record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelRecord {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub color: Option<u8>,
    #[serde(default, rename = "hexColor")]
    pub hex_color: Option<String>,
}

/// Request to create or update a label.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpsertLabelRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Color index (0-19).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
}

/// Request to add a label to a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddLabelRequest {
    #[serde(rename = "labelId")]
    pub label_id: String,
}

/// Chat associated with a label.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelChat {
    #[serde(rename = "chatId")]
    pub chat_id: String,
}
