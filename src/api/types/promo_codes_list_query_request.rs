pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromoCodesListQueryRequest {
    /// Account whose promo codes are listed (`biz_` tag).
    #[serde(default)]
    pub account_id: String,
    /// Promo-code status. `expired` groups inactive and archived codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPromoCodesRequestStatus>,
    /// Only promo codes scoped to these product IDs.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Only promo codes scoped to these plan IDs.
    #[serde(default)]
    pub plan_ids: Vec<Option<String>>,
    /// Only promo codes created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only promo codes created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPromoCodesRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPromoCodesRequestDirection>,
    /// Number of promo codes to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to paginate forwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of promo codes to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl PromoCodesListQueryRequest {
    pub fn builder() -> PromoCodesListQueryRequestBuilder {
        <PromoCodesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromoCodesListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListPromoCodesRequestStatus>,
    product_ids: Option<Vec<Option<String>>>,
    plan_ids: Option<Vec<Option<String>>>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    order: Option<ListPromoCodesRequestOrder>,
    direction: Option<ListPromoCodesRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PromoCodesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPromoCodesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn plan_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.plan_ids = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn order(mut self, value: ListPromoCodesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListPromoCodesRequestDirection) -> Self {
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

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PromoCodesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](PromoCodesListQueryRequestBuilder::account_id)
    /// - [`product_ids`](PromoCodesListQueryRequestBuilder::product_ids)
    /// - [`plan_ids`](PromoCodesListQueryRequestBuilder::plan_ids)
    pub fn build(self) -> Result<PromoCodesListQueryRequest, BuildError> {
        Ok(PromoCodesListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            status: self.status,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            plan_ids: self
                .plan_ids
                .ok_or_else(|| BuildError::missing_field("plan_ids"))?,
            created_before: self.created_before,
            created_after: self.created_after,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
