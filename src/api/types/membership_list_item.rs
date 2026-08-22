pub use crate::prelude::*;

/// A membership represents an active relationship between a user and a product. It tracks the user's access, billing status, and renewal schedule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MembershipListItem {
    /// Whether this membership is set to cancel at the end of the current billing cycle. Only applies to memberships with a recurring plan.
    #[serde(default)]
    pub cancel_at_period_end: bool,
    /// The category selected for why the member canceled (e.g. too_expensive, switching, missing_features).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_option: Option<CancelOptions>,
    /// Whether the customer is canceling, left, or was won back. Null if the membership has no cancellation reason or its cancellation state is indeterminate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelation_status: Option<CancelationStatus>,
    /// The time the customer initiated cancellation of this membership. As a Unix timestamp. Null if the membership has not been canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub canceled_at: Option<DateTime<FixedOffset>>,
    /// Free-text explanation provided by the customer when canceling. Null if the customer did not provide a reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /// The ID of the checkout session/configuration that produced this membership, if any. Use this to map memberships back to the checkout configuration that created them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_configuration_id: Option<String>,
    /// The company this membership belongs to.
    #[serde(default)]
    pub company: MembershipListItemCompany,
    /// The datetime the membership was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code for this membership's billing. Null if the membership is free.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currencies>,
    /// The recurring renewal price for this membership, formatted with currency symbol and billing interval. Null if the membership is not recurring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_renewal_price: Option<String>,
    /// The unique identifier for the membership.
    #[serde(default)]
    pub id: String,
    /// The amount the customer paid when first purchasing this membership, formatted with currency symbol.
    #[serde(default)]
    pub initial_price_paid: String,
    /// The time the user first joined the company associated with this membership. As a Unix timestamp. Null if the member record does not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub joined_at: Option<DateTime<FixedOffset>>,
    /// The software license key associated with this membership. Only present if the product includes a Whop Software Licensing experience. Null otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
    /// The URL where the customer can view and manage this membership, including cancellation and plan changes. Null if no member record exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_url: Option<String>,
    /// The member record linking the user to the company for this membership. Null if the member record has not been created yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<MembershipListItemMember>,
    /// Custom key-value pairs for the membership (commonly used for software licensing, e.g., HWID). Max 50 keys, 100 chars per key, 500 chars per string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Whether recurring payment collection for this membership is temporarily paused by the company.
    #[serde(default)]
    pub payment_collection_paused: bool,
    /// The plan the customer purchased to create this membership.
    #[serde(default)]
    pub plan: MembershipListItemPlan,
    /// The product this membership grants access to.
    #[serde(default)]
    pub product: MembershipListItemProduct,
    /// The promotional code currently applied to this membership's billing. Null if no promo code is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<MembershipListItemPromoCode>,
    /// The end of the current billing period for this recurring membership. As a Unix timestamp. Null if the membership is not recurring.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub renewal_period_end: Option<DateTime<FixedOffset>>,
    /// The start of the current billing period for this recurring membership. As a Unix timestamp. Null if the membership is not recurring.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub renewal_period_start: Option<DateTime<FixedOffset>>,
    /// The current lifecycle status of the membership (e.g., active, trialing, past_due, canceled, expired, completed).
    pub status: MembershipStatus,
    /// The datetime the membership was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The user who owns this membership. Null if the user account has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<MembershipListItemUser>,
}

