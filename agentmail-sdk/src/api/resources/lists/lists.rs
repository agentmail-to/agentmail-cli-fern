use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ListsClient {
    pub http_client: HttpClient,
}

impl ListsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail lists list --direction <direction> --type <type>
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
        direction: &Direction,
        type_: &ListType,
        request: &ListsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListListEntriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/lists/{}/{}", direction, type_),
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
    /// agentmail lists create --direction <direction> --type <type> --entry user@example.com
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
        direction: &Direction,
        type_: &ListType,
        request: &CreateListEntryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEntry, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/lists/{}/{}", direction, type_),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail lists get --direction <direction> --type <type> --entry <entry>
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
        direction: &Direction,
        type_: &ListType,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListEntry, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/lists/{}/{}/{}", direction, type_, entry),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail lists delete --direction <direction> --type <type> --entry <entry>
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
        direction: &Direction,
        type_: &ListType,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/lists/{}/{}/{}", direction, type_, entry),
                None,
                None,
                options,
            )
            .await
    }
}
