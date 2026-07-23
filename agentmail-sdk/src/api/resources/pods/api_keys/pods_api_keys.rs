use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient3 {
    pub http_client: HttpClient,
}

impl ApiKeysClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:api-keys list --pod-id <pod_id>
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
        pod_id: &PodsPodId,
        request: &PodsApiKeysListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListApiKeysResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/api-keys", pod_id.0),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:api-keys create --pod-id <pod_id> --name "My Key"
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
        pod_id: &PodsPodId,
        request: &CreateApiKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateApiKeyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/pods/{}/api-keys", pod_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:api-keys delete --pod-id <pod_id> --api-key-id <api_key_id>
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
        pod_id: &PodsPodId,
        api_key_id: &ApiKeyId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/pods/{}/api-keys/{}", pod_id.0, api_key_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
