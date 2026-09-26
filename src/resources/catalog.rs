use crate::error::OpenWAError;
use crate::transport::{encode_path_segment, Transport};
use crate::types::*;
use reqwest::Method;
use std::sync::Arc;

/// WhatsApp Business Catalog resource.
#[derive(Clone)]
pub struct CatalogResource {
    pub(crate) transport: Arc<Transport>,
}

impl CatalogResource {
    pub fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// Get catalog overview for a session.
    pub async fn get_catalog(&self, session_id: &str) -> Result<CatalogInfo, OpenWAError> {
        let path = format!("/api/sessions/{}/catalog", encode_path_segment(session_id));
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// List products in catalog with pagination.
    pub async fn list_products(
        &self,
        session_id: &str,
        query: Option<ListProductsQuery>,
    ) -> Result<PaginatedProducts, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/catalog/products",
            encode_path_segment(session_id)
        );
        let mut query_params: Vec<(&str, String)> = Vec::new();
        if let Some(ref q) = query {
            if let Some(limit) = q.limit {
                query_params.push(("limit", limit.to_string()));
            }
            if let Some(page) = q.page {
                query_params.push(("page", page.to_string()));
            }
        }
        let q_refs: Vec<(&str, &str)> =
            query_params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let q_opt = if q_refs.is_empty() {
            None
        } else {
            Some(q_refs.as_slice())
        };

        self.transport
            .execute(Method::GET, &path, q_opt, None)
            .await
    }

    /// Get details of a single product.
    pub async fn get_product(
        &self,
        session_id: &str,
        product_id: &str,
    ) -> Result<CatalogProduct, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/catalog/products/{}",
            encode_path_segment(session_id),
            encode_path_segment(product_id)
        );
        self.transport.execute(Method::GET, &path, None, None).await
    }

    /// Send a product message to a chat.
    pub async fn send_product(
        &self,
        session_id: &str,
        req: SendProductRequest,
    ) -> Result<ProductMessageResponse, OpenWAError> {
        let path = format!(
            "/api/sessions/{}/messages/send-product",
            encode_path_segment(session_id)
        );
        let body = serde_json::to_value(req)?;
        self.transport
            .execute(Method::POST, &path, None, Some(body))
            .await
    }
}
