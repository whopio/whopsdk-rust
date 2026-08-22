pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnprocessableEntityErrorBody {
    #[serde(default)]
    pub error: UnprocessableEntityErrorBodyError,
}

impl UnprocessableEntityErrorBody {
    pub fn builder() -> UnprocessableEntityErrorBodyBuilder {
        <UnprocessableEntityErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnprocessableEntityErrorBodyBuilder {
    error: Option<UnprocessableEntityErrorBodyError>,
}

impl UnprocessableEntityErrorBodyBuilder {
    pub fn error(mut self, value: UnprocessableEntityErrorBodyError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnprocessableEntityErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](UnprocessableEntityErrorBodyBuilder::error)
    pub fn build(self) -> Result<UnprocessableEntityErrorBody, BuildError> {
        Ok(UnprocessableEntityErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
