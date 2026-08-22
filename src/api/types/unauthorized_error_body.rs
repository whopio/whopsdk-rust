pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnauthorizedErrorBody {
    #[serde(default)]
    pub error: UnauthorizedErrorBodyError,
}

impl UnauthorizedErrorBody {
    pub fn builder() -> UnauthorizedErrorBodyBuilder {
        <UnauthorizedErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnauthorizedErrorBodyBuilder {
    error: Option<UnauthorizedErrorBodyError>,
}

impl UnauthorizedErrorBodyBuilder {
    pub fn error(mut self, value: UnauthorizedErrorBodyError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnauthorizedErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](UnauthorizedErrorBodyBuilder::error)
    pub fn build(self) -> Result<UnauthorizedErrorBody, BuildError> {
        Ok(UnauthorizedErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
