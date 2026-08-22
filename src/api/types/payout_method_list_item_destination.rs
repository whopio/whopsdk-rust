pub use crate::prelude::*;

/// The payout destination configuration linked to this token. Null if not yet configured.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayoutMethodListItemDestination {
    /// The category of the payout destination
    pub category: PayoutDestinationCategory,
    /// The country code of the payout destination
    #[serde(default)]
    pub country_code: String,
    /// The name of the payer associated with the payout destination
    #[serde(default)]
    pub name: String,
}

impl PayoutMethodListItemDestination {
    pub fn builder() -> PayoutMethodListItemDestinationBuilder {
        <PayoutMethodListItemDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutMethodListItemDestinationBuilder {
    category: Option<PayoutDestinationCategory>,
    country_code: Option<String>,
    name: Option<String>,
}

impl PayoutMethodListItemDestinationBuilder {
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

    /// Consumes the builder and constructs a [`PayoutMethodListItemDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](PayoutMethodListItemDestinationBuilder::category)
    /// - [`country_code`](PayoutMethodListItemDestinationBuilder::country_code)
    /// - [`name`](PayoutMethodListItemDestinationBuilder::name)
    pub fn build(self) -> Result<PayoutMethodListItemDestination, BuildError> {
        Ok(PayoutMethodListItemDestination {
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
