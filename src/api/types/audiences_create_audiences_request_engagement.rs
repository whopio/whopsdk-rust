pub use crate::prelude::*;

/// Rules for membership based on social engagement. Requires a connected social account with advertising access.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAudiencesRequestEngagement {
    /// Exclude anyone matching any exclusion rule. Defaults to an empty array. Video audiences do not support exclusions; use a separate audience in ad-group exclusions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<AudienceEngagementRule>>,
    /// Match any inclusion rule. Video rules must share a retention window and cannot be combined with other sources.
    #[serde(default)]
    pub include: Vec<AudienceEngagementRule>,
    /// Ad platform that maintains membership.
    pub platform: CreateAudiencesRequestEngagementPlatform,
}

impl CreateAudiencesRequestEngagement {
    pub fn builder() -> CreateAudiencesRequestEngagementBuilder {
        <CreateAudiencesRequestEngagementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudiencesRequestEngagementBuilder {
    exclude: Option<Vec<AudienceEngagementRule>>,
    include: Option<Vec<AudienceEngagementRule>>,
    platform: Option<CreateAudiencesRequestEngagementPlatform>,
}

impl CreateAudiencesRequestEngagementBuilder {
    pub fn exclude(mut self, value: Vec<AudienceEngagementRule>) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn include(mut self, value: Vec<AudienceEngagementRule>) -> Self {
        self.include = Some(value);
        self
    }

    pub fn platform(mut self, value: CreateAudiencesRequestEngagementPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAudiencesRequestEngagement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`include`](CreateAudiencesRequestEngagementBuilder::include)
    /// - [`platform`](CreateAudiencesRequestEngagementBuilder::platform)
    pub fn build(self) -> Result<CreateAudiencesRequestEngagement, BuildError> {
        Ok(CreateAudiencesRequestEngagement {
            exclude: self.exclude,
            include: self
                .include
                .ok_or_else(|| BuildError::missing_field("include"))?,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
        })
    }
}
