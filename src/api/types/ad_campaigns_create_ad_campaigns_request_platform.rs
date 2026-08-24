pub use crate::prelude::*;

/// The ad network the campaign runs on.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateAdCampaignsRequestPlatform {
    #[serde(rename = "meta")]
    Meta,
}
impl fmt::Display for CreateAdCampaignsRequestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Meta => "meta",
        };
        write!(f, "{}", s)
    }
}
