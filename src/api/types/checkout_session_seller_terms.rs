pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionSellerTerms {
    /// The seller's end-user licence agreement, or `null` if they have not published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eula_url: Option<String>,
    /// The seller's privacy policy, or `null` if they have not published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    /// Whether the buyer must explicitly accept the seller's terms before confirming. `true` means hold the confirm behind a deliberate act of acceptance; `false` means the buyer paying through a visible disclosure is the acceptance.
    #[serde(default)]
    pub required: bool,
    /// The seller's return policy, or `null` if they have not published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_policy_url: Option<String>,
    /// The seller's terms of service, or `null` if they have not published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_url: Option<String>,
}

impl CheckoutSessionSellerTerms {
    pub fn builder() -> CheckoutSessionSellerTermsBuilder {
        <CheckoutSessionSellerTermsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionSellerTermsBuilder {
    eula_url: Option<String>,
    privacy_policy_url: Option<String>,
    required: Option<bool>,
    return_policy_url: Option<String>,
    terms_url: Option<String>,
}

impl CheckoutSessionSellerTermsBuilder {
    pub fn eula_url(mut self, value: impl Into<String>) -> Self {
        self.eula_url = Some(value.into());
        self
    }

    pub fn privacy_policy_url(mut self, value: impl Into<String>) -> Self {
        self.privacy_policy_url = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn return_policy_url(mut self, value: impl Into<String>) -> Self {
        self.return_policy_url = Some(value.into());
        self
    }

    pub fn terms_url(mut self, value: impl Into<String>) -> Self {
        self.terms_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionSellerTerms`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](CheckoutSessionSellerTermsBuilder::required)
    pub fn build(self) -> Result<CheckoutSessionSellerTerms, BuildError> {
        Ok(CheckoutSessionSellerTerms {
            eula_url: self.eula_url,
            privacy_policy_url: self.privacy_policy_url,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            return_policy_url: self.return_policy_url,
            terms_url: self.terms_url,
        })
    }
}
