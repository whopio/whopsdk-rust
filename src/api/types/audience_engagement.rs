pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudienceEngagement {
    /// Exclude anyone matching any exclusion rule. Supply 0–10 rules. Video audiences do not support exclusions; use a separate audience in ad-group exclusions.
    #[serde(default)]
    pub exclude: Vec<AudienceEngagementRule>,
    /// Match any inclusion rule. Supply 1–10 rules. Video rules must share a retention window and cannot be combined with other sources.
    #[serde(default)]
    pub include: Vec<AudienceEngagementRule>,
    /// Ad platform that maintains membership.
    pub platform: AudienceEngagementPlatform,
}

impl AudienceEngagement {
    pub fn builder() -> AudienceEngagementBuilder {
        <AudienceEngagementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudienceEngagementBuilder {
    exclude: Option<Vec<AudienceEngagementRule>>,
    include: Option<Vec<AudienceEngagementRule>>,
    platform: Option<AudienceEngagementPlatform>,
}

impl AudienceEngagementBuilder {
    pub fn exclude(mut self, value: Vec<AudienceEngagementRule>) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn include(mut self, value: Vec<AudienceEngagementRule>) -> Self {
        self.include = Some(value);
        self
    }

    pub fn platform(mut self, value: AudienceEngagementPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudienceEngagement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`exclude`](AudienceEngagementBuilder::exclude)
    /// - [`include`](AudienceEngagementBuilder::include)
    /// - [`platform`](AudienceEngagementBuilder::platform)
    pub fn build(self) -> Result<AudienceEngagement, BuildError> {
        Ok(AudienceEngagement {
            exclude: self
                .exclude
                .ok_or_else(|| BuildError::missing_field("exclude"))?,
            include: self
                .include
                .ok_or_else(|| BuildError::missing_field("include"))?,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
        })
    }
}
