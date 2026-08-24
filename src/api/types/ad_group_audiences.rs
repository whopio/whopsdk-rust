pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupAudiences {
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub include: Vec<String>,
}

impl AdGroupAudiences {
    pub fn builder() -> AdGroupAudiencesBuilder {
        <AdGroupAudiencesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupAudiencesBuilder {
    exclude: Option<Vec<String>>,
    include: Option<Vec<String>>,
}

impl AdGroupAudiencesBuilder {
    pub fn exclude(mut self, value: Vec<String>) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupAudiences`].
    /// This method will fail if any of the following fields are not set:
    /// - [`exclude`](AdGroupAudiencesBuilder::exclude)
    /// - [`include`](AdGroupAudiencesBuilder::include)
    pub fn build(self) -> Result<AdGroupAudiences, BuildError> {
        Ok(AdGroupAudiences {
            exclude: self
                .exclude
                .ok_or_else(|| BuildError::missing_field("exclude"))?,
            include: self
                .include
                .ok_or_else(|| BuildError::missing_field("include"))?,
        })
    }
}
