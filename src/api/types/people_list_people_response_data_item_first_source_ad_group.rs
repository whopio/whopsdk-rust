pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPeopleResponseDataItemFirstSourceAdGroup {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}

impl ListPeopleResponseDataItemFirstSourceAdGroup {
    pub fn builder() -> ListPeopleResponseDataItemFirstSourceAdGroupBuilder {
        <ListPeopleResponseDataItemFirstSourceAdGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseDataItemFirstSourceAdGroupBuilder {
    id: Option<String>,
    name: Option<String>,
    thumbnail_url: Option<String>,
}

impl ListPeopleResponseDataItemFirstSourceAdGroupBuilder {
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

    /// Consumes the builder and constructs a [`ListPeopleResponseDataItemFirstSourceAdGroup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListPeopleResponseDataItemFirstSourceAdGroupBuilder::id)
    pub fn build(self) -> Result<ListPeopleResponseDataItemFirstSourceAdGroup, BuildError> {
        Ok(ListPeopleResponseDataItemFirstSourceAdGroup {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            thumbnail_url: self.thumbnail_url,
        })
    }
}
