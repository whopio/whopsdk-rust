pub use crate::prelude::*;

/// The Whop ad this event's click resolved to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemRelatedAd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAd {
    pub fn builder() -> ListEventsResponseDataItemRelatedAdBuilder {
        <ListEventsResponseDataItemRelatedAdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedAdBuilder {
    id: Option<String>,
    thumbnail_url: Option<String>,
    title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAdBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedAd`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedAd, BuildError> {
        Ok(ListEventsResponseDataItemRelatedAd {
            id: self.id,
            thumbnail_url: self.thumbnail_url,
            title: self.title,
        })
    }
}
