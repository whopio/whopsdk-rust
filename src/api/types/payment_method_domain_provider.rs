pub use crate::prelude::*;

/// Wallet provider the domain is registered with.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaymentMethodDomainProvider {
    #[serde(rename = "apple")]
    Apple,
}
impl fmt::Display for PaymentMethodDomainProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Apple => "apple",
        };
        write!(f, "{}", s)
    }
}
