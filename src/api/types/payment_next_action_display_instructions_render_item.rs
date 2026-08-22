pub use crate::prelude::*;

/// Where this step can be presented: `inline` inside your own page, `full_page` as a top-level navigation. Pick whichever listed mode suits your surface.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentNextActionDisplayInstructionsRenderItem {
    Inline,
    FullPage,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentNextActionDisplayInstructionsRenderItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Inline => serializer.serialize_str("inline"),
            Self::FullPage => serializer.serialize_str("full_page"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentNextActionDisplayInstructionsRenderItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "inline" => Ok(Self::Inline),
            "full_page" => Ok(Self::FullPage),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentNextActionDisplayInstructionsRenderItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inline => write!(f, "inline"),
            Self::FullPage => write!(f, "full_page"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
