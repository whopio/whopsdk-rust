pub use crate::prelude::*;

/// The kind of business on the latest application. `null` until the account applies.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdatePreferencesResponseAdsCertificationsItemBusinessType {
    OnlinePharmacy,
    PharmaceuticalManufacturer,
    TelehealthProvider,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdatePreferencesResponseAdsCertificationsItemBusinessType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OnlinePharmacy => serializer.serialize_str("online_pharmacy"),
            Self::PharmaceuticalManufacturer => {
                serializer.serialize_str("pharmaceutical_manufacturer")
            }
            Self::TelehealthProvider => serializer.serialize_str("telehealth_provider"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdatePreferencesResponseAdsCertificationsItemBusinessType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "online_pharmacy" => Ok(Self::OnlinePharmacy),
            "pharmaceutical_manufacturer" => Ok(Self::PharmaceuticalManufacturer),
            "telehealth_provider" => Ok(Self::TelehealthProvider),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdatePreferencesResponseAdsCertificationsItemBusinessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OnlinePharmacy => write!(f, "online_pharmacy"),
            Self::PharmaceuticalManufacturer => write!(f, "pharmaceutical_manufacturer"),
            Self::TelehealthProvider => write!(f, "telehealth_provider"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
