pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountSearchResult {
    /// When the account was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub id: String,
    /// Account public route identifier.
    #[serde(default)]
    pub route: String,
    /// Account display name.
    #[serde(default)]
    pub title: String,
}

impl AccountSearchResult {
    pub fn builder() -> AccountSearchResultBuilder {
        <AccountSearchResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSearchResultBuilder {
    created_at: Option<String>,
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl AccountSearchResultBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountSearchResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AccountSearchResultBuilder::created_at)
    /// - [`id`](AccountSearchResultBuilder::id)
    /// - [`route`](AccountSearchResultBuilder::route)
    /// - [`title`](AccountSearchResultBuilder::title)
    pub fn build(self) -> Result<AccountSearchResult, BuildError> {
        Ok(AccountSearchResult {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
