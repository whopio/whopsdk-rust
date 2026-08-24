pub use crate::prelude::*;

/// Custom field input type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PlanCustomFieldFieldType {
    #[serde(rename = "text")]
    Text,
}
impl fmt::Display for PlanCustomFieldFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
        };
        write!(f, "{}", s)
    }
}
