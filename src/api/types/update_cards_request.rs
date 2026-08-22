pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateCardsRequest {
    /// The owning account ID (a biz_ identifier). Provide this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// New billing address. Requires line1, city, region, postal_code, and country_code. On an invited card, passing billing alone (as the invited user) completes onboarding and starts card provisioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<UpdateCardsRequestBilling>,
    /// Pass `true` to permanently cancel the card. A canceled card cannot be uncanceled. Cannot be combined with other fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<bool>,
    /// Pass `true` to freeze the card, `false` to unfreeze it. The assigned cardholder may freeze their own card without the payout:account:update scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<bool>,
    /// A display name for the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New 4-digit PIN. Can only be set on a card assigned to the acting user, who may set it without the payout:account:update scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
    /// Pass `true` to remove the spending limit (make the card unlimited).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_limit: Option<bool>,
    /// Spending limit amount, in dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub spend_limit: Option<f64>,
    /// The window the spend limit applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_limit_frequency: Option<UpdateCardsRequestSpendLimitFrequency>,
    /// Per-transaction limit amount, in dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub transaction_limit: Option<f64>,
    /// The owning user ID (a user_ identifier). Provide this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl UpdateCardsRequest {
    pub fn builder() -> UpdateCardsRequestBuilder {
        <UpdateCardsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCardsRequestBuilder {
    account_id: Option<String>,
    billing: Option<UpdateCardsRequestBilling>,
    canceled: Option<bool>,
    frozen: Option<bool>,
    name: Option<String>,
    pin: Option<String>,
    remove_limit: Option<bool>,
    spend_limit: Option<f64>,
    spend_limit_frequency: Option<UpdateCardsRequestSpendLimitFrequency>,
    transaction_limit: Option<f64>,
    user_id: Option<String>,
}

impl UpdateCardsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn billing(mut self, value: UpdateCardsRequestBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn canceled(mut self, value: bool) -> Self {
        self.canceled = Some(value);
        self
    }

    pub fn frozen(mut self, value: bool) -> Self {
        self.frozen = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn pin(mut self, value: impl Into<String>) -> Self {
        self.pin = Some(value.into());
        self
    }

    pub fn remove_limit(mut self, value: bool) -> Self {
        self.remove_limit = Some(value);
        self
    }

    pub fn spend_limit(mut self, value: f64) -> Self {
        self.spend_limit = Some(value);
        self
    }

    pub fn spend_limit_frequency(mut self, value: UpdateCardsRequestSpendLimitFrequency) -> Self {
        self.spend_limit_frequency = Some(value);
        self
    }

    pub fn transaction_limit(mut self, value: f64) -> Self {
        self.transaction_limit = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCardsRequest`].
    pub fn build(self) -> Result<UpdateCardsRequest, BuildError> {
        Ok(UpdateCardsRequest {
            account_id: self.account_id,
            billing: self.billing,
            canceled: self.canceled,
            frozen: self.frozen,
            name: self.name,
            pin: self.pin,
            remove_limit: self.remove_limit,
            spend_limit: self.spend_limit,
            spend_limit_frequency: self.spend_limit_frequency,
            transaction_limit: self.transaction_limit,
            user_id: self.user_id,
        })
    }
}
