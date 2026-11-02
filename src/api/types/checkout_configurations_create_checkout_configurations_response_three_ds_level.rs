pub use crate::prelude::*;

/// 3D Secure behavior for supported on-session card payments. `mandate_challenge` requires a 3DS challenge before payment processing; `mandate_if_required` mandates a challenge only when the payment processor requires it; `frictionless_if_required` uses the regular frictionless 3DS flow. Payments of $1,000 or more use `mandate_if_required` unless `mandate_challenge` is selected. Risk and authentication recovery requirements can override the preference. Applies in setup mode; `null` uses frictionless 3DS. Payment mode uses the plan policy.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateCheckoutConfigurationsResponseThreeDsLevel {
    MandateChallenge,
    MandateIfRequired,
    FrictionlessIfRequired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateCheckoutConfigurationsResponseThreeDsLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MandateChallenge => serializer.serialize_str("mandate_challenge"),
            Self::MandateIfRequired => serializer.serialize_str("mandate_if_required"),
            Self::FrictionlessIfRequired => serializer.serialize_str("frictionless_if_required"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateCheckoutConfigurationsResponseThreeDsLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mandate_challenge" => Ok(Self::MandateChallenge),
            "mandate_if_required" => Ok(Self::MandateIfRequired),
            "frictionless_if_required" => Ok(Self::FrictionlessIfRequired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateCheckoutConfigurationsResponseThreeDsLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MandateChallenge => write!(f, "mandate_challenge"),
            Self::MandateIfRequired => write!(f, "mandate_if_required"),
            Self::FrictionlessIfRequired => write!(f, "frictionless_if_required"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
