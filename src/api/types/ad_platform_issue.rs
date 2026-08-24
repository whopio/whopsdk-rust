pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdPlatformIssue {
    /// Unique identifier for the issue.
    #[serde(default)]
    pub id: String,
    /// A description of what the issue is and how it can be resolved.
    #[serde(default)]
    pub message: String,
    /// The ID of the campaign, ad group, or ad the issue is attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// The type of resource the issue is attached to.
    pub resource_type: AdPlatformIssueResourceType,
}

impl AdPlatformIssue {
    pub fn builder() -> AdPlatformIssueBuilder {
        <AdPlatformIssueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdPlatformIssueBuilder {
    id: Option<String>,
    message: Option<String>,
    resource_id: Option<String>,
    resource_type: Option<AdPlatformIssueResourceType>,
}

impl AdPlatformIssueBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn resource_type(mut self, value: AdPlatformIssueResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdPlatformIssue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdPlatformIssueBuilder::id)
    /// - [`message`](AdPlatformIssueBuilder::message)
    /// - [`resource_type`](AdPlatformIssueBuilder::resource_type)
    pub fn build(self) -> Result<AdPlatformIssue, BuildError> {
        Ok(AdPlatformIssue {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            resource_id: self.resource_id,
            resource_type: self
                .resource_type
                .ok_or_else(|| BuildError::missing_field("resource_type"))?,
        })
    }
}
