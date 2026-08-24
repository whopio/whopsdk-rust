pub use crate::prelude::*;

/// Why Whop disabled this webhook. `delivery_failures` means every delivery failed for 3 days straight. `null` when `disabled_at` is `null`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WebhookListItemDisabledReason {
    #[serde(rename = "delivery_failures")]
    DeliveryFailures,
}
impl fmt::Display for WebhookListItemDisabledReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DeliveryFailures => "delivery_failures",
        };
        write!(f, "{}", s)
    }
}
