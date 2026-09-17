pub use crate::prelude::*;

/// Why the caller may not change this markup, or `null` when `adjustable`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AccountFeeMarkupUnadjustableReason {
    #[serde(rename = "not_permitted")]
    NotPermitted,
}
impl fmt::Display for AccountFeeMarkupUnadjustableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NotPermitted => "not_permitted",
        };
        write!(f, "{}", s)
    }
}
