pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDomainDeletedPayloadType {
    #[serde(rename = "domain.deleted")]
    DomainDeleted,
}
impl fmt::Display for PostDomainDeletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DomainDeleted => "domain.deleted",
        };
        write!(f, "{}", s)
    }
}
