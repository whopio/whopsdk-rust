pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Membership {
    /// The account (seller) this membership belongs to.
    #[serde(default)]
    pub account: StorefrontAccount,
    /// The membership's affiliate commission. `null` without an affiliate, without `affiliate:basic:read` on the account, and always in webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<MembershipAffiliate>,
    /// Number of days between recurring charges. `null` for non-renewing memberships or memberships with multiple renewal schedules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_days: Option<i64>,
    /// Whether the membership is set to cancel when the current billing period ends. Only meaningful for recurring variants.
    #[serde(default)]
    pub cancel_at_period_end: bool,
    /// The category the member picked for why they canceled. `null` when no category was picked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_option: Option<MembershipCancelOption>,
    /// When cancellation was requested, or when the membership was canceled if no request time is recorded, as an ISO 8601 timestamp. `null` when neither is recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    /// Free-text explanation provided when canceling. `null` when no reason was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /// How renewals are collected. `charge_automatically` charges the saved payment method each period; `send_invoice` emails the buyer an invoice each period. `null` when not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<MembershipCollectionMethod>,
    /// When the membership was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the current billing period renews, or when a non-renewing membership expires, as an ISO 8601 timestamp. `null` for one-time purchases with no expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<String>,
    /// When the current billing period started, as an ISO 8601 timestamp. `null` when no billing period is recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<String>,
    /// When access ends for a membership with a fixed expiration, as an ISO 8601 timestamp. `null` when the membership has no expiration, such as a renewing membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<Vec<MembershipExternalAccount>>,
    /// The account's own cancel reason the member picked, as they saw it. `null` when the member picked a default category or gave no reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_option: Option<MembershipFeedbackOption>,
    /// Membership ID, prefixed `mem_`.
    #[serde(default)]
    pub id: String,
    /// The variant's current up-front price, in the variant's currency. A list price, not necessarily what the buyer paid. `amount` is `"0.00"` when the first charge is free.
    #[serde(default)]
    pub initial_price: Money,
    /// The software license key for this membership. Only present when the product includes a software licensing experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    /// URL where the buyer can sign in to manage billing. `null` without a member record or unless the caller is the buyer or has `member:manage` on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_url: Option<String>,
    /// The caller's member row on the account. Present only when the membership belongs to the caller; `null` on seller-side reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<MembershipMember>,
    /// The buyer's member record on the account, prefixed `mber_`. `null` when the membership has no member record, such as an unclaimed membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Custom key-value pairs stored on the membership, commonly used for software licensing.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// The membership's latest lifecycle event, such as `renewing_on` or `canceled_on`. Pair it with `most_recent_action_at`. `null` when none is recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_action: Option<MembershipMostRecentAction>,
    /// When `most_recent_action` happened or is scheduled to happen, as an ISO 8601 timestamp. A future time for upcoming events such as `renewing_on`. `null` when no action is recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_action_at: Option<String>,
    /// The buyer's phone number recorded for this membership, or `null`. The number collected (or verified) at checkout when the seller's phone collection is on; falls back to the buyer's account number when they have shared one with this seller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The product this membership grants access to.
    #[serde(default)]
    pub product: MembershipProduct,
    /// The product this membership grants access to, prefixed `prod_`.
    #[serde(default)]
    pub product_id: String,
    /// The promo code discounting this membership, prefixed `promo_`. `null` when none is applied. Set at checkout or by Apply Promo Code to Membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code_id: Option<String>,
    /// What the next renewal charges, after any promo code that still applies. `null` for non-renewing memberships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<Money>,
    /// Paid installments counted so far on a split pay membership. `null` when the membership is not on split pay, or before its first installment is counted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_pay_current_payments: Option<i64>,
    /// Installment payments required before the membership stops billing. `null` when the membership is not on split pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_pay_required_payments: Option<i64>,
    /// Billing state of the membership. `active`/`trialing` memberships grant access; `past_due` is the grace period after a failed payment; `completed` one-time purchases keep access; `canceled`/`expired` do not.
    pub status: MembershipStatus,
    /// When the membership was last changed, as an ISO 8601 timestamp. Reflects the most recent change to the membership itself, so you can reconcile against webhook retries, replays, and backfills.
    #[serde(default)]
    pub updated_at: String,
    /// The buyer, prefixed `user_`. `null` when the buyer is another business or the membership is unclaimed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The variant the buyer purchased, prefixed `plan_`.
    #[serde(default)]
    pub variant_id: String,
}

