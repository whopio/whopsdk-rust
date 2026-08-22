pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DetailedTargetingOption {
    /// Low end of the ad platform's estimate of how many people this option can reach. Null when the platform doesn't publish one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub audience_size_lower_bound: Option<f64>,
    /// High end of the ad platform's estimate of how many people this option can reach. Null when the platform doesn't publish one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub audience_size_upper_bound: Option<f64>,
    /// What a behavior category is measured on, on ad platforms that scope them. Send it back on the `detailed_targeting.behaviors` entry alongside the id. Null for options that aren't scoped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_type: Option<DetailedTargetingOptionBehaviorType>,
    /// The ad platform's description of who the option covers, when it publishes one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ad platform's ID for the option in its targeting taxonomy. Use it as the `id` of a `detailed_targeting` entry.
    #[serde(default)]
    pub id: String,
    /// Display name, such as `Movies`.
    #[serde(default)]
    pub name: String,
}

impl DetailedTargetingOption {
    pub fn builder() -> DetailedTargetingOptionBuilder {
        <DetailedTargetingOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetailedTargetingOptionBuilder {
    audience_size_lower_bound: Option<f64>,
    audience_size_upper_bound: Option<f64>,
    behavior_type: Option<DetailedTargetingOptionBehaviorType>,
    description: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

impl DetailedTargetingOptionBuilder {
    pub fn audience_size_lower_bound(mut self, value: f64) -> Self {
        self.audience_size_lower_bound = Some(value);
        self
    }

    pub fn audience_size_upper_bound(mut self, value: f64) -> Self {
        self.audience_size_upper_bound = Some(value);
        self
    }

    pub fn behavior_type(mut self, value: DetailedTargetingOptionBehaviorType) -> Self {
        self.behavior_type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    /// Consumes the builder and constructs a [`DetailedTargetingOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DetailedTargetingOptionBuilder::id)
    /// - [`name`](DetailedTargetingOptionBuilder::name)
    pub fn build(self) -> Result<DetailedTargetingOption, BuildError> {
        Ok(DetailedTargetingOption {
            audience_size_lower_bound: self.audience_size_lower_bound,
            audience_size_upper_bound: self.audience_size_upper_bound,
            behavior_type: self.behavior_type,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
