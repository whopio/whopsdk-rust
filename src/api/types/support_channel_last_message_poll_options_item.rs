pub use crate::prelude::*;

/// Represents a single poll option
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelLastMessagePollOptionsItem {
    /// The unique identifier for the poll option.
    #[serde(default)]
    pub id: String,
}

impl SupportChannelLastMessagePollOptionsItem {
    pub fn builder() -> SupportChannelLastMessagePollOptionsItemBuilder {
        <SupportChannelLastMessagePollOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelLastMessagePollOptionsItemBuilder {
    id: Option<String>,
}

impl SupportChannelLastMessagePollOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelLastMessagePollOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SupportChannelLastMessagePollOptionsItemBuilder::id)
    pub fn build(self) -> Result<SupportChannelLastMessagePollOptionsItem, BuildError> {
        Ok(SupportChannelLastMessagePollOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
