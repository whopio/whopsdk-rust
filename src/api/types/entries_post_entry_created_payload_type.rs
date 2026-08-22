pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostEntryCreatedPayloadType {
    #[serde(rename = "entry.created")]
    EntryCreated,
}
impl fmt::Display for PostEntryCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EntryCreated => "entry.created",
        };
        write!(f, "{}", s)
    }
}
