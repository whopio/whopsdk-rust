pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipsListQueryRequest {
    /// Narrow to one account (`biz_` tag). With read access to the account this lists all of its memberships; without, only the caller's own memberships in it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Narrow to one user's memberships (`user_` tag, or `me` for the caller). A user outside the caller's visible set returns an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Narrow to one member's memberships (`mber_` tag), including memberships bought by another business. A member outside the caller's visible set returns an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Narrow to memberships linked to one Discord or Telegram account, given as the platform's user ID prefixed with `discord_` or `telegram_`. Discord matches include memberships imported with that Discord ID. Any other format is a 400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<String>,
    /// Filter by billing state. `trialing`, `active`, `past_due`, `completed`, `canceled`, and `expired` match the membership's `status`; `active`, `trialing`, and `past_due` exclude memberships set to cancel at period end. The other values are derived buckets that never appear as a `status`: `canceling` matches memberships set to cancel at period end; `paused` matches memberships with payment collection paused; `expiring` matches one-time purchases that have not yet expired; `one_time` matches one-time purchases that never expire; `unclaimed` matches memberships with no buyer user, including memberships bought by another business; `banned` matches memberships whose buyer is banned from the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListMembershipsRequestStatus>,
    /// Filter to memberships matching any of these billing states, with the same meanings as `status`. Repeat the parameter to pass several. Combined with `status` when both are sent.
    #[serde(default)]
    pub statuses: Vec<Option<ListMembershipsRequestStatusesItem>>,
    /// Filter to memberships of this product (`prod_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Filter to memberships of any of these products, each prefixed `prod_`. Repeat the parameter to pass several.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Filter to memberships of this variant (`plan_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    /// Filter to memberships of any of these variants, each prefixed `plan_`. Repeat the parameter to pass several.
    #[serde(default)]
    pub variant_ids: Vec<Option<String>>,
    /// Filter to memberships discounted by any of these promo codes, each prefixed `promo_`. Repeat the parameter to pass several.
    #[serde(default)]
    pub promo_code_ids: Vec<Option<String>>,
    /// Only memberships created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only memberships created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListMembershipsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListMembershipsRequestDirection>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total_count: Option<bool>,
}

impl MembershipsListQueryRequest {
    pub fn builder() -> MembershipsListQueryRequestBuilder {
        <MembershipsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    member_id: Option<String>,
    external_account: Option<String>,
    status: Option<ListMembershipsRequestStatus>,
    statuses: Option<Vec<Option<ListMembershipsRequestStatusesItem>>>,
    product_id: Option<String>,
    product_ids: Option<Vec<Option<String>>>,
    variant_id: Option<String>,
    variant_ids: Option<Vec<Option<String>>>,
    promo_code_ids: Option<Vec<Option<String>>>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListMembershipsRequestOrder>,
    direction: Option<ListMembershipsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    include_total_count: Option<bool>,
}

impl MembershipsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn external_account(mut self, value: impl Into<String>) -> Self {
        self.external_account = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListMembershipsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn statuses(mut self, value: Vec<Option<ListMembershipsRequestStatusesItem>>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    pub fn variant_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    pub fn promo_code_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.promo_code_ids = Some(value);
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListMembershipsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListMembershipsRequestDirection) -> Self {
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

    pub fn include_total_count(mut self, value: bool) -> Self {
        self.include_total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembershipsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`statuses`](MembershipsListQueryRequestBuilder::statuses)
    /// - [`product_ids`](MembershipsListQueryRequestBuilder::product_ids)
    /// - [`variant_ids`](MembershipsListQueryRequestBuilder::variant_ids)
    /// - [`promo_code_ids`](MembershipsListQueryRequestBuilder::promo_code_ids)
    pub fn build(self) -> Result<MembershipsListQueryRequest, BuildError> {
        Ok(MembershipsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            member_id: self.member_id,
            external_account: self.external_account,
            status: self.status,
            statuses: self
                .statuses
                .ok_or_else(|| BuildError::missing_field("statuses"))?,
            product_id: self.product_id,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            variant_id: self.variant_id,
            variant_ids: self
                .variant_ids
                .ok_or_else(|| BuildError::missing_field("variant_ids"))?,
            promo_code_ids: self
                .promo_code_ids
                .ok_or_else(|| BuildError::missing_field("promo_code_ids"))?,
            created_after: self.created_after,
            created_before: self.created_before,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            include_total_count: self.include_total_count,
        })
    }
}
