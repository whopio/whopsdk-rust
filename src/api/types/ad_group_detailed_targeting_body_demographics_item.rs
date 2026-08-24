pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdGroupDetailedTargetingBodyDemographicsItem {
    /// The ad platform's ID for the category in its targeting taxonomy.
    #[serde(default)]
    pub id: String,
    /// Category name, such as `Recently moved`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Kind of demographic the category belongs to.
    pub r#type: AdGroupDetailedTargetingBodyDemographicsItemType,
}

impl AdGroupDetailedTargetingBodyDemographicsItem {
    pub fn builder() -> AdGroupDetailedTargetingBodyDemographicsItemBuilder {
        <AdGroupDetailedTargetingBodyDemographicsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDetailedTargetingBodyDemographicsItemBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<AdGroupDetailedTargetingBodyDemographicsItemType>,
}

impl AdGroupDetailedTargetingBodyDemographicsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AdGroupDetailedTargetingBodyDemographicsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDetailedTargetingBodyDemographicsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdGroupDetailedTargetingBodyDemographicsItemBuilder::id)
    /// - [`r#type`](AdGroupDetailedTargetingBodyDemographicsItemBuilder::r#type)
    pub fn build(self) -> Result<AdGroupDetailedTargetingBodyDemographicsItem, BuildError> {
        Ok(AdGroupDetailedTargetingBodyDemographicsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
