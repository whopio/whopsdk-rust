pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListCardsResponseDataItemObject {
    #[serde(rename = "card")]
    Card,
}
impl fmt::Display for ListCardsResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Card => "card",
        };
        write!(f, "{}", s)
    }
}
