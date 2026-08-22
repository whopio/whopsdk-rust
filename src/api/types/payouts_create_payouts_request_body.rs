pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePayoutsRequestBody {
    Value(serde_json::Value),
}

impl CreatePayoutsRequestBody {
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_value(&self) -> Option<&serde_json::Value> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_value(self) -> Option<serde_json::Value> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }
}
