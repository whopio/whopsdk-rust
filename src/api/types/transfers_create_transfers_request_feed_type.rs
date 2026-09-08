pub use crate::prelude::*;

/// Ledger transfers only. The type of the feed named by `feed_id`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTransfersRequestFeedType {
    DmsFeed,
    ChatFeed,
    ForumFeed,
    LivestreamFeed,
    UniversalPost,
    User,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTransfersRequestFeedType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DmsFeed => serializer.serialize_str("dms_feed"),
            Self::ChatFeed => serializer.serialize_str("chat_feed"),
            Self::ForumFeed => serializer.serialize_str("forum_feed"),
            Self::LivestreamFeed => serializer.serialize_str("livestream_feed"),
            Self::UniversalPost => serializer.serialize_str("universal_post"),
            Self::User => serializer.serialize_str("user"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTransfersRequestFeedType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "dms_feed" => Ok(Self::DmsFeed),
            "chat_feed" => Ok(Self::ChatFeed),
            "forum_feed" => Ok(Self::ForumFeed),
            "livestream_feed" => Ok(Self::LivestreamFeed),
            "universal_post" => Ok(Self::UniversalPost),
            "user" => Ok(Self::User),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateTransfersRequestFeedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DmsFeed => write!(f, "dms_feed"),
            Self::ChatFeed => write!(f, "chat_feed"),
            Self::ForumFeed => write!(f, "forum_feed"),
            Self::LivestreamFeed => write!(f, "livestream_feed"),
            Self::UniversalPost => write!(f, "universal_post"),
            Self::User => write!(f, "user"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
