pub use crate::prelude::*;

/// Always `message`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MemberLastActivityMessageObject {
    #[serde(rename = "message")]
    Message,
}
impl fmt::Display for MemberLastActivityMessageObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Message => "message",
        };
        write!(f, "{}", s)
    }
}
