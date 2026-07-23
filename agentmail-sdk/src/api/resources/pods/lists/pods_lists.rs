use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ListsClient3 {
    pub http_client: HttpClient,
}

impl ListsClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:lists list --pod-id <pod_id> --direction <direction> --type <type>
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
        direction: &Direction,
        type_: &ListType,
        request: &PodsListsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PodListListEntriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/lists/{}/{}", pod_id.0, direction, type_),
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
    /// agentmail pods:lists create --pod-id <pod_id> --direction <direction> --type <type> --entry user@example.com
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
        direction: &Direction,
        type_: &ListType,
        request: &CreateListEntryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PodListEntry, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/pods/{}/lists/{}/{}", pod_id.0, direction, type_),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:lists get --pod-id <pod_id> --direction <direction> --type <type> --entry <entry>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `entry` - Email address or domain.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        pod_id: &PodsPodId,
        direction: &Direction,
        type_: &ListType,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<PodListEntry, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v0/pods/{}/lists/{}/{}/{}",
                    pod_id.0, direction, type_, entry
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:lists delete --pod-id <pod_id> --direction <direction> --type <type> --entry <entry>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `entry` - Email address or domain.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn delete(
        &self,
        pod_id: &PodsPodId,
        direction: &Direction,
        type_: &ListType,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v0/pods/{}/lists/{}/{}/{}",
                    pod_id.0, direction, type_, entry
                ),
                None,
                None,
                options,
            )
            .await
    }
}
