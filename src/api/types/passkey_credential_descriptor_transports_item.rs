pub use crate::prelude::*;

/// How the authenticator holding this passkey can be reached, as it reported at registration. Empty when unknown.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PasskeyCredentialDescriptorTransportsItem {
    Ble,
    Hybrid,
    Internal,
    Nfc,
    SmartCard,
    Usb,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PasskeyCredentialDescriptorTransportsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ble => serializer.serialize_str("ble"),
            Self::Hybrid => serializer.serialize_str("hybrid"),
            Self::Internal => serializer.serialize_str("internal"),
            Self::Nfc => serializer.serialize_str("nfc"),
            Self::SmartCard => serializer.serialize_str("smart-card"),
            Self::Usb => serializer.serialize_str("usb"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PasskeyCredentialDescriptorTransportsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ble" => Ok(Self::Ble),
            "hybrid" => Ok(Self::Hybrid),
            "internal" => Ok(Self::Internal),
            "nfc" => Ok(Self::Nfc),
            "smart-card" => Ok(Self::SmartCard),
            "usb" => Ok(Self::Usb),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PasskeyCredentialDescriptorTransportsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ble => write!(f, "ble"),
            Self::Hybrid => write!(f, "hybrid"),
            Self::Internal => write!(f, "internal"),
            Self::Nfc => write!(f, "nfc"),
            Self::SmartCard => write!(f, "smart-card"),
            Self::Usb => write!(f, "usb"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
