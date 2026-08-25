pub use crate::prelude::*;

/// What kind of input to render. `text` today.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckoutSessionCustomFieldFieldType {
    #[serde(rename = "text")]
    Text,
}
impl fmt::Display for CheckoutSessionCustomFieldFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
        };
        write!(f, "{}", s)
    }
}
