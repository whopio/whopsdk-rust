pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainIssue {
    /// The source of the setup issue.
    #[serde(default)]
    pub code: String,
    /// What needs attention before the domain can serve the website.
    #[serde(default)]
    pub message: String,
}

impl DomainIssue {
    pub fn builder() -> DomainIssueBuilder {
        <DomainIssueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainIssueBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl DomainIssueBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainIssue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](DomainIssueBuilder::code)
    /// - [`message`](DomainIssueBuilder::message)
    pub fn build(self) -> Result<DomainIssue, BuildError> {
        Ok(DomainIssue {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
