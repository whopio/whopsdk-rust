pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListSupportedMethodsResponseDataItemRequiredFieldsItemObject {
    #[serde(rename = "required_field")]
    RequiredField,
}
impl fmt::Display for ListSupportedMethodsResponseDataItemRequiredFieldsItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RequiredField => "required_field",
        };
        write!(f, "{}", s)
    }
}
