pub use crate::prelude::*;

/// Identity document in `documents`. Decides which slots to send: `id_card` → `id_card_front` + `id_card_back`; `drivers` → `drivers_front` + `drivers_back`; `residence_permit` → `residence_permit_front` + `residence_permit_back`; `passport` → `passport_front`. Every type also takes `selfie`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateVerificationsRequestBodyBusinessOwnersItemDocumentType {
    IdCard,
    Drivers,
    ResidencePermit,
    Passport,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateVerificationsRequestBodyBusinessOwnersItemDocumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::IdCard => serializer.serialize_str("id_card"),
            Self::Drivers => serializer.serialize_str("drivers"),
            Self::ResidencePermit => serializer.serialize_str("residence_permit"),
            Self::Passport => serializer.serialize_str("passport"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateVerificationsRequestBodyBusinessOwnersItemDocumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "id_card" => Ok(Self::IdCard),
            "drivers" => Ok(Self::Drivers),
            "residence_permit" => Ok(Self::ResidencePermit),
            "passport" => Ok(Self::Passport),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateVerificationsRequestBodyBusinessOwnersItemDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdCard => write!(f, "id_card"),
            Self::Drivers => write!(f, "drivers"),
            Self::ResidencePermit => write!(f, "residence_permit"),
            Self::Passport => write!(f, "passport"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
