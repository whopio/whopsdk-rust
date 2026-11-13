pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutCashbackRulesRequest {
    /// Pay only this direct subaccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Pay only transactions whose winning cashback rule has this ID and is funded by the authenticated platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashback_rule_id: Option<String>,
    /// Pay only this card transaction belonging to a direct subaccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl PayoutCashbackRulesRequest {
    pub fn builder() -> PayoutCashbackRulesRequestBuilder {
        <PayoutCashbackRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutCashbackRulesRequestBuilder {
    account_id: Option<String>,
    cashback_rule_id: Option<String>,
    transaction_id: Option<String>,
}

impl PayoutCashbackRulesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn cashback_rule_id(mut self, value: impl Into<String>) -> Self {
        self.cashback_rule_id = Some(value.into());
        self
    }

    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutCashbackRulesRequest`].
    pub fn build(self) -> Result<PayoutCashbackRulesRequest, BuildError> {
        Ok(PayoutCashbackRulesRequest {
            account_id: self.account_id,
            cashback_rule_id: self.cashback_rule_id,
            transaction_id: self.transaction_id,
        })
    }
}
