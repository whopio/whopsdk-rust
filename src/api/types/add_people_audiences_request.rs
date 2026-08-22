pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPeopleAudiencesRequest {
    /// The new customer CSV — a file id (`file_...`) returned by `POST /files`. Its headers must match the audience's saved column mapping.
    #[serde(default)]
    pub file_id: String,
}

impl AddPeopleAudiencesRequest {
    pub fn builder() -> AddPeopleAudiencesRequestBuilder {
        <AddPeopleAudiencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPeopleAudiencesRequestBuilder {
    file_id: Option<String>,
}

impl AddPeopleAudiencesRequestBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddPeopleAudiencesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](AddPeopleAudiencesRequestBuilder::file_id)
    pub fn build(self) -> Result<AddPeopleAudiencesRequest, BuildError> {
        Ok(AddPeopleAudiencesRequest {
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
