pub use crate::prelude::*;

/// Set to active to launch a draft campaign (moderates and pushes it live). Live-campaign pause and resume use the pause and unpause actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateAdCampaignsRequestStatus {
    #[serde(rename = "active")]
    Active,
}
impl fmt::Display for UpdateAdCampaignsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Active => "active",
        };
        write!(f, "{}", s)
    }
}
