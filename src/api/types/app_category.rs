pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppCategory {
    /// App Store category ID, prefixed `mcat_`. Pass this as `category_id` when updating an app.
    #[serde(default)]
    pub id: String,
    /// Display name of the App Store category, such as `Business & Productivity`.
    #[serde(default)]
    pub name: String,
    /// Category slug, such as `business-productivity`. Pass this as `marketplace_category_route` when listing apps.
    #[serde(default)]
    pub route: String,
}

impl AppCategory {
    pub fn builder() -> AppCategoryBuilder {
        <AppCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppCategoryBuilder {
    id: Option<String>,
    name: Option<String>,
    route: Option<String>,
}

impl AppCategoryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppCategory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AppCategoryBuilder::id)
    /// - [`name`](AppCategoryBuilder::name)
    /// - [`route`](AppCategoryBuilder::route)
    pub fn build(self) -> Result<AppCategory, BuildError> {
        Ok(AppCategory {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
        })
    }
}
