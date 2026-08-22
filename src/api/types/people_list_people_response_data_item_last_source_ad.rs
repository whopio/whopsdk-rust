pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPeopleResponseDataItemLastSourceAd {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}

impl ListPeopleResponseDataItemLastSourceAd {
    pub fn builder() -> ListPeopleResponseDataItemLastSourceAdBuilder {
        <ListPeopleResponseDataItemLastSourceAdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseDataItemLastSourceAdBuilder {
    id: Option<String>,
    name: Option<String>,
    thumbnail_url: Option<String>,
}

impl ListPeopleResponseDataItemLastSourceAdBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPeopleResponseDataItemLastSourceAd`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListPeopleResponseDataItemLastSourceAdBuilder::id)
    pub fn build(self) -> Result<ListPeopleResponseDataItemLastSourceAd, BuildError> {
        Ok(ListPeopleResponseDataItemLastSourceAd {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            thumbnail_url: self.thumbnail_url,
        })
    }
}
