pub use crate::prelude::*;

/// The uploaded file's MIME type. Uploads are restricted to the types the processor accepts.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeEvidenceDocumentContentType {
    ApplicationPdf,
    ApplicationJson,
    ImageJpeg,
    ImagePng,
    ImageWebp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeEvidenceDocumentContentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ApplicationPdf => serializer.serialize_str("application/pdf"),
            Self::ApplicationJson => serializer.serialize_str("application/json"),
            Self::ImageJpeg => serializer.serialize_str("image/jpeg"),
            Self::ImagePng => serializer.serialize_str("image/png"),
            Self::ImageWebp => serializer.serialize_str("image/webp"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeEvidenceDocumentContentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "application/pdf" => Ok(Self::ApplicationPdf),
            "application/json" => Ok(Self::ApplicationJson),
            "image/jpeg" => Ok(Self::ImageJpeg),
            "image/png" => Ok(Self::ImagePng),
            "image/webp" => Ok(Self::ImageWebp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeEvidenceDocumentContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApplicationPdf => write!(f, "application/pdf"),
            Self::ApplicationJson => write!(f, "application/json"),
            Self::ImageJpeg => write!(f, "image/jpeg"),
            Self::ImagePng => write!(f, "image/png"),
            Self::ImageWebp => write!(f, "image/webp"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
