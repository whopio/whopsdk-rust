pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPayoutCreatedPayloadDataObject {
    #[serde(rename = "payout")]
    Payout,
}
impl fmt::Display for PostPayoutCreatedPayloadDataObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Payout => "payout",
        };
        write!(f, "{}", s)
    }
}
