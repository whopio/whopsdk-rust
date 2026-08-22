pub use crate::prelude::*;

/// The company this authorized user has access to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthorizedUserCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl AuthorizedUserCompany {
    pub fn builder() -> AuthorizedUserCompanyBuilder {
        <AuthorizedUserCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizedUserCompanyBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl AuthorizedUserCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthorizedUserCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AuthorizedUserCompanyBuilder::id)
    /// - [`title`](AuthorizedUserCompanyBuilder::title)
    pub fn build(self) -> Result<AuthorizedUserCompany, BuildError> {
        Ok(AuthorizedUserCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
