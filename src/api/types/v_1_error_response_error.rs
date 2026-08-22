pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1ErrorResponseError {
    /// Machine-readable reason for this specific refusal, such as `bank_warning_not_acknowledged`. Only present when the error carries one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Human-readable error message.
    #[serde(default)]
    pub message: String,
    /// Machine-readable error code.
    #[serde(default)]
    pub r#type: String,
}

impl V1ErrorResponseError {
    pub fn builder() -> V1ErrorResponseErrorBuilder {
        <V1ErrorResponseErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1ErrorResponseErrorBuilder {
    code: Option<String>,
    message: Option<String>,
    r#type: Option<String>,
}

impl V1ErrorResponseErrorBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1ErrorResponseError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](V1ErrorResponseErrorBuilder::message)
    /// - [`r#type`](V1ErrorResponseErrorBuilder::r#type)
    pub fn build(self) -> Result<V1ErrorResponseError, BuildError> {
        Ok(V1ErrorResponseError {
            code: self.code,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
