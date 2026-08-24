pub use crate::prelude::*;

/// The object type. Discriminates the create response from a send or a claim link.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostTransferCompletedPayloadDataObject {
    #[serde(rename = "transfer")]
    Transfer,
}
impl fmt::Display for PostTransferCompletedPayloadDataObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Transfer => "transfer",
        };
        write!(f, "{}", s)
    }
}
