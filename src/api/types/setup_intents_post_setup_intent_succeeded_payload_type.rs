pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostSetupIntentSucceededPayloadType {
    #[serde(rename = "setup_intent.succeeded")]
    SetupIntentSucceeded,
}
impl fmt::Display for PostSetupIntentSucceededPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SetupIntentSucceeded => "setup_intent.succeeded",
        };
        write!(f, "{}", s)
    }
}
