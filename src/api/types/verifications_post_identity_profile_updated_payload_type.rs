pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostIdentityProfileUpdatedPayloadType {
    #[serde(rename = "identity_profile.updated")]
    IdentityProfileUpdated,
}
impl fmt::Display for PostIdentityProfileUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::IdentityProfileUpdated => "identity_profile.updated",
        };
        write!(f, "{}", s)
    }
}
