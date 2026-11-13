pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateDomainsRequest {
    /// Account ID, prefixed biz_. Required for user credentials; otherwise defaults to the credential's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Accept Apple Pay at checkout on the domain. It needs no other capability. Whop registers the domain with Apple: it stays `pending` until Apple answers, then turns `ready`, or `action_required` when Apple couldn't fetch the domain association file. Serve the file, then check the domain to register it again. Pass `null` to turn Apple Pay off: Whop removes the domain from Apple, and the domain stays, like releasing any other capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<CreateDomainsRequestApplePay>,
    /// Bare hostname, such as example.com or checkout.example.com. A bought domain must be a root domain. Wildcards, paths, schemes, and ports are not accepted.
    #[serde(default)]
    pub domain: String,
    /// Send and receive email on the domain. It needs `registration` or `subdomain`, so only a domain bought through Whop, or a subdomain of one in the same account, can have it. Whop publishes the DNS records and stays `pending` until its email provider verifies them, then turns `ready`. Pass `null` to remove email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<CreateDomainsRequestEmail>,
    /// Custom string keys and values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Track page views on the domain with the Whop pixel. It needs no other capability: it stays `pending` until the pixel records a page view, and turns `action_required` if it goes 7 days without one. Pass `null` to stop tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel: Option<CreateDomainsRequestPixel>,
    /// Buy the domain through Whop, or transfer in a domain you own elsewhere with `auth_code`, then renew it every year and let Whop run its DNS. Pass `null` to release an unpaid or failed purchase, or to cancel a transfer while its current registrar can still stop it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<CreateDomainsRequestRegistration>,
    /// Use a subdomain of a domain this account bought through Whop, such as shop.example.com under example.com. It's the default when you add such a subdomain without a capability. Whop publishes its DNS records in the root domain's zone, so it needs nothing from you: it stays `pending` until the root domain is set up, then turns `ready`. Whop releases it, and its website, when the account no longer holds the root domain, and deletes it when the root domain leaves the account. Pass `null` to release it. Can't be combined with `verification` or `registration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<CreateDomainsRequestSubdomain>,
    /// Move a domain registered through Whop to another Whop account. Turning it on returns a code in `transfer.code`; the receiving account enters it as `registration.auth_code` on its own copy of the domain. Whop then removes the domain's email, email addresses and Apple Pay from this account, moves the registration with its expiry, stops charging this account to renew it, and removes the domain from this account. Pass `null` to turn it off and invalidate the code, until another account has entered it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<CreateDomainsRequestTransfer>,
    /// Prove you control the domain's DNS: Whop returns a TXT record to publish in `issues`, and whoever publishes it owns the domain on Whop. Pass `null` to release it. Can't be combined with `registration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CreateDomainsRequestVerification>,
    /// Serve a Whop website on the domain. Pass `null` to stop serving it; the domain keeps its other capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<CreateDomainsRequestWebsite>,
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
    apple_pay: Option<CreateDomainsRequestApplePay>,
    domain: Option<String>,
    email: Option<CreateDomainsRequestEmail>,
    metadata: Option<HashMap<String, String>>,
    pixel: Option<CreateDomainsRequestPixel>,
    registration: Option<CreateDomainsRequestRegistration>,
    subdomain: Option<CreateDomainsRequestSubdomain>,
    transfer: Option<CreateDomainsRequestTransfer>,
    verification: Option<CreateDomainsRequestVerification>,
    website: Option<CreateDomainsRequestWebsite>,
}

impl CreateDomainsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn apple_pay(mut self, value: CreateDomainsRequestApplePay) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn email(mut self, value: CreateDomainsRequestEmail) -> Self {
        self.email = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn pixel(mut self, value: CreateDomainsRequestPixel) -> Self {
        self.pixel = Some(value);
        self
    }

    pub fn registration(mut self, value: CreateDomainsRequestRegistration) -> Self {
        self.registration = Some(value);
        self
    }

    pub fn subdomain(mut self, value: CreateDomainsRequestSubdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn transfer(mut self, value: CreateDomainsRequestTransfer) -> Self {
        self.transfer = Some(value);
        self
    }

    pub fn verification(mut self, value: CreateDomainsRequestVerification) -> Self {
        self.verification = Some(value);
        self
    }

    pub fn website(mut self, value: CreateDomainsRequestWebsite) -> Self {
        self.website = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDomainsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](CreateDomainsRequestBuilder::domain)
    pub fn build(self) -> Result<CreateDomainsRequest, BuildError> {
        Ok(CreateDomainsRequest {
            account_id: self.account_id,
            apple_pay: self.apple_pay,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            email: self.email,
            metadata: self.metadata,
            pixel: self.pixel,
            registration: self.registration,
            subdomain: self.subdomain,
            transfer: self.transfer,
            verification: self.verification,
            website: self.website,
        })
    }
}
