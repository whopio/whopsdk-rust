pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateDepositsResponseObject {
    #[serde(rename = "deposit")]
    Deposit,
}
impl fmt::Display for CreateDepositsResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Deposit => "deposit",
        };
        write!(f, "{}", s)
    }
}
