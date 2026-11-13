pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostEventsCreatedPayloadType {
    #[serde(rename = "events.created")]
    EventsCreated,
}
impl fmt::Display for PostEventsCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EventsCreated => "events.created",
        };
        write!(f, "{}", s)
    }
}
