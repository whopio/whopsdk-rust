pub use crate::prelude::*;

/// The payout destination configuration linked to this token. Null if not yet configured.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayoutMethodDestination {
    /// The category of the payout destination
    pub category: PayoutDestinationCategory,
    /// The country code of the payout destination
    #[serde(default)]
    pub country_code: String,
    /// The name of the payer associated with the payout destination
    #[serde(default)]
    pub name: String,
}

impl PayoutMethodDestination {
    pub fn builder() -> PayoutMethodDestinationBuilder {
        <PayoutMethodDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutMethodDestinationBuilder {
    category: Option<PayoutDestinationCategory>,
    country_code: Option<String>,
    name: Option<String>,
}

impl PayoutMethodDestinationBuilder {
    pub fn category(mut self, value: PayoutDestinationCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutMethodDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](PayoutMethodDestinationBuilder::category)
    /// - [`country_code`](PayoutMethodDestinationBuilder::country_code)
    /// - [`name`](PayoutMethodDestinationBuilder::name)
    pub fn build(self) -> Result<PayoutMethodDestination, BuildError> {
        Ok(PayoutMethodDestination {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
