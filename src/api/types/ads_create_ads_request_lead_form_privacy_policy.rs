pub use crate::prelude::*;

/// Your privacy policy. url is required by the ad platform.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestLeadFormPrivacyPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CreateAdsRequestLeadFormPrivacyPolicy {
    pub fn builder() -> CreateAdsRequestLeadFormPrivacyPolicyBuilder {
        <CreateAdsRequestLeadFormPrivacyPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestLeadFormPrivacyPolicyBuilder {
    link_text: Option<String>,
    url: Option<String>,
}

impl CreateAdsRequestLeadFormPrivacyPolicyBuilder {
    pub fn link_text(mut self, value: impl Into<String>) -> Self {
        self.link_text = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestLeadFormPrivacyPolicy`].
    pub fn build(self) -> Result<CreateAdsRequestLeadFormPrivacyPolicy, BuildError> {
        Ok(CreateAdsRequestLeadFormPrivacyPolicy {
            link_text: self.link_text,
            url: self.url,
        })
    }
}
