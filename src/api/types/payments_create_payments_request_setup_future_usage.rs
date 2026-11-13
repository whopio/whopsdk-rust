pub use crate::prelude::*;

/// Attests that the buyer was told the payment method will be saved for later charges, and saves it once they approve, so it can be charged with `member_id` and `payment_method_id`. PayPal shows the buyer the agreement to future charges on its approval page. Omit it for a one-time payment that saves nothing. Ignored unless a `paypal` `payment_method` is provided.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreatePaymentsRequestSetupFutureUsage {
    #[serde(rename = "off_session")]
    OffSession,
}
impl fmt::Display for CreatePaymentsRequestSetupFutureUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OffSession => "off_session",
        };
        write!(f, "{}", s)
    }
}
