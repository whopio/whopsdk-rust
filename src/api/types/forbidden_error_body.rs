pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForbiddenErrorBody {
    #[serde(default)]
    pub error: ForbiddenErrorBodyError,
}

impl ForbiddenErrorBody {
    pub fn builder() -> ForbiddenErrorBodyBuilder {
        <ForbiddenErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForbiddenErrorBodyBuilder {
    error: Option<ForbiddenErrorBodyError>,
}

impl ForbiddenErrorBodyBuilder {
    pub fn error(mut self, value: ForbiddenErrorBodyError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForbiddenErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](ForbiddenErrorBodyBuilder::error)
    pub fn build(self) -> Result<ForbiddenErrorBody, BuildError> {
        Ok(ForbiddenErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
