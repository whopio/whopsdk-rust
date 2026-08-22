pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupCity {
    /// The ad platform's key for the city in its location taxonomy.
    #[serde(default)]
    pub key: String,
    /// City name, such as `Austin`. Absent when the platform doesn't return one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AdGroupCity {
    pub fn builder() -> AdGroupCityBuilder {
        <AdGroupCityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupCityBuilder {
    key: Option<String>,
    name: Option<String>,
}

impl AdGroupCityBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdGroupCity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](AdGroupCityBuilder::key)
    pub fn build(self) -> Result<AdGroupCity, BuildError> {
        Ok(AdGroupCity {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            name: self.name,
        })
    }
}
