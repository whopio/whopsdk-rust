pub use crate::prelude::*;

/// An error code for a verification attempt.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerificationErrorCodes {
    Abandoned,
    ConsentDeclined,
    CountryNotSupported,
    DeviceNotSupported,
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    EmailUnverifiedOther,
    EmailVerificationDeclined,
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    PhoneUnverifiedOther,
    PhoneVerificationDeclined,
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    UnderSupportedAge,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VerificationErrorCodes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Abandoned => serializer.serialize_str("abandoned"),
            Self::ConsentDeclined => serializer.serialize_str("consent_declined"),
            Self::CountryNotSupported => serializer.serialize_str("country_not_supported"),
            Self::DeviceNotSupported => serializer.serialize_str("device_not_supported"),
            Self::DocumentExpired => serializer.serialize_str("document_expired"),
            Self::DocumentTypeNotSupported => {
                serializer.serialize_str("document_type_not_supported")
            }
            Self::DocumentUnverifiedOther => serializer.serialize_str("document_unverified_other"),
            Self::EmailUnverifiedOther => serializer.serialize_str("email_unverified_other"),
            Self::EmailVerificationDeclined => {
                serializer.serialize_str("email_verification_declined")
            }
            Self::IdNumberInsufficientDocumentData => {
                serializer.serialize_str("id_number_insufficient_document_data")
            }
            Self::IdNumberMismatch => serializer.serialize_str("id_number_mismatch"),
            Self::IdNumberUnverifiedOther => serializer.serialize_str("id_number_unverified_other"),
            Self::PhoneUnverifiedOther => serializer.serialize_str("phone_unverified_other"),
            Self::PhoneVerificationDeclined => {
                serializer.serialize_str("phone_verification_declined")
            }
            Self::SelfieDocumentMissingPhoto => {
                serializer.serialize_str("selfie_document_missing_photo")
            }
            Self::SelfieFaceMismatch => serializer.serialize_str("selfie_face_mismatch"),
            Self::SelfieManipulated => serializer.serialize_str("selfie_manipulated"),
            Self::SelfieUnverifiedOther => serializer.serialize_str("selfie_unverified_other"),
            Self::UnderSupportedAge => serializer.serialize_str("under_supported_age"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VerificationErrorCodes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "abandoned" => Ok(Self::Abandoned),
            "consent_declined" => Ok(Self::ConsentDeclined),
            "country_not_supported" => Ok(Self::CountryNotSupported),
            "device_not_supported" => Ok(Self::DeviceNotSupported),
            "document_expired" => Ok(Self::DocumentExpired),
            "document_type_not_supported" => Ok(Self::DocumentTypeNotSupported),
            "document_unverified_other" => Ok(Self::DocumentUnverifiedOther),
            "email_unverified_other" => Ok(Self::EmailUnverifiedOther),
            "email_verification_declined" => Ok(Self::EmailVerificationDeclined),
            "id_number_insufficient_document_data" => Ok(Self::IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(Self::IdNumberMismatch),
            "id_number_unverified_other" => Ok(Self::IdNumberUnverifiedOther),
            "phone_unverified_other" => Ok(Self::PhoneUnverifiedOther),
            "phone_verification_declined" => Ok(Self::PhoneVerificationDeclined),
            "selfie_document_missing_photo" => Ok(Self::SelfieDocumentMissingPhoto),
            "selfie_face_mismatch" => Ok(Self::SelfieFaceMismatch),
            "selfie_manipulated" => Ok(Self::SelfieManipulated),
            "selfie_unverified_other" => Ok(Self::SelfieUnverifiedOther),
            "under_supported_age" => Ok(Self::UnderSupportedAge),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VerificationErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abandoned => write!(f, "abandoned"),
            Self::ConsentDeclined => write!(f, "consent_declined"),
            Self::CountryNotSupported => write!(f, "country_not_supported"),
            Self::DeviceNotSupported => write!(f, "device_not_supported"),
            Self::DocumentExpired => write!(f, "document_expired"),
            Self::DocumentTypeNotSupported => write!(f, "document_type_not_supported"),
            Self::DocumentUnverifiedOther => write!(f, "document_unverified_other"),
            Self::EmailUnverifiedOther => write!(f, "email_unverified_other"),
            Self::EmailVerificationDeclined => write!(f, "email_verification_declined"),
            Self::IdNumberInsufficientDocumentData => {
                write!(f, "id_number_insufficient_document_data")
            }
            Self::IdNumberMismatch => write!(f, "id_number_mismatch"),
            Self::IdNumberUnverifiedOther => write!(f, "id_number_unverified_other"),
            Self::PhoneUnverifiedOther => write!(f, "phone_unverified_other"),
            Self::PhoneVerificationDeclined => write!(f, "phone_verification_declined"),
            Self::SelfieDocumentMissingPhoto => write!(f, "selfie_document_missing_photo"),
            Self::SelfieFaceMismatch => write!(f, "selfie_face_mismatch"),
            Self::SelfieManipulated => write!(f, "selfie_manipulated"),
            Self::SelfieUnverifiedOther => write!(f, "selfie_unverified_other"),
            Self::UnderSupportedAge => write!(f, "under_supported_age"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
