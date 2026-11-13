pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReviewAccount {
    /// The unique identifier for the account.
    #[serde(default)]
    pub id: String,
    /// URL slug for the account's store page. For example, `pickaxe` in whop.com/pickaxe.
    #[serde(default)]
    pub route: String,
    /// The display name of the account shown to customers.
    #[serde(default)]
    pub title: String,
}

impl ReviewAccount {
    pub fn builder() -> ReviewAccountBuilder {
        <ReviewAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewAccountBuilder {
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl ReviewAccountBuilder {
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

    /// Consumes the builder and constructs a [`ReviewAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReviewAccountBuilder::id)
    /// - [`route`](ReviewAccountBuilder::route)
    /// - [`title`](ReviewAccountBuilder::title)
    pub fn build(self) -> Result<ReviewAccount, BuildError> {
        Ok(ReviewAccount {
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
