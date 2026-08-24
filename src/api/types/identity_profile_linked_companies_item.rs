pub use crate::prelude::*;

/// A company is a seller on Whop. Companies own products, manage members, and receive payouts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IdentityProfileLinkedCompaniesItem {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl IdentityProfileLinkedCompaniesItem {
    pub fn builder() -> IdentityProfileLinkedCompaniesItemBuilder {
        <IdentityProfileLinkedCompaniesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentityProfileLinkedCompaniesItemBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl IdentityProfileLinkedCompaniesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IdentityProfileLinkedCompaniesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](IdentityProfileLinkedCompaniesItemBuilder::id)
    /// - [`title`](IdentityProfileLinkedCompaniesItemBuilder::title)
    pub fn build(self) -> Result<IdentityProfileLinkedCompaniesItem, BuildError> {
        Ok(IdentityProfileLinkedCompaniesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
