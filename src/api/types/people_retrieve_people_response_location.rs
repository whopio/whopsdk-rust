pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePeopleResponseLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl RetrievePeopleResponseLocation {
    pub fn builder() -> RetrievePeopleResponseLocationBuilder {
        <RetrievePeopleResponseLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseLocationBuilder {
    city: Option<String>,
    continent: Option<String>,
    country: Option<String>,
}

impl RetrievePeopleResponseLocationBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn continent(mut self, value: impl Into<String>) -> Self {
        self.continent = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseLocation`].
    pub fn build(self) -> Result<RetrievePeopleResponseLocation, BuildError> {
        Ok(RetrievePeopleResponseLocation {
            city: self.city,
            continent: self.continent,
            country: self.country,
        })
    }
}
