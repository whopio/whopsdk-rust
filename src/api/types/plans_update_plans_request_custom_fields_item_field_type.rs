pub use crate::prelude::*;

/// The type of the custom field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdatePlansRequestCustomFieldsItemFieldType {
    #[serde(rename = "text")]
    Text,
}
impl fmt::Display for UpdatePlansRequestCustomFieldsItemFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
        };
        write!(f, "{}", s)
    }
}
