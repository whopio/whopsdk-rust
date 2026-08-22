pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemRelatedApp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ListEventsResponseDataItemRelatedApp {
    pub fn builder() -> ListEventsResponseDataItemRelatedAppBuilder {
        <ListEventsResponseDataItemRelatedAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedAppBuilder {
    domain_id: Option<String>,
    icon_url: Option<String>,
    id: Option<String>,
    title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAppBuilder {
    pub fn domain_id(mut self, value: impl Into<String>) -> Self {
        self.domain_id = Some(value.into());
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedApp`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedApp, BuildError> {
        Ok(ListEventsResponseDataItemRelatedApp {
            domain_id: self.domain_id,
            icon_url: self.icon_url,
            id: self.id,
            title: self.title,
        })
    }
}
