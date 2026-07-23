pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryMetricsResponse(pub HashMap<String, Vec<MetricBucket>>);