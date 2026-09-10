pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCashbackRulesRequest {
    /// Optional description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Exclusive end, strictly later than starts_at. Omit or set null for no expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Four-digit MCC, including leading zeros. Must match together with merchant_name.
    #[serde(default)]
    pub merchant_category_code: String,
    /// Raw merchant name reported by the card provider, not the enriched display name. Matched with the MCC; not a substring or wildcard.
    #[serde(default)]
    pub merchant_name: String,
    /// Cashback rate in basis points: 500 means 5%.
    #[serde(default)]
    pub rate_bps: i64,
    /// Account ID prefixed biz_ belonging to a direct connected account. Omit or set null to designate all direct connected accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoped_account_id: Option<String>,
    /// Inclusive start, strictly later than the current time, as an ISO 8601 timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub starts_at: DateTime<FixedOffset>,
}

impl CreateCashbackRulesRequest {
    pub fn builder() -> CreateCashbackRulesRequestBuilder {
        <CreateCashbackRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCashbackRulesRequestBuilder {
    description: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    merchant_category_code: Option<String>,
    merchant_name: Option<String>,
    rate_bps: Option<i64>,
    scoped_account_id: Option<String>,
    starts_at: Option<DateTime<FixedOffset>>,
}

impl CreateCashbackRulesRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn merchant_category_code(mut self, value: impl Into<String>) -> Self {
        self.merchant_category_code = Some(value.into());
        self
    }

    pub fn merchant_name(mut self, value: impl Into<String>) -> Self {
        self.merchant_name = Some(value.into());
        self
    }

    pub fn rate_bps(mut self, value: i64) -> Self {
        self.rate_bps = Some(value);
        self
    }

    pub fn scoped_account_id(mut self, value: impl Into<String>) -> Self {
        self.scoped_account_id = Some(value.into());
        self
    }

    pub fn starts_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.starts_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCashbackRulesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`merchant_category_code`](CreateCashbackRulesRequestBuilder::merchant_category_code)
    /// - [`merchant_name`](CreateCashbackRulesRequestBuilder::merchant_name)
    /// - [`rate_bps`](CreateCashbackRulesRequestBuilder::rate_bps)
    /// - [`starts_at`](CreateCashbackRulesRequestBuilder::starts_at)
    pub fn build(self) -> Result<CreateCashbackRulesRequest, BuildError> {
        Ok(CreateCashbackRulesRequest {
            description: self.description,
            expires_at: self.expires_at,
            merchant_category_code: self
                .merchant_category_code
                .ok_or_else(|| BuildError::missing_field("merchant_category_code"))?,
            merchant_name: self
                .merchant_name
                .ok_or_else(|| BuildError::missing_field("merchant_name"))?,
            rate_bps: self
                .rate_bps
                .ok_or_else(|| BuildError::missing_field("rate_bps"))?,
            scoped_account_id: self.scoped_account_id,
            starts_at: self
                .starts_at
                .ok_or_else(|| BuildError::missing_field("starts_at"))?,
        })
    }
}
