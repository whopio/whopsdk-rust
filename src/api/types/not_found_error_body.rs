pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotFoundErrorBody {
    #[serde(default)]
    pub error: NotFoundErrorBodyError,
}

impl NotFoundErrorBody {
    pub fn builder() -> NotFoundErrorBodyBuilder {
        <NotFoundErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotFoundErrorBodyBuilder {
    error: Option<NotFoundErrorBodyError>,
}

impl NotFoundErrorBodyBuilder {
    pub fn error(mut self, value: NotFoundErrorBodyError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotFoundErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](NotFoundErrorBodyBuilder::error)
    pub fn build(self) -> Result<NotFoundErrorBody, BuildError> {
        Ok(NotFoundErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
