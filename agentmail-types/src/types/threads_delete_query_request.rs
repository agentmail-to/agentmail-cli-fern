pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ThreadsDeleteQueryRequest {
    /// If true, permanently delete the thread instead of moving to trash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

impl ThreadsDeleteQueryRequest {
    pub fn builder() -> ThreadsDeleteQueryRequestBuilder {
        <ThreadsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThreadsDeleteQueryRequestBuilder {
    permanent: Option<bool>,
}

impl ThreadsDeleteQueryRequestBuilder {
    pub fn permanent(mut self, value: bool) -> Self {
        self.permanent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThreadsDeleteQueryRequest`].
    pub fn build(self) -> Result<ThreadsDeleteQueryRequest, BuildError> {
        Ok(ThreadsDeleteQueryRequest {
            permanent: self.permanent,
        })
    }
}

