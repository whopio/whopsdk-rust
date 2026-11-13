pub use crate::prelude::*;

/// Buy the domain through Whop, or transfer in a domain you own elsewhere with `auth_code`, then renew it every year and let Whop run its DNS. Pass `null` to release an unpaid or failed purchase, or to cancel a transfer while its current registrar can still stop it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainsRequestRegistration {
    /// Transfer the domain to Whop instead of buying it. This is the transfer authorization code (also called an EPP code) from the domain's current registrar, which you usually find in its domain settings after unlocking the domain. Whop becomes the domain's registrant, and the domain moves to Whop DNS once the transfer completes, so its current DNS records, including email, stop working. After the transfer, the registry may lock the domain against another transfer for 60 days. While it's `transferring`, pass the same or a new code again to resubmit. Whop never returns the code. To take a domain from another Whop account instead, pass the `whop_dt_` code from its `transfer`: the domain moves without a payment, keeps its expiry and Whop DNS, and can't be cancelled once entered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// Whether Whop charges the saved card to renew the domain before it expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// Saved card to charge, prefixed `payt_`. It must belong to the signed-in user. Pass `null` to remove it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
}

impl CreateDomainsRequestRegistration {
    pub fn builder() -> CreateDomainsRequestRegistrationBuilder {
        <CreateDomainsRequestRegistrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainsRequestRegistrationBuilder {
    auth_code: Option<String>,
    auto_renew: Option<bool>,
    payment_method_id: Option<String>,
}

impl CreateDomainsRequestRegistrationBuilder {
    pub fn auth_code(mut self, value: impl Into<String>) -> Self {
        self.auth_code = Some(value.into());
        self
    }

    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateDomainsRequestRegistration`].
    pub fn build(self) -> Result<CreateDomainsRequestRegistration, BuildError> {
        Ok(CreateDomainsRequestRegistration {
            auth_code: self.auth_code,
            auto_renew: self.auto_renew,
            payment_method_id: self.payment_method_id,
        })
    }
}
