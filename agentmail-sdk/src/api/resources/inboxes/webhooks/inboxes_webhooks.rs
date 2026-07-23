use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient2 {
    pub http_client: HttpClient,
}

impl WebhooksClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes:webhooks list --inbox-id <inbox_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        inbox_id: &InboxesInboxId,
        request: &InboxesWebhooksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksListWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/webhooks", inbox_id.0),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .serialize("ascending", request.ascending.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a webhook scoped to this inbox.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes:webhooks create --inbox-id <inbox_id> --url https://example.com/webhook --event-type message.received
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        inbox_id: &InboxesInboxId,
        request: &WebhooksCreateInboxWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/webhooks", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes:webhooks get --inbox-id <inbox_id> --webhook-id <webhook_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/webhooks/{}", inbox_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes:webhooks delete --inbox-id <inbox_id> --webhook-id <webhook_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn delete(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/inboxes/{}/webhooks/{}", inbox_id.0, webhook_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes:webhooks update --inbox-id <inbox_id> --webhook-id <webhook_id> --event-type message.received
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        request: &WebhooksUpdateInboxWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/inboxes/{}/webhooks/{}", inbox_id.0, webhook_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
