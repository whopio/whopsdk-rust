pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrievePayoutsResponseObject {
    #[serde(rename = "payout")]
    Payout,
}
impl fmt::Display for RetrievePayoutsResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Payout => "payout",
        };
        write!(f, "{}", s)
    }
}
