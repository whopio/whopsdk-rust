pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdPlatformIssue {
    /// The kind of issue: information about delivery, a warning, or an error requiring attention.
    pub category: AdPlatformIssueCategory,
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
    /// A short, creator-facing title for the issue: the rule a content-review rejection names, or a generic label.
    #[serde(default)]
    pub title: String,
}

impl AdPlatformIssue {
    pub fn builder() -> AdPlatformIssueBuilder {
        <AdPlatformIssueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdPlatformIssueBuilder {
    category: Option<AdPlatformIssueCategory>,
    id: Option<String>,
    message: Option<String>,
    resource_id: Option<String>,
    resource_type: Option<AdPlatformIssueResourceType>,
    title: Option<String>,
}

impl AdPlatformIssueBuilder {
    pub fn category(mut self, value: AdPlatformIssueCategory) -> Self {
        self.category = Some(value);
        self
    }

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

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdPlatformIssue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](AdPlatformIssueBuilder::category)
    /// - [`id`](AdPlatformIssueBuilder::id)
    /// - [`message`](AdPlatformIssueBuilder::message)
    /// - [`resource_type`](AdPlatformIssueBuilder::resource_type)
    /// - [`title`](AdPlatformIssueBuilder::title)
    pub fn build(self) -> Result<AdPlatformIssue, BuildError> {
        Ok(AdPlatformIssue {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            resource_id: self.resource_id,
            resource_type: self
                .resource_type
                .ok_or_else(|| BuildError::missing_field("resource_type"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
