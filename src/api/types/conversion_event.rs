pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ConversionEvent {
    ConversionEventZero(ConversionEventZero),

    String(String),
}

impl ConversionEvent {
    pub fn is_conversion_event_zero(&self) -> bool {
        matches!(self, Self::ConversionEventZero(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_conversion_event_zero(&self) -> Option<&ConversionEventZero> {
        match self {
            Self::ConversionEventZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_conversion_event_zero(self) -> Option<ConversionEventZero> {
        match self {
            Self::ConversionEventZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ConversionEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversionEventZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
