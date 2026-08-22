pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListPaymentMethodDomainsRequestProvider {
    #[serde(rename = "apple")]
    Apple,
}
impl fmt::Display for ListPaymentMethodDomainsRequestProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Apple => "apple",
        };
        write!(f, "{}", s)
    }
}
