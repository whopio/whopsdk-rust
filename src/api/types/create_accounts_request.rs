pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAccountsRequest {
    /// A saved partner referral link code for this new business account. An existing primary user referral takes priority. Used with user tokens creating top-level accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    /// The blueprint App ID, prefixed `app_`. Creates a hosted website for the account and queues its deployment asynchronously; the Account response does not report deployment completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    /// The ISO 3166-1 alpha-2 country code where the account's business is located (e.g. `US`). Defaults to the parent account's country for subaccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Whether Whop assembles and files dispute evidence for this account. Enabling it opts into the success fee charged on disputes it wins. Requires payment:dispute. Omit to preserve the existing setting or creation default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_fighter_enabled: Option<bool>,
    /// The email address of the account owner. Required when creating a subaccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Your platform's own identifier for this subaccount, such as your user or store ID. Unique among your subaccounts: creating a second account with the same `external_id` and a different `email` returns a 409, unless `reassign_external_id` is `true`. Retrying with the same `email` and `external_id` returns the existing account. Subaccounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Arbitrary key/value metadata to store on the account. When `external_id` is not sent, an `external_id` key here sets `external_id` and follows its rules. It must be unique among your subaccounts unless `reassign_external_id` is `true`. It stays in `metadata` as sent. Changing it later does not change `external_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Whether payment orchestration is enabled for this account. Requires payout:account:update. Omit to preserve the existing setting or creation default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration_enabled: Option<bool>,
    /// Pass `true` to move `external_id` to the new subaccount when another of your subaccounts already has it. Without it, that request returns a 409. The subaccount that had it is not deleted. It stays active with `external_id` set to `null`, and its `metadata` is unchanged. Requires `external_id`. Also requires the `company:update` permission on your platform account, in addition to `company:create_child`. Ignored when the same `email` already has the `external_id`, which returns that account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reassign_external_id: Option<bool>,
    /// Whether Whop sends transactional emails to customers on behalf of the subaccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_customer_emails: Option<bool>,
    /// The display name of the account. Defaults to `external_id` or the owner's email when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The account's business website, as an `http` or `https` URL of at most 255 characters. Also added to the account's `social_links` as a `website` entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
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
    dispute_fighter_enabled: Option<bool>,
    email: Option<String>,
    external_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    orchestration_enabled: Option<bool>,
    reassign_external_id: Option<bool>,
    send_customer_emails: Option<bool>,
    title: Option<String>,
    website: Option<String>,
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

    pub fn dispute_fighter_enabled(mut self, value: bool) -> Self {
        self.dispute_fighter_enabled = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn orchestration_enabled(mut self, value: bool) -> Self {
        self.orchestration_enabled = Some(value);
        self
    }

    pub fn reassign_external_id(mut self, value: bool) -> Self {
        self.reassign_external_id = Some(value);
        self
    }

    pub fn send_customer_emails(mut self, value: bool) -> Self {
        self.send_customer_emails = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAccountsRequest`].
    pub fn build(self) -> Result<CreateAccountsRequest, BuildError> {
        Ok(CreateAccountsRequest {
            affiliate_code: self.affiliate_code,
            blueprint_id: self.blueprint_id,
            country: self.country,
            dispute_fighter_enabled: self.dispute_fighter_enabled,
            email: self.email,
            external_id: self.external_id,
            metadata: self.metadata,
            orchestration_enabled: self.orchestration_enabled,
            reassign_external_id: self.reassign_external_id,
            send_customer_emails: self.send_customer_emails,
            title: self.title,
            website: self.website,
        })
    }
}
