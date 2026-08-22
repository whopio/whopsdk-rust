pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardApplicationApprovedPayloadDataObject {
    #[serde(rename = "card_application")]
    CardApplication,
}
impl fmt::Display for PostCardApplicationApprovedPayloadDataObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardApplication => "card_application",
        };
        write!(f, "{}", s)
    }
}
