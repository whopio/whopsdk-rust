pub use crate::prelude::*;

/// The moderation status of the review. Only published reviews are returned.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReviewListItemStatus {
    #[serde(rename = "published")]
    Published,
}
impl fmt::Display for ReviewListItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Published => "published",
        };
        write!(f, "{}", s)
    }
}
