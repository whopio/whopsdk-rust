pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinancingTerms {
    /// The exact plain-text contract snapshot covered by this version. Present this content, the policies, and the disclosure before collecting acceptance.
    #[serde(default)]
    pub content: String,
    /// Affirmative acceptance statement to present to the merchant.
    #[serde(default)]
    pub disclosure: String,
    /// Additional financing fee information.
    #[serde(default)]
    pub fees_url: String,
    #[serde(default)]
    pub policies: Vec<String>,
    /// Source URL for the Splitit contract snapshot.
    #[serde(default)]
    pub url: String,
    /// Content fingerprint to pass as merchant_acceptance.terms_version when submitting.
    #[serde(default)]
    pub version: String,
}

impl FinancingTerms {
    pub fn builder() -> FinancingTermsBuilder {
        <FinancingTermsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancingTermsBuilder {
    content: Option<String>,
    disclosure: Option<String>,
    fees_url: Option<String>,
    policies: Option<Vec<String>>,
    url: Option<String>,
    version: Option<String>,
}

impl FinancingTermsBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn disclosure(mut self, value: impl Into<String>) -> Self {
        self.disclosure = Some(value.into());
        self
    }

    pub fn fees_url(mut self, value: impl Into<String>) -> Self {
        self.fees_url = Some(value.into());
        self
    }

    pub fn policies(mut self, value: Vec<String>) -> Self {
        self.policies = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinancingTerms`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](FinancingTermsBuilder::content)
    /// - [`disclosure`](FinancingTermsBuilder::disclosure)
    /// - [`fees_url`](FinancingTermsBuilder::fees_url)
    /// - [`policies`](FinancingTermsBuilder::policies)
    /// - [`url`](FinancingTermsBuilder::url)
    /// - [`version`](FinancingTermsBuilder::version)
    pub fn build(self) -> Result<FinancingTerms, BuildError> {
        Ok(FinancingTerms {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            disclosure: self
                .disclosure
                .ok_or_else(|| BuildError::missing_field("disclosure"))?,
            fees_url: self
                .fees_url
                .ok_or_else(|| BuildError::missing_field("fees_url"))?,
            policies: self
                .policies
                .ok_or_else(|| BuildError::missing_field("policies"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
