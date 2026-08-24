pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TooManyRequestsErrorBody {
    #[serde(default)]
    pub error: TooManyRequestsErrorBodyError,
}

impl TooManyRequestsErrorBody {
    pub fn builder() -> TooManyRequestsErrorBodyBuilder {
        <TooManyRequestsErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TooManyRequestsErrorBodyBuilder {
    error: Option<TooManyRequestsErrorBodyError>,
}

impl TooManyRequestsErrorBodyBuilder {
    pub fn error(mut self, value: TooManyRequestsErrorBodyError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TooManyRequestsErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](TooManyRequestsErrorBodyBuilder::error)
    pub fn build(self) -> Result<TooManyRequestsErrorBody, BuildError> {
        Ok(TooManyRequestsErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
