pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupDetailedTargetingBodyBehaviorsItem {
    /// On ad platforms that scope behavior categories, what this one is measured on. Send back the value the targeting_options endpoint returned alongside the id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_type: Option<AdGroupDetailedTargetingBodyBehaviorsItemBehaviorType>,
    /// The ad platform's ID for the category in its targeting taxonomy.
    #[serde(default)]
    pub id: String,
    /// Category name, such as `Frequent travelers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// On ad platforms that scope behavior categories, how many days of activity the category covers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
}

impl AdGroupDetailedTargetingBodyBehaviorsItem {
    pub fn builder() -> AdGroupDetailedTargetingBodyBehaviorsItemBuilder {
        <AdGroupDetailedTargetingBodyBehaviorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDetailedTargetingBodyBehaviorsItemBuilder {
    behavior_type: Option<AdGroupDetailedTargetingBodyBehaviorsItemBehaviorType>,
    id: Option<String>,
    name: Option<String>,
    period: Option<i64>,
}

impl AdGroupDetailedTargetingBodyBehaviorsItemBuilder {
    pub fn behavior_type(
        mut self,
        value: AdGroupDetailedTargetingBodyBehaviorsItemBehaviorType,
    ) -> Self {
        self.behavior_type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn period(mut self, value: i64) -> Self {
        self.period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDetailedTargetingBodyBehaviorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdGroupDetailedTargetingBodyBehaviorsItemBuilder::id)
    pub fn build(self) -> Result<AdGroupDetailedTargetingBodyBehaviorsItem, BuildError> {
        Ok(AdGroupDetailedTargetingBodyBehaviorsItem {
            behavior_type: self.behavior_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            period: self.period,
        })
    }
}
