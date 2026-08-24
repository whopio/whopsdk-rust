pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerificationsListQueryRequest {
    /// Account or user ID whose verifications you want to list. Use a `biz_` account ID, or the caller's `user_` ID for personal verifications.
    #[serde(default)]
    pub account_id: String,
    /// Field used to sort returned verifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListVerificationsRequestOrder>,
    /// Sort direction for returned verifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListVerificationsRequestDirection>,
}

impl VerificationsListQueryRequest {
    pub fn builder() -> VerificationsListQueryRequestBuilder {
        <VerificationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerificationsListQueryRequestBuilder {
    account_id: Option<String>,
    order: Option<ListVerificationsRequestOrder>,
    direction: Option<ListVerificationsRequestDirection>,
}

impl VerificationsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListVerificationsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListVerificationsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VerificationsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](VerificationsListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<VerificationsListQueryRequest, BuildError> {
        Ok(VerificationsListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            order: self.order,
            direction: self.direction,
        })
    }
}
