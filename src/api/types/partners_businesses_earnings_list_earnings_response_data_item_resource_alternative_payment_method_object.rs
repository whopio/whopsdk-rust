pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemResourceAlternativePaymentMethodObject {
    #[serde(rename = "receipt")]
    Receipt,
}
impl fmt::Display for ListEarningsResponseDataItemResourceAlternativePaymentMethodObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Receipt => "receipt",
        };
        write!(f, "{}", s)
    }
}
