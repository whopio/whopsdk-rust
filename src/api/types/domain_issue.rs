pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainIssue {
    /// The capability the issue is about.
    pub capability: DomainIssueCapability,
    /// What needs attention, as a stable code.
    pub code: DomainIssueCode,
    /// Code to add to the site to fix the issue, ready to copy. `null` when the issue needs none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block: Option<String>,
    #[serde(default)]
    pub dns_records: Vec<DomainDnsRecord>,
    /// What needs attention and how to fix it.
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
    capability: Option<DomainIssueCapability>,
    code: Option<DomainIssueCode>,
    code_block: Option<String>,
    dns_records: Option<Vec<DomainDnsRecord>>,
    message: Option<String>,
}

impl DomainIssueBuilder {
    pub fn capability(mut self, value: DomainIssueCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn code(mut self, value: DomainIssueCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn code_block(mut self, value: impl Into<String>) -> Self {
        self.code_block = Some(value.into());
        self
    }

    pub fn dns_records(mut self, value: Vec<DomainDnsRecord>) -> Self {
        self.dns_records = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainIssue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capability`](DomainIssueBuilder::capability)
    /// - [`code`](DomainIssueBuilder::code)
    /// - [`dns_records`](DomainIssueBuilder::dns_records)
    /// - [`message`](DomainIssueBuilder::message)
    pub fn build(self) -> Result<DomainIssue, BuildError> {
        Ok(DomainIssue {
            capability: self
                .capability
                .ok_or_else(|| BuildError::missing_field("capability"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            code_block: self.code_block,
            dns_records: self
                .dns_records
                .ok_or_else(|| BuildError::missing_field("dns_records"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
