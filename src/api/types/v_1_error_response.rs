pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1ErrorResponse {
    #[serde(default)]
    pub error: V1ErrorResponseError,
}

impl V1ErrorResponse {
    pub fn builder() -> V1ErrorResponseBuilder {
        <V1ErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1ErrorResponseBuilder {
    error: Option<V1ErrorResponseError>,
}

impl V1ErrorResponseBuilder {
    pub fn error(mut self, value: V1ErrorResponseError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1ErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](V1ErrorResponseBuilder::error)
    pub fn build(self) -> Result<V1ErrorResponse, BuildError> {
        Ok(V1ErrorResponse {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
