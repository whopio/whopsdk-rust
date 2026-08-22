pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardCanceledPayloadDataObject {
    #[serde(rename = "card")]
    Card,
}
impl fmt::Display for PostCardCanceledPayloadDataObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Card => "card",
        };
        write!(f, "{}", s)
    }
}
