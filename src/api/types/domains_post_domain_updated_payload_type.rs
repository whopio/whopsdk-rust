pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDomainUpdatedPayloadType {
    #[serde(rename = "domain.updated")]
    DomainUpdated,
}
impl fmt::Display for PostDomainUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DomainUpdated => "domain.updated",
        };
        write!(f, "{}", s)
    }
}
