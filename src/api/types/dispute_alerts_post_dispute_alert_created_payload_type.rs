pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDisputeAlertCreatedPayloadType {
    #[serde(rename = "dispute_alert.created")]
    DisputeAlertCreated,
}
impl fmt::Display for PostDisputeAlertCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DisputeAlertCreated => "dispute_alert.created",
        };
        write!(f, "{}", s)
    }
}
