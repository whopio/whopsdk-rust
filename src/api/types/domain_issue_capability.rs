pub use crate::prelude::*;

/// The capability the issue is about.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainIssueCapability {
    Verification,
    Registration,
    Platform,
    Subdomain,
    Website,
    ApplePay,
    Pixel,
    Email,
    Transfer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainIssueCapability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Verification => serializer.serialize_str("verification"),
            Self::Registration => serializer.serialize_str("registration"),
            Self::Platform => serializer.serialize_str("platform"),
            Self::Subdomain => serializer.serialize_str("subdomain"),
            Self::Website => serializer.serialize_str("website"),
            Self::ApplePay => serializer.serialize_str("apple_pay"),
            Self::Pixel => serializer.serialize_str("pixel"),
            Self::Email => serializer.serialize_str("email"),
            Self::Transfer => serializer.serialize_str("transfer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainIssueCapability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "verification" => Ok(Self::Verification),
            "registration" => Ok(Self::Registration),
            "platform" => Ok(Self::Platform),
            "subdomain" => Ok(Self::Subdomain),
            "website" => Ok(Self::Website),
            "apple_pay" => Ok(Self::ApplePay),
            "pixel" => Ok(Self::Pixel),
            "email" => Ok(Self::Email),
            "transfer" => Ok(Self::Transfer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainIssueCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Verification => write!(f, "verification"),
            Self::Registration => write!(f, "registration"),
            Self::Platform => write!(f, "platform"),
            Self::Subdomain => write!(f, "subdomain"),
            Self::Website => write!(f, "website"),
            Self::ApplePay => write!(f, "apple_pay"),
            Self::Pixel => write!(f, "pixel"),
            Self::Email => write!(f, "email"),
            Self::Transfer => write!(f, "transfer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
