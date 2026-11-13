pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateNotesRequest {
    /// The note text.
    #[serde(default)]
    pub content: String,
}

impl CreateNotesRequest {
    pub fn builder() -> CreateNotesRequestBuilder {
        <CreateNotesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateNotesRequestBuilder {
    content: Option<String>,
}

impl CreateNotesRequestBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateNotesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](CreateNotesRequestBuilder::content)
    pub fn build(self) -> Result<CreateNotesRequest, BuildError> {
        Ok(CreateNotesRequest {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
