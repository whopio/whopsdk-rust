pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostSetupIntentRequiresActionPayloadType {
    #[serde(rename = "setup_intent.requires_action")]
    SetupIntentRequiresAction,
}
impl fmt::Display for PostSetupIntentRequiresActionPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SetupIntentRequiresAction => "setup_intent.requires_action",
        };
        write!(f, "{}", s)
    }
}
