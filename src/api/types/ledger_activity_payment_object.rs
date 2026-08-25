pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgerActivityPaymentObject {
    #[serde(rename = "payment")]
    Payment,
}
impl fmt::Display for LedgerActivityPaymentObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Payment => "payment",
        };
        write!(f, "{}", s)
    }
}
