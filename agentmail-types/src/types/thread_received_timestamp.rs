pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct ThreadReceivedTimestamp(
    #[serde(deserialize_with = "crate::core::flexible_datetime::offset::deserialize")]
    pub DateTime<FixedOffset>
);