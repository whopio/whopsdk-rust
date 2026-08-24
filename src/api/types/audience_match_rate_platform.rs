pub use crate::prelude::*;

/// The ad platform that provided the match-rate estimate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AudienceMatchRatePlatform {
    #[serde(rename = "meta")]
    Meta,
}
impl fmt::Display for AudienceMatchRatePlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Meta => "meta",
        };
        write!(f, "{}", s)
    }
}
