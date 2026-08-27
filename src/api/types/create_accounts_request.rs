pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAccountsRequest {
    /// The username, if any, of the partner who referred this account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    /// The blueprint App ID, prefixed `app_`. Creates a hosted website for the account and queues its deployment asynchronously; the Account response does not report deployment completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    /// The ISO 3166-1 alpha-2 country code where the account's business is located (e.g. `US`). Defaults to the parent account's country for connected accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The email address of the account owner. Required for Account API key requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Arbitrary key/value metadata to store on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The display name of the account. Defaults to `metadata.external_id` or the owner's email when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateAccountsRequest {
    pub fn builder() -> CreateAccountsRequestBuilder {
        <CreateAccountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAccountsRequestBuilder {
    affiliate_code: Option<String>,
    blueprint_id: Option<String>,
    country: Option<String>,
    email: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    title: Option<String>,
}

impl CreateAccountsRequestBuilder {
    pub fn affiliate_code(mut self, value: impl Into<String>) -> Self {
        self.affiliate_code = Some(value.into());
        self
    }

    pub fn blueprint_id(mut self, value: impl Into<String>) -> Self {
        self.blueprint_id = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAccountsRequest`].
    pub fn build(self) -> Result<CreateAccountsRequest, BuildError> {
        Ok(CreateAccountsRequest {
            affiliate_code: self.affiliate_code,
            blueprint_id: self.blueprint_id,
            country: self.country,
            email: self.email,
            metadata: self.metadata,
            title: self.title,
        })
    }
}
