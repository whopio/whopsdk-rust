pub use crate::prelude::*;

/// The available languages for a course
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Languages {
    En,
    Es,
    It,
    Pt,
    De,
    Fr,
    Pl,
    Ru,
    Nl,
    Ca,
    Tr,
    Sv,
    Uk,
    No,
    Fi,
    Sk,
    El,
    Cs,
    Hr,
    Da,
    Ro,
    Bg,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Languages {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::En => serializer.serialize_str("en"),
            Self::Es => serializer.serialize_str("es"),
            Self::It => serializer.serialize_str("it"),
            Self::Pt => serializer.serialize_str("pt"),
            Self::De => serializer.serialize_str("de"),
            Self::Fr => serializer.serialize_str("fr"),
            Self::Pl => serializer.serialize_str("pl"),
            Self::Ru => serializer.serialize_str("ru"),
            Self::Nl => serializer.serialize_str("nl"),
            Self::Ca => serializer.serialize_str("ca"),
            Self::Tr => serializer.serialize_str("tr"),
            Self::Sv => serializer.serialize_str("sv"),
            Self::Uk => serializer.serialize_str("uk"),
            Self::No => serializer.serialize_str("no"),
            Self::Fi => serializer.serialize_str("fi"),
            Self::Sk => serializer.serialize_str("sk"),
            Self::El => serializer.serialize_str("el"),
            Self::Cs => serializer.serialize_str("cs"),
            Self::Hr => serializer.serialize_str("hr"),
            Self::Da => serializer.serialize_str("da"),
            Self::Ro => serializer.serialize_str("ro"),
            Self::Bg => serializer.serialize_str("bg"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Languages {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "it" => Ok(Self::It),
            "pt" => Ok(Self::Pt),
            "de" => Ok(Self::De),
            "fr" => Ok(Self::Fr),
            "pl" => Ok(Self::Pl),
            "ru" => Ok(Self::Ru),
            "nl" => Ok(Self::Nl),
            "ca" => Ok(Self::Ca),
            "tr" => Ok(Self::Tr),
            "sv" => Ok(Self::Sv),
            "uk" => Ok(Self::Uk),
            "no" => Ok(Self::No),
            "fi" => Ok(Self::Fi),
            "sk" => Ok(Self::Sk),
            "el" => Ok(Self::El),
            "cs" => Ok(Self::Cs),
            "hr" => Ok(Self::Hr),
            "da" => Ok(Self::Da),
            "ro" => Ok(Self::Ro),
            "bg" => Ok(Self::Bg),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Languages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::En => write!(f, "en"),
            Self::Es => write!(f, "es"),
            Self::It => write!(f, "it"),
            Self::Pt => write!(f, "pt"),
            Self::De => write!(f, "de"),
            Self::Fr => write!(f, "fr"),
            Self::Pl => write!(f, "pl"),
            Self::Ru => write!(f, "ru"),
            Self::Nl => write!(f, "nl"),
            Self::Ca => write!(f, "ca"),
            Self::Tr => write!(f, "tr"),
            Self::Sv => write!(f, "sv"),
            Self::Uk => write!(f, "uk"),
            Self::No => write!(f, "no"),
            Self::Fi => write!(f, "fi"),
            Self::Sk => write!(f, "sk"),
            Self::El => write!(f, "el"),
            Self::Cs => write!(f, "cs"),
            Self::Hr => write!(f, "hr"),
            Self::Da => write!(f, "da"),
            Self::Ro => write!(f, "ro"),
            Self::Bg => write!(f, "bg"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
