pub use crate::prelude::*;

/// Always optional — never blocking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AccountRecommendedActionStatus {
    #[serde(rename = "optional")]
    Optional,
}
impl fmt::Display for AccountRecommendedActionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Optional => "optional",
        };
        write!(f, "{}", s)
    }
}
