pub use crate::prelude::*;

/// Where the domain is moving. `internal` is another Whop account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainTransferDestination {
    #[serde(rename = "internal")]
    Internal,
}
impl fmt::Display for DomainTransferDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Internal => "internal",
        };
        write!(f, "{}", s)
    }
}
