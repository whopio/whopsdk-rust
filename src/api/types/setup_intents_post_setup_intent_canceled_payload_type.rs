pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostSetupIntentCanceledPayloadType {
    #[serde(rename = "setup_intent.canceled")]
    SetupIntentCanceled,
}
impl fmt::Display for PostSetupIntentCanceledPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SetupIntentCanceled => "setup_intent.canceled",
        };
        write!(f, "{}", s)
    }
}
