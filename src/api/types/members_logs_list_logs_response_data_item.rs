pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLogsResponseDataItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<ListLogsResponseDataItemActor>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ListLogsResponseDataItem {
    pub fn builder() -> ListLogsResponseDataItemBuilder {
        <ListLogsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLogsResponseDataItemBuilder {
    action: Option<String>,
    actor: Option<ListLogsResponseDataItemActor>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ListLogsResponseDataItemBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn actor(mut self, value: ListLogsResponseDataItemActor) -> Self {
        self.actor = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLogsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListLogsResponseDataItemBuilder::created_at)
    pub fn build(self) -> Result<ListLogsResponseDataItem, BuildError> {
        Ok(ListLogsResponseDataItem {
            action: self.action,
            actor: self.actor,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
