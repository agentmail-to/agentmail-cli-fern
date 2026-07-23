use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DomainsClient2 {
    pub http_client: HttpClient,
}

impl DomainsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:domains list --pod-id <pod_id>
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
        request: &PodsDomainsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDomainsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/domains", pod_id.0),
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

    /// **CLI:**
    /// ```bash
    /// agentmail pods:domains create --pod-id <pod_id> --domain example.com
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
        request: &CreateDomainRequest,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/pods/{}/domains", pod_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:domains get --pod-id <pod_id> --domain-id <domain_id>
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
        pod_id: &PodsPodId,
        domain_id: &DomainId,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/domains/{}", pod_id.0, domain_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:domains delete --pod-id <pod_id> --domain-id <domain_id>
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
        domain_id: &DomainId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/pods/{}/domains/{}", pod_id.0, domain_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:domains update --pod-id <pod_id> --domain-id <domain_id>
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
        pod_id: &PodsPodId,
        domain_id: &DomainId,
        request: &UpdateDomainRequest,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/pods/{}/domains/{}", pod_id.0, domain_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:domains get-zone-file --pod-id <pod_id> --domain-id <domain_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn get_zone_file(
        &self,
        pod_id: &PodsPodId,
        domain_id: &DomainId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/domains/{}/zone-file", pod_id.0, domain_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods:domains verify --pod-id <pod_id> --domain-id <domain_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn verify(
        &self,
        pod_id: &PodsPodId,
        domain_id: &DomainId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/pods/{}/domains/{}/verify", pod_id.0, domain_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
