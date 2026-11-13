pub use crate::prelude::*;

/// The type of the custom field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateVariantsRequestCustomFieldsItemFieldType {
    #[serde(rename = "text")]
    Text,
}
impl fmt::Display for UpdateVariantsRequestCustomFieldsItemFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
        };
        write!(f, "{}", s)
    }
}
