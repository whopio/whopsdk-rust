pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAccountUpdatedPayloadType {
    #[serde(rename = "account.updated")]
    AccountUpdated,
}
impl fmt::Display for PostAccountUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AccountUpdated => "account.updated",
        };
        write!(f, "{}", s)
    }
}
