pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SearchTargetingOptionsAdGroupsRequestPlatform {
    #[serde(rename = "meta")]
    Meta,
}
impl fmt::Display for SearchTargetingOptionsAdGroupsRequestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Meta => "meta",
        };
        write!(f, "{}", s)
    }
}
