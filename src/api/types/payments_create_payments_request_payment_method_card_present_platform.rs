pub use crate::prelude::*;

/// Which Tap to Pay SDK collects the payment. Stripe offers Tap to Pay in different countries per platform, so eligibility is checked for the named one, as it is for the connection-token session.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePaymentsRequestPaymentMethodCardPresentPlatform {
    Ios,
    Android,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePaymentsRequestPaymentMethodCardPresentPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ios => serializer.serialize_str("ios"),
            Self::Android => serializer.serialize_str("android"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePaymentsRequestPaymentMethodCardPresentPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ios" => Ok(Self::Ios),
            "android" => Ok(Self::Android),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePaymentsRequestPaymentMethodCardPresentPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ios => write!(f, "ios"),
            Self::Android => write!(f, "android"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
