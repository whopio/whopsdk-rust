pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgerActivityResourceBankObject {
    #[serde(rename = "payment_method")]
    PaymentMethod,
}
impl fmt::Display for LedgerActivityResourceBankObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentMethod => "payment_method",
        };
        write!(f, "{}", s)
    }
}
