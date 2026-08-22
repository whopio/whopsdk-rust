pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaymentRequiredErrorBodyErrorType {
    #[serde(rename = "payment_required")]
    PaymentRequired,
}
impl fmt::Display for PaymentRequiredErrorBodyErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentRequired => "payment_required",
        };
        write!(f, "{}", s)
    }
}
