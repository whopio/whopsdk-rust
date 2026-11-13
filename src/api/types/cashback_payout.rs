pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CashbackPayout {
    /// Subaccount filter from the request, prefixed `biz_`. Omitted when not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Cashback rule filter from the request, prefixed `cicbr_`. Omitted when not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashback_rule_id: Option<String>,
    /// Request status. `processing` means background processing was queued, not that payment completed. `failed`, returned with HTTP `200`, means the queue rejected the request. Subsequent transaction failures retry automatically.
    pub status: CashbackPayoutStatus,
    /// Card transaction filter from the request, prefixed `citx_`. Omitted when not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl CashbackPayout {
    pub fn builder() -> CashbackPayoutBuilder {
        <CashbackPayoutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CashbackPayoutBuilder {
    account_id: Option<String>,
    cashback_rule_id: Option<String>,
    status: Option<CashbackPayoutStatus>,
    transaction_id: Option<String>,
}

impl CashbackPayoutBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn cashback_rule_id(mut self, value: impl Into<String>) -> Self {
        self.cashback_rule_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CashbackPayoutStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CashbackPayout`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](CashbackPayoutBuilder::status)
    pub fn build(self) -> Result<CashbackPayout, BuildError> {
        Ok(CashbackPayout {
            account_id: self.account_id,
            cashback_rule_id: self.cashback_rule_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            transaction_id: self.transaction_id,
        })
    }
}
