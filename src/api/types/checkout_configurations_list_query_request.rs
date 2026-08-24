pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutConfigurationsListQueryRequest {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Only return checkout configurations for this plan ID, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Only return checkout configurations created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return checkout configurations created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Field used to sort checkout configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListCheckoutConfigurationsRequestOrder>,
    /// Sort direction. Defaults to `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListCheckoutConfigurationsRequestDirection>,
    /// Number of checkout configurations to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl CheckoutConfigurationsListQueryRequest {
    pub fn builder() -> CheckoutConfigurationsListQueryRequestBuilder {
        <CheckoutConfigurationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutConfigurationsListQueryRequestBuilder {
    account_id: Option<String>,
    plan_id: Option<String>,
    created_before: Option<String>,
    created_after: Option<String>,
    order: Option<ListCheckoutConfigurationsRequestOrder>,
    direction: Option<ListCheckoutConfigurationsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
}

impl CheckoutConfigurationsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListCheckoutConfigurationsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListCheckoutConfigurationsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutConfigurationsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CheckoutConfigurationsListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<CheckoutConfigurationsListQueryRequest, BuildError> {
        Ok(CheckoutConfigurationsListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            plan_id: self.plan_id,
            created_before: self.created_before,
            created_after: self.created_after,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
        })
    }
}
