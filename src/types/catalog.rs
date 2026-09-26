use serde::{Deserialize, Serialize};

/// Catalog information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Catalog product entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogProduct {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(
        default,
        rename = "currencyCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency_code: Option<String>,
    #[serde(default, rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Paginated products response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedProducts {
    pub products: Vec<CatalogProduct>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Value>,
}

/// Response returned from sending a product message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductMessageResponse {
    #[serde(rename = "messageId")]
    pub message_id: String,
}

/// Request to send a catalog product to a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendProductRequest {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing)]
    pub footer: Option<String>,
}

impl SendProductRequest {
    pub fn new(chat_id: impl Into<String>, product_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            product_id: product_id.into(),
            body: None,
            footer: None,
        }
    }
}

/// Query parameters for listing catalog products.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListProductsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
