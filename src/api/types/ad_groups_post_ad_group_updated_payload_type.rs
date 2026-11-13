pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAdGroupUpdatedPayloadType {
    #[serde(rename = "ad_group.updated")]
    AdGroupUpdated,
}
impl fmt::Display for PostAdGroupUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AdGroupUpdated => "ad_group.updated",
        };
        write!(f, "{}", s)
    }
}
