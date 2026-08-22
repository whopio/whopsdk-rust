pub use crate::prelude::*;

/// Saved audiences to deliver to or exclude. Can't be combined with demographics.automatic.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupAudiencesBody {
    /// IDs of saved audiences to exclude from delivery, prefixed `adaud_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// IDs of saved audiences to deliver to, prefixed `adaud_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

impl AdGroupAudiencesBody {
    pub fn builder() -> AdGroupAudiencesBodyBuilder {
        <AdGroupAudiencesBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupAudiencesBodyBuilder {
    exclude: Option<Vec<String>>,
    include: Option<Vec<String>>,
}

impl AdGroupAudiencesBodyBuilder {
    pub fn exclude(mut self, value: Vec<String>) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupAudiencesBody`].
    pub fn build(self) -> Result<AdGroupAudiencesBody, BuildError> {
        Ok(AdGroupAudiencesBody {
            exclude: self.exclude,
            include: self.include,
        })
    }
}
