pub use crate::prelude::*;

/// The saved payout method used. Requires payout:destination:read; null without it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelPayoutsResponsePayoutMethod {
    /// Saved payout method nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Supported payout method display details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method: Option<CancelPayoutsResponsePayoutMethodSupportedPayoutMethod>,
}

impl CancelPayoutsResponsePayoutMethod {
    pub fn builder() -> CancelPayoutsResponsePayoutMethodBuilder {
        <CancelPayoutsResponsePayoutMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelPayoutsResponsePayoutMethodBuilder {
    nickname: Option<String>,
    supported_payout_method: Option<CancelPayoutsResponsePayoutMethodSupportedPayoutMethod>,
}

impl CancelPayoutsResponsePayoutMethodBuilder {
    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn supported_payout_method(
        mut self,
        value: CancelPayoutsResponsePayoutMethodSupportedPayoutMethod,
    ) -> Self {
        self.supported_payout_method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelPayoutsResponsePayoutMethod`].
    pub fn build(self) -> Result<CancelPayoutsResponsePayoutMethod, BuildError> {
        Ok(CancelPayoutsResponsePayoutMethod {
            nickname: self.nickname,
            supported_payout_method: self.supported_payout_method,
        })
    }
}
