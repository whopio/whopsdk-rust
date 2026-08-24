pub use crate::prelude::*;

/// The company this authorized user has access to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthorizedUserListItemCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl AuthorizedUserListItemCompany {
    pub fn builder() -> AuthorizedUserListItemCompanyBuilder {
        <AuthorizedUserListItemCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizedUserListItemCompanyBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl AuthorizedUserListItemCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthorizedUserListItemCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AuthorizedUserListItemCompanyBuilder::id)
    /// - [`title`](AuthorizedUserListItemCompanyBuilder::title)
    pub fn build(self) -> Result<AuthorizedUserListItemCompany, BuildError> {
        Ok(AuthorizedUserListItemCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
