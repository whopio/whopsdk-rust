pub use crate::prelude::*;

/// Represents a single poll option
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelListItemLastMessagePollOptionsItem {
    /// The unique identifier for the poll option.
    #[serde(default)]
    pub id: String,
}

impl SupportChannelListItemLastMessagePollOptionsItem {
    pub fn builder() -> SupportChannelListItemLastMessagePollOptionsItemBuilder {
        <SupportChannelListItemLastMessagePollOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelListItemLastMessagePollOptionsItemBuilder {
    id: Option<String>,
}

impl SupportChannelListItemLastMessagePollOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelListItemLastMessagePollOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SupportChannelListItemLastMessagePollOptionsItemBuilder::id)
    pub fn build(self) -> Result<SupportChannelListItemLastMessagePollOptionsItem, BuildError> {
        Ok(SupportChannelListItemLastMessagePollOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
