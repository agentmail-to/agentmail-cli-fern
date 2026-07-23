pub use crate::prelude::*;
use super::*;

/// Provide at least one of `feedback_enabled` or `subdomains_enabled`. Omitted
/// fields are left unchanged; an empty body is rejected. Enabling
/// `subdomains_enabled` on a verified domain returns it to `PENDING` until the
/// newly-required wildcard MX record (`*.<domain>`) is published and verified.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDomainRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_enabled: Option<FeedbackEnabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomains_enabled: Option<SubdomainsEnabled>,
}

impl UpdateDomainRequest {
    pub fn builder() -> UpdateDomainRequestBuilder {
        <UpdateDomainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDomainRequestBuilder {
    feedback_enabled: Option<FeedbackEnabled>,
    subdomains_enabled: Option<SubdomainsEnabled>,
}

impl UpdateDomainRequestBuilder {
    pub fn feedback_enabled(mut self, value: FeedbackEnabled) -> Self {
        self.feedback_enabled = Some(value);
        self
    }

    pub fn subdomains_enabled(mut self, value: SubdomainsEnabled) -> Self {
        self.subdomains_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDomainRequest`].
    pub fn build(self) -> Result<UpdateDomainRequest, BuildError> {
        Ok(UpdateDomainRequest {
            feedback_enabled: self.feedback_enabled,
            subdomains_enabled: self.subdomains_enabled,
        })
    }
}
