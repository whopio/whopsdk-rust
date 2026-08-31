pub use crate::prelude::*;

/// The selected identity-document type from the method's payer_document_requirements entry.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateConfirmationTokensRequestPaymentMethodPayerDocumentType {
    Dni,
    Cuil,
    Cuit,
    Passport,
    Cc,
    Ci,
    Rut,
    Curp,
    Rfc,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateConfirmationTokensRequestPaymentMethodPayerDocumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dni => serializer.serialize_str("dni"),
            Self::Cuil => serializer.serialize_str("cuil"),
            Self::Cuit => serializer.serialize_str("cuit"),
            Self::Passport => serializer.serialize_str("passport"),
            Self::Cc => serializer.serialize_str("cc"),
            Self::Ci => serializer.serialize_str("ci"),
            Self::Rut => serializer.serialize_str("rut"),
            Self::Curp => serializer.serialize_str("curp"),
            Self::Rfc => serializer.serialize_str("rfc"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateConfirmationTokensRequestPaymentMethodPayerDocumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "dni" => Ok(Self::Dni),
            "cuil" => Ok(Self::Cuil),
            "cuit" => Ok(Self::Cuit),
            "passport" => Ok(Self::Passport),
            "cc" => Ok(Self::Cc),
            "ci" => Ok(Self::Ci),
            "rut" => Ok(Self::Rut),
            "curp" => Ok(Self::Curp),
            "rfc" => Ok(Self::Rfc),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateConfirmationTokensRequestPaymentMethodPayerDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dni => write!(f, "dni"),
            Self::Cuil => write!(f, "cuil"),
            Self::Cuit => write!(f, "cuit"),
            Self::Passport => write!(f, "passport"),
            Self::Cc => write!(f, "cc"),
            Self::Ci => write!(f, "ci"),
            Self::Rut => write!(f, "rut"),
            Self::Curp => write!(f, "curp"),
            Self::Rfc => write!(f, "rfc"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
