pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupBehaviorCategory {
    /// On ad platforms that scope behavior categories, what this one is measured on. Send back the value the targeting_options endpoint returned alongside the id. Absent on platforms that don't scope them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_type: Option<AdGroupBehaviorCategoryBehaviorType>,
    /// The ad platform's ID for the category in its targeting taxonomy.
    #[serde(default)]
    pub id: String,
    /// Category name, such as `Frequent travelers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// On ad platforms that scope behavior categories, how many days of activity the category covers. Absent on platforms that don't scope them.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub period: Option<f64>,
}

impl AdGroupBehaviorCategory {
    pub fn builder() -> AdGroupBehaviorCategoryBuilder {
        <AdGroupBehaviorCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupBehaviorCategoryBuilder {
    behavior_type: Option<AdGroupBehaviorCategoryBehaviorType>,
    id: Option<String>,
    name: Option<String>,
    period: Option<f64>,
}

impl AdGroupBehaviorCategoryBuilder {
    pub fn behavior_type(mut self, value: AdGroupBehaviorCategoryBehaviorType) -> Self {
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

    pub fn period(mut self, value: f64) -> Self {
        self.period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupBehaviorCategory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdGroupBehaviorCategoryBuilder::id)
    pub fn build(self) -> Result<AdGroupBehaviorCategory, BuildError> {
        Ok(AdGroupBehaviorCategory {
            behavior_type: self.behavior_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            period: self.period,
        })
    }
}
