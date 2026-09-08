pub use crate::prelude::*;

/// The billing address. On an issued card this replaces the card's billing address and region is also required. On an invited card, sending it as the invited user completes onboarding and starts card provisioning.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCardsRequestBilling {
    /// Billing city.
    #[serde(default)]
    pub city: String,
    /// Billing country code, ISO 3166-1 alpha-2.
    #[serde(default)]
    pub country_code: String,
    /// Street address line 1.
    #[serde(default)]
    pub line1: String,
    /// Street address line 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Billing postal code.
    #[serde(default)]
    pub postal_code: String,
    /// Billing region or state. Required when updating an issued card's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl UpdateCardsRequestBilling {
    pub fn builder() -> UpdateCardsRequestBillingBuilder {
        <UpdateCardsRequestBillingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCardsRequestBillingBuilder {
    city: Option<String>,
    country_code: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    region: Option<String>,
}

impl UpdateCardsRequestBillingBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = Some(value.into());
        self
    }

    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCardsRequestBilling`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](UpdateCardsRequestBillingBuilder::city)
    /// - [`country_code`](UpdateCardsRequestBillingBuilder::country_code)
    /// - [`line1`](UpdateCardsRequestBillingBuilder::line1)
    /// - [`postal_code`](UpdateCardsRequestBillingBuilder::postal_code)
    pub fn build(self) -> Result<UpdateCardsRequestBilling, BuildError> {
        Ok(UpdateCardsRequestBilling {
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self.line2,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            region: self.region,
        })
    }
}
