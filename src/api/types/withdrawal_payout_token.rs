pub use crate::prelude::*;

/// The saved payout destination used for this withdrawal (e.g., a bank account or PayPal address). Null if no payout token was used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WithdrawalPayoutToken {
    /// The datetime the payout token was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code that payouts are delivered in for this destination.
    #[serde(default)]
    pub destination_currency_code: String,
    /// The unique identifier for the payout token.
    #[serde(default)]
    pub id: String,
    /// A user-defined label to help identify this payout destination. Not sent to the provider. Null if no nickname has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The legal name of the account holder receiving payouts. Null if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
}

impl WithdrawalPayoutToken {
    pub fn builder() -> WithdrawalPayoutTokenBuilder {
        <WithdrawalPayoutTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WithdrawalPayoutTokenBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    destination_currency_code: Option<String>,
    id: Option<String>,
    nickname: Option<String>,
    payer_name: Option<String>,
}

impl WithdrawalPayoutTokenBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn destination_currency_code(mut self, value: impl Into<String>) -> Self {
        self.destination_currency_code = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn payer_name(mut self, value: impl Into<String>) -> Self {
        self.payer_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WithdrawalPayoutToken`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](WithdrawalPayoutTokenBuilder::created_at)
    /// - [`destination_currency_code`](WithdrawalPayoutTokenBuilder::destination_currency_code)
    /// - [`id`](WithdrawalPayoutTokenBuilder::id)
    pub fn build(self) -> Result<WithdrawalPayoutToken, BuildError> {
        Ok(WithdrawalPayoutToken {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            destination_currency_code: self
                .destination_currency_code
                .ok_or_else(|| BuildError::missing_field("destination_currency_code"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            nickname: self.nickname,
            payer_name: self.payer_name,
        })
    }
}
