pub use crate::prelude::*;

/// Why this method is unavailable: `destination_retired` means the payout provider stopped offering the destination. Whop may automatically remap an eligible method that was not linked through Plaid to a compatible replacement; otherwise, the account owner must re-add it. `null` means no unavailability reason is known.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateMethodsResponseUnavailableReason {
    #[serde(rename = "destination_retired")]
    DestinationRetired,
}
impl fmt::Display for CreateMethodsResponseUnavailableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DestinationRetired => "destination_retired",
        };
        write!(f, "{}", s)
    }
}
