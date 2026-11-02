pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCardsRequest {
    /// The owning account ID (a biz_ identifier). Provide this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The account member (a user_ identifier) to assign the card to. Required for business card issuing accounts, and whenever a company API key files an account's first card application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user_id: Option<String>,
    /// A display name for the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Spending limit amount, in dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub spend_limit: Option<f64>,
    /// The window the spend limit applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_limit_frequency: Option<CreateCardsRequestSpendLimitFrequency>,
    /// Per-transaction limit amount, in dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub transaction_limit: Option<f64>,
    /// The owning user ID (a user_ identifier). Provide this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreateCardsRequest {
    pub fn builder() -> CreateCardsRequestBuilder {
        <CreateCardsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCardsRequestBuilder {
    account_id: Option<String>,
    assigned_user_id: Option<String>,
    name: Option<String>,
    spend_limit: Option<f64>,
    spend_limit_frequency: Option<CreateCardsRequestSpendLimitFrequency>,
    transaction_limit: Option<f64>,
    user_id: Option<String>,
}

impl CreateCardsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn assigned_user_id(mut self, value: impl Into<String>) -> Self {
        self.assigned_user_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn spend_limit(mut self, value: f64) -> Self {
        self.spend_limit = Some(value);
        self
    }

    pub fn spend_limit_frequency(mut self, value: CreateCardsRequestSpendLimitFrequency) -> Self {
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

    /// Consumes the builder and constructs a [`CreateCardsRequest`].
    pub fn build(self) -> Result<CreateCardsRequest, BuildError> {
        Ok(CreateCardsRequest {
            account_id: self.account_id,
            assigned_user_id: self.assigned_user_id,
            name: self.name,
            spend_limit: self.spend_limit,
            spend_limit_frequency: self.spend_limit_frequency,
            transaction_limit: self.transaction_limit,
            user_id: self.user_id,
        })
    }
}
