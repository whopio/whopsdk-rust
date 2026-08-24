pub use crate::prelude::*;

/// The API version for this webhook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostVerificationSucceededPayloadApiVersion {
    #[serde(rename = "v1")]
    V1,
}
impl fmt::Display for PostVerificationSucceededPayloadApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1 => "v1",
        };
        write!(f, "{}", s)
    }
}
