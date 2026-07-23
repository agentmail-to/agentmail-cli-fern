pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryUsageResponse(pub HashMap<String, Vec<UsagePoint>>);