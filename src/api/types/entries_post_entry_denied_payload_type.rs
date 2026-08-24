pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostEntryDeniedPayloadType {
    #[serde(rename = "entry.denied")]
    EntryDenied,
}
impl fmt::Display for PostEntryDeniedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EntryDenied => "entry.denied",
        };
        write!(f, "{}", s)
    }
}
