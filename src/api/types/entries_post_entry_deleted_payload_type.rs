pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostEntryDeletedPayloadType {
    #[serde(rename = "entry.deleted")]
    EntryDeleted,
}
impl fmt::Display for PostEntryDeletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EntryDeleted => "entry.deleted",
        };
        write!(f, "{}", s)
    }
}
