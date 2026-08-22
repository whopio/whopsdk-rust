pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupTargetingCategory {
    /// The ad platform's ID for the category in its targeting taxonomy.
    #[serde(default)]
    pub id: String,
    /// Category name, such as `Movies`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AdGroupTargetingCategory {
    pub fn builder() -> AdGroupTargetingCategoryBuilder {
        <AdGroupTargetingCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupTargetingCategoryBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl AdGroupTargetingCategoryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdGroupTargetingCategory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdGroupTargetingCategoryBuilder::id)
    pub fn build(self) -> Result<AdGroupTargetingCategory, BuildError> {
        Ok(AdGroupTargetingCategory {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
