pub use crate::prelude::*;

/// Ad platform that maintains membership.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AudienceEngagementPlatform {
    #[serde(rename = "meta")]
    Meta,
}
impl fmt::Display for AudienceEngagementPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Meta => "meta",
        };
        write!(f, "{}", s)
    }
}
