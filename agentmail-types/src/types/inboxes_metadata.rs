pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InboxesMetadata(pub HashMap<String, InboxesMetadataValue>);