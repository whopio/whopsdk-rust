pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InternalServerErrorBody {
    #[serde(default)]
    pub error: InternalServerErrorBodyError,
}

impl InternalServerErrorBody {
    pub fn builder() -> InternalServerErrorBodyBuilder {
        <InternalServerErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InternalServerErrorBodyBuilder {
    error: Option<InternalServerErrorBodyError>,
}

impl InternalServerErrorBodyBuilder {
    pub fn error(mut self, value: InternalServerErrorBodyError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InternalServerErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](InternalServerErrorBodyBuilder::error)
    pub fn build(self) -> Result<InternalServerErrorBody, BuildError> {
        Ok(InternalServerErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
