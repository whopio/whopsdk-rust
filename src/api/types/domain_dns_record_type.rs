pub use crate::prelude::*;

/// DNS record type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainDnsRecordType {
    Txt,
    Cname,
    A,
    Aaaa,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainDnsRecordType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Txt => serializer.serialize_str("TXT"),
            Self::Cname => serializer.serialize_str("CNAME"),
            Self::A => serializer.serialize_str("A"),
            Self::Aaaa => serializer.serialize_str("AAAA"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainDnsRecordType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TXT" => Ok(Self::Txt),
            "CNAME" => Ok(Self::Cname),
            "A" => Ok(Self::A),
            "AAAA" => Ok(Self::Aaaa),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainDnsRecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Txt => write!(f, "TXT"),
            Self::Cname => write!(f, "CNAME"),
            Self::A => write!(f, "A"),
            Self::Aaaa => write!(f, "AAAA"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
