pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateDomainsRequest {
    /// Accept Apple Pay at checkout on the domain. It needs no other capability. Whop registers the domain with Apple: it stays `pending` until Apple answers, then turns `ready`, or `action_required` when Apple couldn't fetch the domain association file. Serve the file, then check the domain to register it again. Pass `null` to turn Apple Pay off: Whop removes the domain from Apple, and the domain stays, like releasing any other capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<UpdateDomainsRequestApplePay>,
    /// Send and receive email on the domain. It needs `registration` or `subdomain`, so only a domain bought through Whop, or a subdomain of one in the same account, can have it. Whop publishes the DNS records and stays `pending` until its email provider verifies them, then turns `ready`. Pass `null` to remove email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<UpdateDomainsRequestEmail>,
    /// Replacement custom string keys and values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Track page views on the domain with the Whop pixel. It needs no other capability: it stays `pending` until the pixel records a page view, and turns `action_required` if it goes 7 days without one. Pass `null` to stop tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel: Option<UpdateDomainsRequestPixel>,
    /// Buy the domain through Whop, or transfer in a domain you own elsewhere with `auth_code`, then renew it every year and let Whop run its DNS. Pass `null` to release an unpaid or failed purchase, or to cancel a transfer while its current registrar can still stop it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<UpdateDomainsRequestRegistration>,
    /// Use a subdomain of a domain this account bought through Whop, such as shop.example.com under example.com. It's the default when you add such a subdomain without a capability. Whop publishes its DNS records in the root domain's zone, so it needs nothing from you: it stays `pending` until the root domain is set up, then turns `ready`. Whop releases it, and its website, when the account no longer holds the root domain, and deletes it when the root domain leaves the account. Pass `null` to release it. Can't be combined with `verification` or `registration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<UpdateDomainsRequestSubdomain>,
    /// Move a domain registered through Whop to another Whop account. Turning it on returns a code in `transfer.code`; the receiving account enters it as `registration.auth_code` on its own copy of the domain. Whop then removes the domain's email, email addresses and Apple Pay from this account, moves the registration with its expiry, stops charging this account to renew it, and removes the domain from this account. Pass `null` to turn it off and invalidate the code, until another account has entered it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<UpdateDomainsRequestTransfer>,
    /// Prove you control the domain's DNS: Whop returns a TXT record to publish in `issues`, and whoever publishes it owns the domain on Whop. Pass `null` to release it. Can't be combined with `registration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<UpdateDomainsRequestVerification>,
    /// Serve a Whop website on the domain. Pass `null` to stop serving it; the domain keeps its other capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<UpdateDomainsRequestWebsite>,
}

impl UpdateDomainsRequest {
    pub fn builder() -> UpdateDomainsRequestBuilder {
        <UpdateDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDomainsRequestBuilder {
    apple_pay: Option<UpdateDomainsRequestApplePay>,
    email: Option<UpdateDomainsRequestEmail>,
    metadata: Option<HashMap<String, String>>,
    pixel: Option<UpdateDomainsRequestPixel>,
    registration: Option<UpdateDomainsRequestRegistration>,
    subdomain: Option<UpdateDomainsRequestSubdomain>,
    transfer: Option<UpdateDomainsRequestTransfer>,
    verification: Option<UpdateDomainsRequestVerification>,
    website: Option<UpdateDomainsRequestWebsite>,
}

impl UpdateDomainsRequestBuilder {
    pub fn apple_pay(mut self, value: UpdateDomainsRequestApplePay) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn email(mut self, value: UpdateDomainsRequestEmail) -> Self {
        self.email = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn pixel(mut self, value: UpdateDomainsRequestPixel) -> Self {
        self.pixel = Some(value);
        self
    }

    pub fn registration(mut self, value: UpdateDomainsRequestRegistration) -> Self {
        self.registration = Some(value);
        self
    }

    pub fn subdomain(mut self, value: UpdateDomainsRequestSubdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn transfer(mut self, value: UpdateDomainsRequestTransfer) -> Self {
        self.transfer = Some(value);
        self
    }

    pub fn verification(mut self, value: UpdateDomainsRequestVerification) -> Self {
        self.verification = Some(value);
        self
    }

    pub fn website(mut self, value: UpdateDomainsRequestWebsite) -> Self {
        self.website = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDomainsRequest`].
    pub fn build(self) -> Result<UpdateDomainsRequest, BuildError> {
        Ok(UpdateDomainsRequest {
            apple_pay: self.apple_pay,
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
