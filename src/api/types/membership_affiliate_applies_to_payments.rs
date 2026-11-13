pub use crate::prelude::*;

/// Whether the commission is paid on the membership's first payment only or on all its payments.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipAffiliateAppliesToPayments {
    FirstPayment,
    AllPayments,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipAffiliateAppliesToPayments {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FirstPayment => serializer.serialize_str("first_payment"),
            Self::AllPayments => serializer.serialize_str("all_payments"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipAffiliateAppliesToPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "first_payment" => Ok(Self::FirstPayment),
            "all_payments" => Ok(Self::AllPayments),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipAffiliateAppliesToPayments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstPayment => write!(f, "first_payment"),
            Self::AllPayments => write!(f, "all_payments"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
