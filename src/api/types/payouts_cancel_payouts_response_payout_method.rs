pub use crate::prelude::*;

/// The saved payout method used. Requires payout:destination:read; null without it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelPayoutsResponsePayoutMethod {
    /// Saved payout method nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Supported payout method display details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method: Option<CancelPayoutsResponsePayoutMethodSupportedPayoutMethod>,
    /// The kind of payout method the payout used. `external` sent funds out of Whop. `whop_card` moved funds onto the account's own Whop Card.
    pub r#type: CancelPayoutsResponsePayoutMethodType,
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
    r#type: Option<CancelPayoutsResponsePayoutMethodType>,
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

    pub fn r#type(mut self, value: CancelPayoutsResponsePayoutMethodType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelPayoutsResponsePayoutMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CancelPayoutsResponsePayoutMethodBuilder::r#type)
    pub fn build(self) -> Result<CancelPayoutsResponsePayoutMethod, BuildError> {
        Ok(CancelPayoutsResponsePayoutMethod {
            nickname: self.nickname,
            supported_payout_method: self.supported_payout_method,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
