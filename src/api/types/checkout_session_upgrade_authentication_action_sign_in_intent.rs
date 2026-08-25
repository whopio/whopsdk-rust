pub use crate::prelude::*;

/// The `intent` to declare on the session-intent create. Declaring it is what keeps the verification credential checkout-scoped — the full login is minted only by the upgrade call, never by the code alone.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckoutSessionUpgradeAuthenticationActionSignInIntent {
    #[serde(rename = "payment")]
    Payment,
}
impl fmt::Display for CheckoutSessionUpgradeAuthenticationActionSignInIntent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Payment => "payment",
        };
        write!(f, "{}", s)
    }
}
