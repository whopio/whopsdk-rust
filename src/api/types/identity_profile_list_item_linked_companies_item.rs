pub use crate::prelude::*;

/// A company is a seller on Whop. Companies own products, manage members, and receive payouts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IdentityProfileListItemLinkedCompaniesItem {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl IdentityProfileListItemLinkedCompaniesItem {
    pub fn builder() -> IdentityProfileListItemLinkedCompaniesItemBuilder {
        <IdentityProfileListItemLinkedCompaniesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentityProfileListItemLinkedCompaniesItemBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl IdentityProfileListItemLinkedCompaniesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IdentityProfileListItemLinkedCompaniesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](IdentityProfileListItemLinkedCompaniesItemBuilder::id)
    /// - [`title`](IdentityProfileListItemLinkedCompaniesItemBuilder::title)
    pub fn build(self) -> Result<IdentityProfileListItemLinkedCompaniesItem, BuildError> {
        Ok(IdentityProfileListItemLinkedCompaniesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
