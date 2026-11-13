pub use crate::prelude::*;

/// Where the domain moves. `internal` moves it to another Whop account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateDomainsRequestTransferDestination {
    #[serde(rename = "internal")]
    Internal,
}
impl fmt::Display for CreateDomainsRequestTransferDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Internal => "internal",
        };
        write!(f, "{}", s)
    }
}
