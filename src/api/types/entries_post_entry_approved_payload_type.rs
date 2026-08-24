pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostEntryApprovedPayloadType {
    #[serde(rename = "entry.approved")]
    EntryApproved,
}
impl fmt::Display for PostEntryApprovedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EntryApproved => "entry.approved",
        };
        write!(f, "{}", s)
    }
}
