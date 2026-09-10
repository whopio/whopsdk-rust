pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateDomainsRequest {
    /// Account ID, prefixed biz_. Required for user credentials; otherwise defaults to the credential's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// App ID, prefixed app_. The app must belong to the account.
    #[serde(default)]
    pub app_id: String,
    /// Bare hostname, such as example.com or checkout.example.com. Wildcards, paths, schemes, and ports are not accepted.
    #[serde(default)]
    pub domain: String,
    /// Custom string keys and values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Explicitly transfer a domain from its current owner after publishing this new claim's TXT proof. Create the claim after the current owner verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<bool>,
}

impl CreateDomainsRequest {
    pub fn builder() -> CreateDomainsRequestBuilder {
        <CreateDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainsRequestBuilder {
    account_id: Option<String>,
    app_id: Option<String>,
    domain: Option<String>,
    metadata: Option<HashMap<String, String>>,
    replace_existing: Option<bool>,
}

impl CreateDomainsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn replace_existing(mut self, value: bool) -> Self {
        self.replace_existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDomainsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app_id`](CreateDomainsRequestBuilder::app_id)
    /// - [`domain`](CreateDomainsRequestBuilder::domain)
    pub fn build(self) -> Result<CreateDomainsRequest, BuildError> {
        Ok(CreateDomainsRequest {
            account_id: self.account_id,
            app_id: self
                .app_id
                .ok_or_else(|| BuildError::missing_field("app_id"))?,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            metadata: self.metadata,
            replace_existing: self.replace_existing,
        })
    }
}
