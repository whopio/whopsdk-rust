pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BadRequestErrorBody {
    #[serde(default)]
    pub error: BadRequestErrorBodyError,
}

impl BadRequestErrorBody {
    pub fn builder() -> BadRequestErrorBodyBuilder {
        <BadRequestErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadRequestErrorBodyBuilder {
    error: Option<BadRequestErrorBodyError>,
}

impl BadRequestErrorBodyBuilder {
    pub fn error(mut self, value: BadRequestErrorBodyError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BadRequestErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](BadRequestErrorBodyBuilder::error)
    pub fn build(self) -> Result<BadRequestErrorBody, BuildError> {
        Ok(BadRequestErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
