pub use crate::prelude::*;

/// How the commission rate was set: `custom` for a rate set for this affiliate, `member` for the member affiliate rate of the plan or product, `global` for the product's global affiliate rate, or `marketplace` for a Whop marketplace referral.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipAffiliateCommissionSource {
    Custom,
    Member,
    Marketplace,
    Global,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipAffiliateCommissionSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Custom => serializer.serialize_str("custom"),
            Self::Member => serializer.serialize_str("member"),
            Self::Marketplace => serializer.serialize_str("marketplace"),
            Self::Global => serializer.serialize_str("global"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipAffiliateCommissionSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "custom" => Ok(Self::Custom),
            "member" => Ok(Self::Member),
            "marketplace" => Ok(Self::Marketplace),
            "global" => Ok(Self::Global),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipAffiliateCommissionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "custom"),
            Self::Member => write!(f, "member"),
            Self::Marketplace => write!(f, "marketplace"),
            Self::Global => write!(f, "global"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
