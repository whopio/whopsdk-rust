pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StorefrontAccount {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub id: String,
    /// Account public route identifier — the `whop.com/{route}` storefront path.
    #[serde(default)]
    pub route: String,
    /// Account display name.
    #[serde(default)]
    pub title: String,
}

impl StorefrontAccount {
    pub fn builder() -> StorefrontAccountBuilder {
        <StorefrontAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StorefrontAccountBuilder {
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl StorefrontAccountBuilder {
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

    /// Consumes the builder and constructs a [`StorefrontAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](StorefrontAccountBuilder::id)
    /// - [`route`](StorefrontAccountBuilder::route)
    /// - [`title`](StorefrontAccountBuilder::title)
    pub fn build(self) -> Result<StorefrontAccount, BuildError> {
        Ok(StorefrontAccount {
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
