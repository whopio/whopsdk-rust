pub use crate::prelude::*;

/// Why the stream cannot be adjusted: `no_merchant_fee` (the business is charged nothing on it), `commission_fixed` (the commission is a flat share a fee discount may not move), `no_headroom` (already at the floor), `fee_unavailable` (no fee or rate could be resolved). Null when adjustable.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeeOptionsPartnersResponseStreamsItemUnadjustableReason {
    NoMerchantFee,
    CommissionFixed,
    NoHeadroom,
    FeeUnavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FeeOptionsPartnersResponseStreamsItemUnadjustableReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NoMerchantFee => serializer.serialize_str("no_merchant_fee"),
            Self::CommissionFixed => serializer.serialize_str("commission_fixed"),
            Self::NoHeadroom => serializer.serialize_str("no_headroom"),
            Self::FeeUnavailable => serializer.serialize_str("fee_unavailable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FeeOptionsPartnersResponseStreamsItemUnadjustableReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "no_merchant_fee" => Ok(Self::NoMerchantFee),
            "commission_fixed" => Ok(Self::CommissionFixed),
            "no_headroom" => Ok(Self::NoHeadroom),
            "fee_unavailable" => Ok(Self::FeeUnavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FeeOptionsPartnersResponseStreamsItemUnadjustableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMerchantFee => write!(f, "no_merchant_fee"),
            Self::CommissionFixed => write!(f, "commission_fixed"),
            Self::NoHeadroom => write!(f, "no_headroom"),
            Self::FeeUnavailable => write!(f, "fee_unavailable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
