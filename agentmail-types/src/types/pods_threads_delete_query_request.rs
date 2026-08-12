pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodsThreadsDeleteQueryRequest {
    /// If true, permanently delete the thread instead of moving to trash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

impl PodsThreadsDeleteQueryRequest {
    pub fn builder() -> PodsThreadsDeleteQueryRequestBuilder {
        <PodsThreadsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodsThreadsDeleteQueryRequestBuilder {
    permanent: Option<bool>,
}

impl PodsThreadsDeleteQueryRequestBuilder {
    pub fn permanent(mut self, value: bool) -> Self {
        self.permanent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodsThreadsDeleteQueryRequest`].
    pub fn build(self) -> Result<PodsThreadsDeleteQueryRequest, BuildError> {
        Ok(PodsThreadsDeleteQueryRequest {
            permanent: self.permanent,
        })
    }
}

