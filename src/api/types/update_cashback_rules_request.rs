pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCashbackRulesRequest {
    /// Description of the rule. Set null to clear it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Exclusive end as an ISO 8601 timestamp, strictly later than the original starts_at. May be in the past to end an active rule. Set null to remove the expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Four-digit MCC, including leading zeros. Must match together with merchant_name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    /// Raw merchant name reported by the card provider. Must contain a non-whitespace character. Matched with the MCC; not a substring or wildcard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
}

impl UpdateCashbackRulesRequest {
    pub fn builder() -> UpdateCashbackRulesRequestBuilder {
        <UpdateCashbackRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCashbackRulesRequestBuilder {
    description: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    merchant_category_code: Option<String>,
    merchant_name: Option<String>,
}

impl UpdateCashbackRulesRequestBuilder {
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

    /// Consumes the builder and constructs a [`UpdateCashbackRulesRequest`].
    pub fn build(self) -> Result<UpdateCashbackRulesRequest, BuildError> {
        Ok(UpdateCashbackRulesRequest {
            description: self.description,
            expires_at: self.expires_at,
            merchant_category_code: self.merchant_category_code,
            merchant_name: self.merchant_name,
        })
    }
}
