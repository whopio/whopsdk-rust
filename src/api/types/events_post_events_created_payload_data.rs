pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostEventsCreatedPayloadData {
    /// Up to 50 of the account's new events, in the order Whop recorded them.
    #[serde(default)]
    pub events: Vec<PostEventsCreatedPayloadDataEventsItem>,
    /// The batch id. The same events always carry the same id, so a redelivered batch can be ignored.
    #[serde(default)]
    pub id: String,
}

impl PostEventsCreatedPayloadData {
    pub fn builder() -> PostEventsCreatedPayloadDataBuilder {
        <PostEventsCreatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostEventsCreatedPayloadDataBuilder {
    events: Option<Vec<PostEventsCreatedPayloadDataEventsItem>>,
    id: Option<String>,
}

impl PostEventsCreatedPayloadDataBuilder {
    pub fn events(mut self, value: Vec<PostEventsCreatedPayloadDataEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostEventsCreatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](PostEventsCreatedPayloadDataBuilder::events)
    /// - [`id`](PostEventsCreatedPayloadDataBuilder::id)
    pub fn build(self) -> Result<PostEventsCreatedPayloadData, BuildError> {
        Ok(PostEventsCreatedPayloadData {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
