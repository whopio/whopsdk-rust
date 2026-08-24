pub use crate::prelude::*;

/// The account whose wallet this is. Null for the buyer's own personal wallet. A buyer sees an account's balance here when they hold permission to spend it, so a list can hold several — their own and one per account they are on.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccount {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The company's logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogo>,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccount {
    pub fn builder() -> PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountBuilder {
        <PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountBuilder {
    id: Option<String>,
    logo: Option<PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogo>,
    title: Option<String>,
}

impl PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo(
        mut self,
        value: PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogo,
    ) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountBuilder::id)
    /// - [`title`](PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountBuilder::title)
    pub fn build(
        self,
    ) -> Result<PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccount, BuildError> {
        Ok(
            PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccount {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                logo: self.logo,
                title: self
                    .title
                    .ok_or_else(|| BuildError::missing_field("title"))?,
            },
        )
    }
}
