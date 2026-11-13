pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDomainCreatedPayloadType {
    #[serde(rename = "domain.created")]
    DomainCreated,
}
impl fmt::Display for PostDomainCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DomainCreated => "domain.created",
        };
        write!(f, "{}", s)
    }
}
