pub use crate::prelude::*;

/// Why the caller may not change this fee, or `null` when `adjustable`. `not_permitted` when the caller has no say over it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AccountFeeUnadjustableReason {
    #[serde(rename = "not_permitted")]
    NotPermitted,
}
impl fmt::Display for AccountFeeUnadjustableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NotPermitted => "not_permitted",
        };
        write!(f, "{}", s)
    }
}
