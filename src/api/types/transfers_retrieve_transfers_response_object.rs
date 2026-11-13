pub use crate::prelude::*;

/// The object type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrieveTransfersResponseObject {
    #[serde(rename = "transfer")]
    Transfer,
}
impl fmt::Display for RetrieveTransfersResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Transfer => "transfer",
        };
        write!(f, "{}", s)
    }
}
