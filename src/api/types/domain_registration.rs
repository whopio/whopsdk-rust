pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainRegistration {
    /// What the domain owes now. `null` when nothing is owed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<Money>,
    /// Whether Whop charges the saved payment method to renew the domain before it expires.
    #[serde(default)]
    pub auto_renew: bool,
    /// When the registration expires unless renewed, as an ISO 8601 timestamp. `null` until it's registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// How the domain came to Whop: `registration` when it was bought through Whop, `transfer` when it was moved from another registrar.
    pub method: DomainRegistrationMethod,
    /// ID of the saved card charged for the domain, prefixed `payt_`. `null` when none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Where the registration is: awaiting payment, ordering at the registrar, transferring from another registrar, delegating DNS to Whop, held, expired, or releasing.
    pub phase: DomainRegistrationPhase,
    /// Link to pay what the domain owes now: its registration or transfer, or a yearly renewal. `null` when nothing is owed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_url: Option<String>,
    /// What each yearly renewal costs at the registrar's current price. Set when the response is a single domain; `null` in lists, while the domain is being released, or when the price can't be read right now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<Money>,
    /// How far registering and keeping the domain has got.
    pub state: DomainRegistrationState,
}

impl DomainRegistration {
    pub fn builder() -> DomainRegistrationBuilder {
        <DomainRegistrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainRegistrationBuilder {
    amount_due: Option<Money>,
    auto_renew: Option<bool>,
    expires_at: Option<String>,
    method: Option<DomainRegistrationMethod>,
    payment_method_id: Option<String>,
    phase: Option<DomainRegistrationPhase>,
    purchase_url: Option<String>,
    renewal_price: Option<Money>,
    state: Option<DomainRegistrationState>,
}

impl DomainRegistrationBuilder {
    pub fn amount_due(mut self, value: Money) -> Self {
        self.amount_due = Some(value);
        self
    }

    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn method(mut self, value: DomainRegistrationMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn phase(mut self, value: DomainRegistrationPhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn purchase_url(mut self, value: impl Into<String>) -> Self {
        self.purchase_url = Some(value.into());
        self
    }

    pub fn renewal_price(mut self, value: Money) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn state(mut self, value: DomainRegistrationState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainRegistration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_renew`](DomainRegistrationBuilder::auto_renew)
    /// - [`method`](DomainRegistrationBuilder::method)
    /// - [`phase`](DomainRegistrationBuilder::phase)
    /// - [`state`](DomainRegistrationBuilder::state)
    pub fn build(self) -> Result<DomainRegistration, BuildError> {
        Ok(DomainRegistration {
            amount_due: self.amount_due,
            auto_renew: self
                .auto_renew
                .ok_or_else(|| BuildError::missing_field("auto_renew"))?,
            expires_at: self.expires_at,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            payment_method_id: self.payment_method_id,
            phase: self
                .phase
                .ok_or_else(|| BuildError::missing_field("phase"))?,
            purchase_url: self.purchase_url,
            renewal_price: self.renewal_price,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
