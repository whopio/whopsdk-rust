pub use crate::prelude::*;

/// The acquirer region `percentage` and `fixed` describe, for a fee that varies by where the money is processed. `null` for a fee that does not vary by region.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountFeeRegion {
    Usa,
    Eu,
    Ca,
    Uk,
    Au,
    Co,
    Mx,
    Ke,
    Cl,
    Pe,
    Ar,
    Cr,
    Gt,
    Uy,
    Br,
    Ph,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountFeeRegion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Usa => serializer.serialize_str("usa"),
            Self::Eu => serializer.serialize_str("eu"),
            Self::Ca => serializer.serialize_str("ca"),
            Self::Uk => serializer.serialize_str("uk"),
            Self::Au => serializer.serialize_str("au"),
            Self::Co => serializer.serialize_str("co"),
            Self::Mx => serializer.serialize_str("mx"),
            Self::Ke => serializer.serialize_str("ke"),
            Self::Cl => serializer.serialize_str("cl"),
            Self::Pe => serializer.serialize_str("pe"),
            Self::Ar => serializer.serialize_str("ar"),
            Self::Cr => serializer.serialize_str("cr"),
            Self::Gt => serializer.serialize_str("gt"),
            Self::Uy => serializer.serialize_str("uy"),
            Self::Br => serializer.serialize_str("br"),
            Self::Ph => serializer.serialize_str("ph"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountFeeRegion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "usa" => Ok(Self::Usa),
            "eu" => Ok(Self::Eu),
            "ca" => Ok(Self::Ca),
            "uk" => Ok(Self::Uk),
            "au" => Ok(Self::Au),
            "co" => Ok(Self::Co),
            "mx" => Ok(Self::Mx),
            "ke" => Ok(Self::Ke),
            "cl" => Ok(Self::Cl),
            "pe" => Ok(Self::Pe),
            "ar" => Ok(Self::Ar),
            "cr" => Ok(Self::Cr),
            "gt" => Ok(Self::Gt),
            "uy" => Ok(Self::Uy),
            "br" => Ok(Self::Br),
            "ph" => Ok(Self::Ph),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountFeeRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usa => write!(f, "usa"),
            Self::Eu => write!(f, "eu"),
            Self::Ca => write!(f, "ca"),
            Self::Uk => write!(f, "uk"),
            Self::Au => write!(f, "au"),
            Self::Co => write!(f, "co"),
            Self::Mx => write!(f, "mx"),
            Self::Ke => write!(f, "ke"),
            Self::Cl => write!(f, "cl"),
            Self::Pe => write!(f, "pe"),
            Self::Ar => write!(f, "ar"),
            Self::Cr => write!(f, "cr"),
            Self::Gt => write!(f, "gt"),
            Self::Uy => write!(f, "uy"),
            Self::Br => write!(f, "br"),
            Self::Ph => write!(f, "ph"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
