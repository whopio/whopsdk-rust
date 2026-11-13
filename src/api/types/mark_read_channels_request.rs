pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkReadChannelsRequest {
    /// Required for DM-backed kinds; not accepted for experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_viewed_at: Option<DateTime<FixedOffset>>,
    /// Messages actually seen in this channel. Optional for all channel kinds; up to 100 IDs. Omitted or empty records no message views. Experience view counts update asynchronously.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewed_message_ids: Option<Vec<String>>,
}

impl MarkReadChannelsRequest {
    pub fn builder() -> MarkReadChannelsRequestBuilder {
        <MarkReadChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkReadChannelsRequestBuilder {
    last_viewed_at: Option<DateTime<FixedOffset>>,
    viewed_message_ids: Option<Vec<String>>,
}

impl MarkReadChannelsRequestBuilder {
    pub fn last_viewed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_viewed_at = Some(value);
        self
    }

    pub fn viewed_message_ids(mut self, value: Vec<String>) -> Self {
        self.viewed_message_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarkReadChannelsRequest`].
    pub fn build(self) -> Result<MarkReadChannelsRequest, BuildError> {
        Ok(MarkReadChannelsRequest {
            last_viewed_at: self.last_viewed_at,
            viewed_message_ids: self.viewed_message_ids,
        })
    }
}
