pub use crate::prelude::*;

/// The Whop ad group this event's click resolved to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemRelatedAdGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAdGroup {
    pub fn builder() -> ListEventsResponseDataItemRelatedAdGroupBuilder {
        <ListEventsResponseDataItemRelatedAdGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedAdGroupBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAdGroupBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedAdGroup`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedAdGroup, BuildError> {
        Ok(ListEventsResponseDataItemRelatedAdGroup {
            id: self.id,
            title: self.title,
        })
    }
}
