pub use crate::prelude::*;

/// The saved payout method used. Requires payout:destination:read; null without it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPayoutsResponseDataItemPayoutMethod {
    /// Saved payout method nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Supported payout method display details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method:
        Option<ListPayoutsResponseDataItemPayoutMethodSupportedPayoutMethod>,
    /// The kind of payout method the payout used. `external` sent funds out of Whop. `whop_card` moved funds onto the account's own Whop Card.
    pub r#type: ListPayoutsResponseDataItemPayoutMethodType,
}

impl ListPayoutsResponseDataItemPayoutMethod {
    pub fn builder() -> ListPayoutsResponseDataItemPayoutMethodBuilder {
        <ListPayoutsResponseDataItemPayoutMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPayoutsResponseDataItemPayoutMethodBuilder {
    nickname: Option<String>,
    supported_payout_method: Option<ListPayoutsResponseDataItemPayoutMethodSupportedPayoutMethod>,
    r#type: Option<ListPayoutsResponseDataItemPayoutMethodType>,
}

impl ListPayoutsResponseDataItemPayoutMethodBuilder {
    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn supported_payout_method(
        mut self,
        value: ListPayoutsResponseDataItemPayoutMethodSupportedPayoutMethod,
    ) -> Self {
        self.supported_payout_method = Some(value);
        self
    }

    pub fn r#type(mut self, value: ListPayoutsResponseDataItemPayoutMethodType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPayoutsResponseDataItemPayoutMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ListPayoutsResponseDataItemPayoutMethodBuilder::r#type)
    pub fn build(self) -> Result<ListPayoutsResponseDataItemPayoutMethod, BuildError> {
        Ok(ListPayoutsResponseDataItemPayoutMethod {
            nickname: self.nickname,
            supported_payout_method: self.supported_payout_method,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
