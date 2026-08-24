pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupDetailedTargetingBodyInterestsItem {
    /// The ad platform's ID for the category in its targeting taxonomy.
    #[serde(default)]
    pub id: String,
    /// Category name, such as `Movies`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AdGroupDetailedTargetingBodyInterestsItem {
    pub fn builder() -> AdGroupDetailedTargetingBodyInterestsItemBuilder {
        <AdGroupDetailedTargetingBodyInterestsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDetailedTargetingBodyInterestsItemBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl AdGroupDetailedTargetingBodyInterestsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDetailedTargetingBodyInterestsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdGroupDetailedTargetingBodyInterestsItemBuilder::id)
    pub fn build(self) -> Result<AdGroupDetailedTargetingBodyInterestsItem, BuildError> {
        Ok(AdGroupDetailedTargetingBodyInterestsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
