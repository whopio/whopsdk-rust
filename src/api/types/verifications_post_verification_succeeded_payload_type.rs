pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostVerificationSucceededPayloadType {
    #[serde(rename = "verification.succeeded")]
    VerificationSucceeded,
}
impl fmt::Display for PostVerificationSucceededPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::VerificationSucceeded => "verification.succeeded",
        };
        write!(f, "{}", s)
    }
}
