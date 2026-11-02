pub use crate::prelude::*;

/// `public` files are served via an unsigned CDN URL; `private` files via a signed, expiring URL.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeEvidenceDocumentVisibility {
    Public,
    Private,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeEvidenceDocumentVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Public => serializer.serialize_str("public"),
            Self::Private => serializer.serialize_str("private"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeEvidenceDocumentVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeEvidenceDocumentVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