impl Membership {
    pub fn builder() -> MembershipBuilder {
        <MembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipBuilder {
    account: Option<StorefrontAccount>,
    affiliate: Option<MembershipAffiliate>,
    billing_period_days: Option<i64>,
    cancel_at_period_end: Option<bool>,
    cancel_option: Option<MembershipCancelOption>,
    canceled_at: Option<String>,
    cancellation_reason: Option<String>,
    collection_method: Option<MembershipCollectionMethod>,
    created_at: Option<String>,
    current_period_end: Option<String>,
    current_period_start: Option<String>,
    expires_at: Option<String>,
    external_accounts: Option<Vec<MembershipExternalAccount>>,
    feedback_option: Option<MembershipFeedbackOption>,
    id: Option<String>,
    initial_price: Option<Money>,
    license_key: Option<String>,
    manage_url: Option<String>,
    member: Option<MembershipMember>,
    member_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    most_recent_action: Option<MembershipMostRecentAction>,
    most_recent_action_at: Option<String>,
    phone_number: Option<String>,
    product: Option<MembershipProduct>,
    product_id: Option<String>,
    promo_code_id: Option<String>,
    renewal_price: Option<Money>,
    split_pay_current_payments: Option<i64>,
    split_pay_required_payments: Option<i64>,
    status: Option<MembershipStatus>,
    updated_at: Option<String>,
    user_id: Option<String>,
    variant_id: Option<String>,
}

impl MembershipBuilder {
    pub fn account(mut self, value: StorefrontAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn affiliate(mut self, value: MembershipAffiliate) -> Self {
        self.affiliate = Some(value);
        self
    }

    pub fn billing_period_days(mut self, value: i64) -> Self {
        self.billing_period_days = Some(value);
        self
    }

    pub fn cancel_at_period_end(mut self, value: bool) -> Self {
        self.cancel_at_period_end = Some(value);
        self
    }

    pub fn cancel_option(mut self, value: MembershipCancelOption) -> Self {
        self.cancel_option = Some(value);
        self
    }

    pub fn canceled_at(mut self, value: impl Into<String>) -> Self {
        self.canceled_at = Some(value.into());
        self
    }

    pub fn cancellation_reason(mut self, value: impl Into<String>) -> Self {
        self.cancellation_reason = Some(value.into());
        self
    }

    pub fn collection_method(mut self, value: MembershipCollectionMethod) -> Self {
        self.collection_method = Some(value);
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

    pub fn current_period_start(mut self, value: impl Into<String>) -> Self {
        self.current_period_start = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn external_accounts(mut self, value: Vec<MembershipExternalAccount>) -> Self {
        self.external_accounts = Some(value);
        self
    }

    pub fn feedback_option(mut self, value: MembershipFeedbackOption) -> Self {
        self.feedback_option = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn initial_price(mut self, value: Money) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn license_key(mut self, value: impl Into<String>) -> Self {
        self.license_key = Some(value.into());
        self
    }

    pub fn manage_url(mut self, value: impl Into<String>) -> Self {
        self.manage_url = Some(value.into());
        self
    }

    pub fn member(mut self, value: MembershipMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn most_recent_action(mut self, value: MembershipMostRecentAction) -> Self {
        self.most_recent_action = Some(value);
        self
    }

    pub fn most_recent_action_at(mut self, value: impl Into<String>) -> Self {
        self.most_recent_action_at = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn product(mut self, value: MembershipProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn promo_code_id(mut self, value: impl Into<String>) -> Self {
        self.promo_code_id = Some(value.into());
        self
    }

    pub fn renewal_price(mut self, value: Money) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn split_pay_current_payments(mut self, value: i64) -> Self {
        self.split_pay_current_payments = Some(value);
        self
    }

    pub fn split_pay_required_payments(mut self, value: i64) -> Self {
        self.split_pay_required_payments = Some(value);
        self
    }

    pub fn status(mut self, value: MembershipStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Membership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](MembershipBuilder::account)
    /// - [`cancel_at_period_end`](MembershipBuilder::cancel_at_period_end)
    /// - [`created_at`](MembershipBuilder::created_at)
    /// - [`id`](MembershipBuilder::id)
    /// - [`initial_price`](MembershipBuilder::initial_price)
    /// - [`metadata`](MembershipBuilder::metadata)
    /// - [`product`](MembershipBuilder::product)
    /// - [`product_id`](MembershipBuilder::product_id)
    /// - [`status`](MembershipBuilder::status)
    /// - [`updated_at`](MembershipBuilder::updated_at)
    /// - [`variant_id`](MembershipBuilder::variant_id)
    pub fn build(self) -> Result<Membership, BuildError> {
        Ok(Membership {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            affiliate: self.affiliate,
            billing_period_days: self.billing_period_days,
            cancel_at_period_end: self
                .cancel_at_period_end
                .ok_or_else(|| BuildError::missing_field("cancel_at_period_end"))?,
            cancel_option: self.cancel_option,
            canceled_at: self.canceled_at,
            cancellation_reason: self.cancellation_reason,
            collection_method: self.collection_method,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            current_period_end: self.current_period_end,
            current_period_start: self.current_period_start,
            expires_at: self.expires_at,
            external_accounts: self.external_accounts,
            feedback_option: self.feedback_option,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_price: self
                .initial_price
                .ok_or_else(|| BuildError::missing_field("initial_price"))?,
            license_key: self.license_key,
            manage_url: self.manage_url,
            member: self.member,
            member_id: self.member_id,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            most_recent_action: self.most_recent_action,
            most_recent_action_at: self.most_recent_action_at,
            phone_number: self.phone_number,
            product: self
                .product
                .ok_or_else(|| BuildError::missing_field("product"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
            promo_code_id: self.promo_code_id,
            renewal_price: self.renewal_price,
            split_pay_current_payments: self.split_pay_current_payments,
            split_pay_required_payments: self.split_pay_required_payments,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user_id: self.user_id,
            variant_id: self
                .variant_id
                .ok_or_else(|| BuildError::missing_field("variant_id"))?,
        })
    }
}
