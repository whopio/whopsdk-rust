pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostSwapCompletedPayloadType {
    #[serde(rename = "swap.completed")]
    SwapCompleted,
}
impl fmt::Display for PostSwapCompletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SwapCompleted => "swap.completed",
        };
        write!(f, "{}", s)
    }
}
