pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPeopleResponseDataItemLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl ListPeopleResponseDataItemLocation {
    pub fn builder() -> ListPeopleResponseDataItemLocationBuilder {
        <ListPeopleResponseDataItemLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseDataItemLocationBuilder {
    city: Option<String>,
    continent: Option<String>,
    country: Option<String>,
}

impl ListPeopleResponseDataItemLocationBuilder {
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

    /// Consumes the builder and constructs a [`ListPeopleResponseDataItemLocation`].
    pub fn build(self) -> Result<ListPeopleResponseDataItemLocation, BuildError> {
        Ok(ListPeopleResponseDataItemLocation {
            city: self.city,
            continent: self.continent,
            country: self.country,
        })
    }
}
