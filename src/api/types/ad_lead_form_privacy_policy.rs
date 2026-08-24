pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdLeadFormPrivacyPolicy {
    /// Link text shown for the policy. `null` uses the platform default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_text: Option<String>,
    /// URL of your privacy policy.
    #[serde(default)]
    pub url: String,
}

impl AdLeadFormPrivacyPolicy {
    pub fn builder() -> AdLeadFormPrivacyPolicyBuilder {
        <AdLeadFormPrivacyPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormPrivacyPolicyBuilder {
    link_text: Option<String>,
    url: Option<String>,
}

impl AdLeadFormPrivacyPolicyBuilder {
    pub fn link_text(mut self, value: impl Into<String>) -> Self {
        self.link_text = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdLeadFormPrivacyPolicy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](AdLeadFormPrivacyPolicyBuilder::url)
    pub fn build(self) -> Result<AdLeadFormPrivacyPolicy, BuildError> {
        Ok(AdLeadFormPrivacyPolicy {
            link_text: self.link_text,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
