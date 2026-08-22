pub use crate::prelude::*;

/// A configured payout destination where a user receives earned funds, such as a bank account or digital wallet.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutMethod {
    /// A masked identifier for the payout destination, such as the last four digits of a bank account or an email address. Null if no reference is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_reference: Option<String>,
    /// The company associated with this payout destination. Null if not linked to a specific company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<PayoutMethodCompany>,
    /// The datetime the payout token was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code that payouts are delivered in for this destination.
    #[serde(default)]
    pub currency: String,
    /// The payout destination configuration linked to this token. Null if not yet configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<PayoutMethodDestination>,
    /// The unique identifier for the payout token.
    #[serde(default)]
    pub id: String,
    /// The name of the bank or financial institution receiving payouts. Null if not applicable or not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Whether this is the default payout destination for the associated payout account.
    #[serde(default)]
    pub is_default: bool,
    /// A user-defined label to help identify this payout destination. Not sent to the provider. Null if no nickname has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

impl PayoutMethod {
    pub fn builder() -> PayoutMethodBuilder {
        <PayoutMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutMethodBuilder {
    account_reference: Option<String>,
    company: Option<PayoutMethodCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    destination: Option<PayoutMethodDestination>,
    id: Option<String>,
    institution_name: Option<String>,
    is_default: Option<bool>,
    nickname: Option<String>,
}

impl PayoutMethodBuilder {
    pub fn account_reference(mut self, value: impl Into<String>) -> Self {
        self.account_reference = Some(value.into());
        self
    }

    pub fn company(mut self, value: PayoutMethodCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn destination(mut self, value: PayoutMethodDestination) -> Self {
        self.destination = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn institution_name(mut self, value: impl Into<String>) -> Self {
        self.institution_name = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PayoutMethodBuilder::created_at)
    /// - [`currency`](PayoutMethodBuilder::currency)
    /// - [`id`](PayoutMethodBuilder::id)
    /// - [`is_default`](PayoutMethodBuilder::is_default)
    pub fn build(self) -> Result<PayoutMethod, BuildError> {
        Ok(PayoutMethod {
            account_reference: self.account_reference,
            company: self.company,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            destination: self.destination,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            institution_name: self.institution_name,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            nickname: self.nickname,
        })
    }
}
