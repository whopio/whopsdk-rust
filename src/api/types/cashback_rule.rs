pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CashbackRule {
    /// When the rule was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Optional description of the cashback rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the rule was discarded, as an ISO 8601 timestamp. Null means it has not been discarded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discarded_at: Option<String>,
    /// Exclusive end of the eligibility window, as an ISO 8601 timestamp. Null means no expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Platform account designated to fund cashback, prefixed `biz_`. Derived from the authenticated credential.
    #[serde(default)]
    pub funding_account_id: String,
    /// Cashback rule ID, prefixed `cicbr_`.
    #[serde(default)]
    pub id: String,
    /// Four-digit merchant category code. Both merchant filters must match.
    #[serde(default)]
    pub merchant_category_code: String,
    /// Raw merchant name reported by the card provider. Matched together with the merchant category code; not a substring or enriched display-name match.
    #[serde(default)]
    pub merchant_name: String,
    /// Cashback rate in basis points. 100 means 1%, and 10000 means 100%.
    #[serde(default)]
    pub rate_bps: i64,
    /// Connected account ID, prefixed `biz_`. Null designates all direct connected accounts of the funding platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoped_account_id: Option<String>,
    /// Inclusive start of the rule's eligibility window, as an ISO 8601 timestamp.
    #[serde(default)]
    pub starts_at: String,
    /// When the rule was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl CashbackRule {
    pub fn builder() -> CashbackRuleBuilder {
        <CashbackRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CashbackRuleBuilder {
    created_at: Option<String>,
    description: Option<String>,
    discarded_at: Option<String>,
    expires_at: Option<String>,
    funding_account_id: Option<String>,
    id: Option<String>,
    merchant_category_code: Option<String>,
    merchant_name: Option<String>,
    rate_bps: Option<i64>,
    scoped_account_id: Option<String>,
    starts_at: Option<String>,
    updated_at: Option<String>,
}

impl CashbackRuleBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn discarded_at(mut self, value: impl Into<String>) -> Self {
        self.discarded_at = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn funding_account_id(mut self, value: impl Into<String>) -> Self {
        self.funding_account_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn starts_at(mut self, value: impl Into<String>) -> Self {
        self.starts_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CashbackRule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CashbackRuleBuilder::created_at)
    /// - [`funding_account_id`](CashbackRuleBuilder::funding_account_id)
    /// - [`id`](CashbackRuleBuilder::id)
    /// - [`merchant_category_code`](CashbackRuleBuilder::merchant_category_code)
    /// - [`merchant_name`](CashbackRuleBuilder::merchant_name)
    /// - [`rate_bps`](CashbackRuleBuilder::rate_bps)
    /// - [`starts_at`](CashbackRuleBuilder::starts_at)
    /// - [`updated_at`](CashbackRuleBuilder::updated_at)
    pub fn build(self) -> Result<CashbackRule, BuildError> {
        Ok(CashbackRule {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            discarded_at: self.discarded_at,
            expires_at: self.expires_at,
            funding_account_id: self
                .funding_account_id
                .ok_or_else(|| BuildError::missing_field("funding_account_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
