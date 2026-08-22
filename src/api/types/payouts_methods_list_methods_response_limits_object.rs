pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListMethodsResponseLimitsObject {
    #[serde(rename = "payout_limit")]
    PayoutLimit,
}
impl fmt::Display for ListMethodsResponseLimitsObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayoutLimit => "payout_limit",
        };
        write!(f, "{}", s)
    }
}
