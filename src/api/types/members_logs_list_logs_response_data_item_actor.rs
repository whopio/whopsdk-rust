pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLogsResponseDataItemActor {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl ListLogsResponseDataItemActor {
    pub fn builder() -> ListLogsResponseDataItemActorBuilder {
        <ListLogsResponseDataItemActorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLogsResponseDataItemActorBuilder {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl ListLogsResponseDataItemActorBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListLogsResponseDataItemActor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListLogsResponseDataItemActorBuilder::id)
    pub fn build(self) -> Result<ListLogsResponseDataItemActor, BuildError> {
        Ok(ListLogsResponseDataItemActor {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self.username,
        })
    }
}
