pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainAvailability {
    /// Whether the domain can be registered now.
    #[serde(default)]
    pub available: bool,
    /// Whether the registry charges more than its standard price for this domain.
    #[serde(default)]
    pub premium: bool,
    /// What the first year of registration costs. `null` when the domain is not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Money>,
    /// What each yearly renewal costs after the first year. `null` when the domain is not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<Money>,
    /// How desirable the domain is, from 0 to 100. Short, real-word names on well-known extensions score highest.
    #[serde(default)]
    pub score: i64,
    /// What moving the domain to Whop costs if it's registered elsewhere, including the year it adds to the domain's registration. `null` when it's available or can't be transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_price: Option<Money>,
}

impl DomainAvailability {
    pub fn builder() -> DomainAvailabilityBuilder {
        <DomainAvailabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainAvailabilityBuilder {
    available: Option<bool>,
    premium: Option<bool>,
    price: Option<Money>,
    renewal_price: Option<Money>,
    score: Option<i64>,
    transfer_price: Option<Money>,
}

impl DomainAvailabilityBuilder {
    pub fn available(mut self, value: bool) -> Self {
        self.available = Some(value);
        self
    }

    pub fn premium(mut self, value: bool) -> Self {
        self.premium = Some(value);
        self
    }

    pub fn price(mut self, value: Money) -> Self {
        self.price = Some(value);
        self
    }

    pub fn renewal_price(mut self, value: Money) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn score(mut self, value: i64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn transfer_price(mut self, value: Money) -> Self {
        self.transfer_price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainAvailability`].
    /// This method will fail if any of the following fields are not set:
    /// - [`available`](DomainAvailabilityBuilder::available)
    /// - [`premium`](DomainAvailabilityBuilder::premium)
    /// - [`score`](DomainAvailabilityBuilder::score)
    pub fn build(self) -> Result<DomainAvailability, BuildError> {
        Ok(DomainAvailability {
            available: self
                .available
                .ok_or_else(|| BuildError::missing_field("available"))?,
            premium: self
                .premium
                .ok_or_else(|| BuildError::missing_field("premium"))?,
            price: self.price,
            renewal_price: self.renewal_price,
            score: self
                .score
                .ok_or_else(|| BuildError::missing_field("score"))?,
            transfer_price: self.transfer_price,
        })
    }
}
