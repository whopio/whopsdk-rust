pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupLastSetupError {
    /// A machine-readable classification of the failure, e.g. `enrollment_declined`. Absent when the buyer simply abandoned the setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable explanation of the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl SetupLastSetupError {
    pub fn builder() -> SetupLastSetupErrorBuilder {
        <SetupLastSetupErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupLastSetupErrorBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl SetupLastSetupErrorBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupLastSetupError`].
    pub fn build(self) -> Result<SetupLastSetupError, BuildError> {
        Ok(SetupLastSetupError {
            code: self.code,
            message: self.message,
        })
    }
}
