pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForbiddenErrorBodyError {
    /// A short string indicating the specific error code, e.g. 'parameter_missing', 'parameter_invalid', 'invalid_json'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub message: String,
    /// The parameter that caused the error, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(default)]
    pub r#type: String,
}

impl ForbiddenErrorBodyError {
    pub fn builder() -> ForbiddenErrorBodyErrorBuilder {
        <ForbiddenErrorBodyErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForbiddenErrorBodyErrorBuilder {
    code: Option<String>,
    message: Option<String>,
    param: Option<String>,
    r#type: Option<String>,
}

impl ForbiddenErrorBodyErrorBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn param(mut self, value: impl Into<String>) -> Self {
        self.param = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ForbiddenErrorBodyError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ForbiddenErrorBodyErrorBuilder::message)
    /// - [`r#type`](ForbiddenErrorBodyErrorBuilder::r#type)
    pub fn build(self) -> Result<ForbiddenErrorBodyError, BuildError> {
        Ok(ForbiddenErrorBodyError {
            code: self.code,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            param: self.param,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