impl MembershipListItem {
    pub fn builder() -> MembershipListItemBuilder {
        <MembershipListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipListItemBuilder {
    cancel_at_period_end: Option<bool>,
    cancel_option: Option<CancelOptions>,
    cancelation_status: Option<CancelationStatus>,
    canceled_at: Option<DateTime<FixedOffset>>,
    cancellation_reason: Option<String>,
    checkout_configuration_id: Option<String>,
    company: Option<MembershipListItemCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    formatted_renewal_price: Option<String>,
    id: Option<String>,
    initial_price_paid: Option<String>,
    joined_at: Option<DateTime<FixedOffset>>,
    license_key: Option<String>,
    manage_url: Option<String>,
    member: Option<MembershipListItemMember>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    payment_collection_paused: Option<bool>,
    plan: Option<MembershipListItemPlan>,
    product: Option<MembershipListItemProduct>,
    promo_code: Option<MembershipListItemPromoCode>,
    renewal_period_end: Option<DateTime<FixedOffset>>,
    renewal_period_start: Option<DateTime<FixedOffset>>,
    status: Option<MembershipStatus>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<MembershipListItemUser>,
}

impl MembershipListItemBuilder {
    pub fn cancel_at_period_end(mut self, value: bool) -> Self {
        self.cancel_at_period_end = Some(value);
        self
    }

    pub fn cancel_option(mut self, value: CancelOptions) -> Self {
        self.cancel_option = Some(value);
        self
    }

    pub fn cancelation_status(mut self, value: CancelationStatus) -> Self {
        self.cancelation_status = Some(value);
        self
    }

    pub fn canceled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.canceled_at = Some(value);
        self
    }

    pub fn cancellation_reason(mut self, value: impl Into<String>) -> Self {
        self.cancellation_reason = Some(value.into());
        self
    }

    pub fn checkout_configuration_id(mut self, value: impl Into<String>) -> Self {
        self.checkout_configuration_id = Some(value.into());
        self
    }

    pub fn company(mut self, value: MembershipListItemCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn formatted_renewal_price(mut self, value: impl Into<String>) -> Self {
        self.formatted_renewal_price = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn initial_price_paid(mut self, value: impl Into<String>) -> Self {
        self.initial_price_paid = Some(value.into());
        self
    }

    pub fn joined_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.joined_at = Some(value);
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

    pub fn member(mut self, value: MembershipListItemMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn payment_collection_paused(mut self, value: bool) -> Self {
        self.payment_collection_paused = Some(value);
        self
    }

    pub fn plan(mut self, value: MembershipListItemPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: MembershipListItemProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn promo_code(mut self, value: MembershipListItemPromoCode) -> Self {
        self.promo_code = Some(value);
        self
    }

    pub fn renewal_period_end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.renewal_period_end = Some(value);
        self
    }

    pub fn renewal_period_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.renewal_period_start = Some(value);
        self
    }

    pub fn status(mut self, value: MembershipStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn user(mut self, value: MembershipListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembershipListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cancel_at_period_end`](MembershipListItemBuilder::cancel_at_period_end)
    /// - [`company`](MembershipListItemBuilder::company)
    /// - [`created_at`](MembershipListItemBuilder::created_at)
    /// - [`id`](MembershipListItemBuilder::id)
    /// - [`initial_price_paid`](MembershipListItemBuilder::initial_price_paid)
    /// - [`payment_collection_paused`](MembershipListItemBuilder::payment_collection_paused)
    /// - [`plan`](MembershipListItemBuilder::plan)
    /// - [`product`](MembershipListItemBuilder::product)
    /// - [`status`](MembershipListItemBuilder::status)
    /// - [`updated_at`](MembershipListItemBuilder::updated_at)
    pub fn build(self) -> Result<MembershipListItem, BuildError> {
        Ok(MembershipListItem {
            cancel_at_period_end: self
                .cancel_at_period_end
                .ok_or_else(|| BuildError::missing_field("cancel_at_period_end"))?,
            cancel_option: self.cancel_option,
            cancelation_status: self.cancelation_status,
            canceled_at: self.canceled_at,
            cancellation_reason: self.cancellation_reason,
            checkout_configuration_id: self.checkout_configuration_id,
            company: self
                .company
                .ok_or_else(|| BuildError::missing_field("company"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self.currency,
            formatted_renewal_price: self.formatted_renewal_price,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_price_paid: self
                .initial_price_paid
                .ok_or_else(|| BuildError::missing_field("initial_price_paid"))?,
            joined_at: self.joined_at,
            license_key: self.license_key,
            manage_url: self.manage_url,
            member: self.member,
            metadata: self.metadata,
            payment_collection_paused: self
                .payment_collection_paused
                .ok_or_else(|| BuildError::missing_field("payment_collection_paused"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            product: self
                .product
                .ok_or_else(|| BuildError::missing_field("product"))?,
            promo_code: self.promo_code,
            renewal_period_end: self.renewal_period_end,
            renewal_period_start: self.renewal_period_start,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user,
        })
    }
}
