pub use crate::prelude::*;

/// Identity document being sent, when verifying with `documents`. Decides exactly which file slots to send: `ID_CARD` → `id_card_front` + `id_card_back` + `selfie`; `DRIVERS` → `drivers_front` + `drivers_back` + `selfie`; `RESIDENCE_PERMIT` → `residence_permit_front` + `residence_permit_back` + `selfie`; `PASSPORT` → `passport_front` + `selfie`. See [Identity documents](/developer/verification/identity-documents).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateVerificationsRequestBodyIndividualDocumentType {
    IdCard,
    Drivers,
    ResidencePermit,
    Passport,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateVerificationsRequestBodyIndividualDocumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::IdCard => serializer.serialize_str("ID_CARD"),
            Self::Drivers => serializer.serialize_str("DRIVERS"),
            Self::ResidencePermit => serializer.serialize_str("RESIDENCE_PERMIT"),
            Self::Passport => serializer.serialize_str("PASSPORT"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateVerificationsRequestBodyIndividualDocumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ID_CARD" => Ok(Self::IdCard),
            "DRIVERS" => Ok(Self::Drivers),
            "RESIDENCE_PERMIT" => Ok(Self::ResidencePermit),
            "PASSPORT" => Ok(Self::Passport),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateVerificationsRequestBodyIndividualDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdCard => write!(f, "ID_CARD"),
            Self::Drivers => write!(f, "DRIVERS"),
            Self::ResidencePermit => write!(f, "RESIDENCE_PERMIT"),
            Self::Passport => write!(f, "PASSPORT"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
