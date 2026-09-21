pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdConversionValueRuleTarget {
    /// Ad platform that receives the adjusted value.
    pub platform: AdConversionValueRuleTargetPlatform,
    /// Selected campaign, ad group, or ad ID. Null for a business target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Name of the selected item. Null when it is no longer available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_title: Option<String>,
    /// Business, campaign, ad group, or ad covered by this target.
    pub scope: AdConversionValueRuleTargetScope,
}

impl AdConversionValueRuleTarget {
    pub fn builder() -> AdConversionValueRuleTargetBuilder {
        <AdConversionValueRuleTargetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdConversionValueRuleTargetBuilder {
    platform: Option<AdConversionValueRuleTargetPlatform>,
    resource_id: Option<String>,
    resource_title: Option<String>,
    scope: Option<AdConversionValueRuleTargetScope>,
}

impl AdConversionValueRuleTargetBuilder {
    pub fn platform(mut self, value: AdConversionValueRuleTargetPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn resource_title(mut self, value: impl Into<String>) -> Self {
        self.resource_title = Some(value.into());
        self
    }

    pub fn scope(mut self, value: AdConversionValueRuleTargetScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdConversionValueRuleTarget`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](AdConversionValueRuleTargetBuilder::platform)
    /// - [`scope`](AdConversionValueRuleTargetBuilder::scope)
    pub fn build(self) -> Result<AdConversionValueRuleTarget, BuildError> {
        Ok(AdConversionValueRuleTarget {
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            resource_id: self.resource_id,
            resource_title: self.resource_title,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
