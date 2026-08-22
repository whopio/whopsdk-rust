pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Membership {
    /// The account (seller) this membership belongs to.
    #[serde(default)]
    pub account: MembershipAccount,
    /// Whether the membership is set to cancel when the current billing period ends. Only meaningful for recurring plans.
    #[serde(default)]
    pub cancel_at_period_end: bool,
    /// When the membership was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the current billing period renews, or when a non-renewing membership expires, as an ISO 8601 timestamp. `null` for one-time purchases with no expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<String>,
    /// Membership ID, prefixed `mem_`.
    #[serde(default)]
    pub id: String,
    /// The software license key for this membership. Only present when the product includes a software licensing experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    /// The caller's member row on the account. Present only when the membership belongs to the caller; `null` on seller-side reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<MembershipMember>,
    /// Custom key-value pairs stored on the membership, commonly used for software licensing.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// The plan the buyer purchased, prefixed `plan_`.
    #[serde(default)]
    pub plan_id: String,
    /// The product this membership grants access to, prefixed `prod_`.
    #[serde(default)]
    pub product_id: String,
    /// Billing state of the membership. `active`/`trialing` memberships grant access; `past_due` is the grace period after a failed payment; `completed` one-time purchases keep access; `canceled`/`expired` do not.
    pub status: MembershipStatus,
    /// The buyer, prefixed `user_`. `null` when the buyer is another business or the membership is unclaimed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl Membership {
    pub fn builder() -> MembershipBuilder {
        <MembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipBuilder {
    account: Option<MembershipAccount>,
    cancel_at_period_end: Option<bool>,
    created_at: Option<String>,
    current_period_end: Option<String>,
    id: Option<String>,
    license_key: Option<String>,
    member: Option<MembershipMember>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    plan_id: Option<String>,
    product_id: Option<String>,
    status: Option<MembershipStatus>,
    user_id: Option<String>,
}

impl MembershipBuilder {
    pub fn account(mut self, value: MembershipAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn cancel_at_period_end(mut self, value: bool) -> Self {
        self.cancel_at_period_end = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn current_period_end(mut self, value: impl Into<String>) -> Self {
        self.current_period_end = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn license_key(mut self, value: impl Into<String>) -> Self {
        self.license_key = Some(value.into());
        self
    }

    pub fn member(mut self, value: MembershipMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: MembershipStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Membership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](MembershipBuilder::account)
    /// - [`cancel_at_period_end`](MembershipBuilder::cancel_at_period_end)
    /// - [`created_at`](MembershipBuilder::created_at)
    /// - [`id`](MembershipBuilder::id)
    /// - [`metadata`](MembershipBuilder::metadata)
    /// - [`plan_id`](MembershipBuilder::plan_id)
    /// - [`product_id`](MembershipBuilder::product_id)
    /// - [`status`](MembershipBuilder::status)
    pub fn build(self) -> Result<Membership, BuildError> {
        Ok(Membership {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            cancel_at_period_end: self
                .cancel_at_period_end
                .ok_or_else(|| BuildError::missing_field("cancel_at_period_end"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            current_period_end: self.current_period_end,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            license_key: self.license_key,
            member: self.member,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            plan_id: self
                .plan_id
                .ok_or_else(|| BuildError::missing_field("plan_id"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            user_id: self.user_id,
        })
    }
}
