pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardApplicationUpdatedPayloadDataObject {
    #[serde(rename = "card_application")]
    CardApplication,
}
impl fmt::Display for PostCardApplicationUpdatedPayloadDataObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardApplication => "card_application",
        };
        write!(f, "{}", s)
    }
}
