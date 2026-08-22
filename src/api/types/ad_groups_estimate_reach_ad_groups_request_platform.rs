pub use crate::prelude::*;

/// The ad network the estimate runs on.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EstimateReachAdGroupsRequestPlatform {
    #[serde(rename = "meta")]
    Meta,
}
impl fmt::Display for EstimateReachAdGroupsRequestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Meta => "meta",
        };
        write!(f, "{}", s)
    }
}
