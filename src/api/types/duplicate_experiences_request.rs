pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateExperiencesRequest {
    /// The display name for the duplicated experience. Defaults to the original experience's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DuplicateExperiencesRequest {
    pub fn builder() -> DuplicateExperiencesRequestBuilder {
        <DuplicateExperiencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateExperiencesRequestBuilder {
    name: Option<String>,
}

impl DuplicateExperiencesRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DuplicateExperiencesRequest`].
    pub fn build(self) -> Result<DuplicateExperiencesRequest, BuildError> {
        Ok(DuplicateExperiencesRequest { name: self.name })
    }
}
