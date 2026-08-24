pub use crate::prelude::*;

/// A product is a digital good or service sold on Whop. Products contain plans for pricing and experiences for content delivery.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperienceProductsItem {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// URL slug in the product's public link, e.g. `pickaxe-analytics` in whop.com/company/pickaxe-analytics.
    #[serde(default)]
    pub route: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl ExperienceProductsItem {
    pub fn builder() -> ExperienceProductsItemBuilder {
        <ExperienceProductsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceProductsItemBuilder {
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl ExperienceProductsItemBuilder {
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

    /// Consumes the builder and constructs a [`ExperienceProductsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExperienceProductsItemBuilder::id)
    /// - [`route`](ExperienceProductsItemBuilder::route)
    /// - [`title`](ExperienceProductsItemBuilder::title)
    pub fn build(self) -> Result<ExperienceProductsItem, BuildError> {
        Ok(ExperienceProductsItem {
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
