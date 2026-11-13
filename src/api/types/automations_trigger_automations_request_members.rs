pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TriggerAutomationsRequestMembers {
    StringList(Vec<String>),

    StringToValueMap(HashMap<String, serde_json::Value>),
}

impl TriggerAutomationsRequestMembers {
    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_to_value_map(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            Self::StringToValueMap(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_to_value_map(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
            Self::StringToValueMap(value) => Some(value),
            _ => None,
        }
    }
}
