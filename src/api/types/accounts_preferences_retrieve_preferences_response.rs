pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePreferencesResponse {
    /// The account's Whop Ads services and payment authorization agreement. While `pending_signature`, campaign launch is blocked; sign by answering `requested_information` via `PATCH /verifications/{id}`.
    pub ads_agreement: RetrievePreferencesResponseAdsAgreement,
    /// How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when the primary fails. `null` until ads billing has been configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_payment_methods: Option<RetrievePreferencesResponseAdsPaymentMethods>,
    /// Lowercase ISO currency code, such as `usd` or `eur`, used to display ad spend and stats. Defaults to `usd`.
    #[serde(default)]
    pub ads_reporting_currency: String,
    /// IANA timezone (e.g. `America/New_York`) used to interpret campaign start/end times and to bucket reports. Defaults to `America/New_York` until explicitly overridden.
    #[serde(default)]
    pub ads_scheduling_timezone: String,
    /// The account's Triple Whale integration, which pushes Whop ad spend to Triple Whale's Data-In API so it reports as a `whop` channel. Available to any Triple Whale customer — Shopify, WooCommerce, or a custom checkout — by setting `shop_domain`. Requires the `ad_campaign:create` scope. Once connected, ad click-through URLs Whop serves carry `tw_source=whop` and `tw_adid=<ad id>` query parameters so Triple Whale's pixel attributes conversions back to the originating ad — no destination URL changes are needed.
    pub ads_triple_whale_integration: RetrievePreferencesResponseAdsTripleWhaleIntegration,
    /// Whether Whop emails members when their membership is canceled or terminated. `true` by default. Set it to `false` to stop these emails while keeping the account's other customer emails. They also stop while the account's `send_customer_emails` is `false`.
    #[serde(default)]
    pub cancellation_emails: bool,
    /// Whether incoming funds are automatically moved to the account's cards balance. `false` when the account has no cards balance.
    #[serde(default)]
    pub cards_auto_top_up: bool,
    /// Whether Whop Card notifications reach this account's team. `true` by default, including when the account has no cards balance. Set it to `false` to stop every card email and push notification for the account — application status, verification and action-required alerts, card-ready alerts, declines, large charges, and cashback summaries. Cardholder onboarding invitations still send, because they carry the only link an invited cardholder can onboard with. Requesting a card is rejected while notifications are off, since the request reaches nobody. Cards on personal accounts are unaffected.
    #[serde(default)]
    pub cards_notifications: bool,
    /// Whether this account covers its subaccounts' negative balances. When a subaccount's balance drops below zero (from refunds, disputes, or fees), this account's available balance in the same currency pays it back, up to what is available. Refunds on its subaccounts may also take them negative by up to this account's available balance. `null` when the account has no subaccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covers_subaccount_negative_balances: Option<bool>,
    /// The saved customer tracking settings for the hosted tracking page and Element, with defaults applied. `null` until set; tracking is off unless `enabled` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tracking: Option<CustomerTrackingPreferences>,
    /// Whether Whop assembles and files the evidence response when this account's payments are disputed. Off by default; enabling it also opts the account into the success fee charged only on disputes it wins.
    #[serde(default)]
    pub dispute_fighter_enabled: bool,
    /// Whether Economic Intelligence is on for the account. It turns off automatically at `economic_intelligence_ends_at`.
    #[serde(default)]
    pub economic_intelligence: bool,
    /// Whether Economic Intelligence renews itself every period, with no end date. Turn it on by setting this to `true`, or by passing the `3_days` offer's key as `economic_intelligence_duration_key`. Set it to `false` to stop renewing: Economic Intelligence stays on until the end of the current period, shown in `economic_intelligence_ends_at`, and the fee is charged until then. Set it back to `true` before then to keep renewing.
    #[serde(default)]
    pub economic_intelligence_auto_renew: bool,
    /// When Economic Intelligence turns off, as an ISO 8601 timestamp: the end of the current period once auto-renew is turned off. `null` when Economic Intelligence is off or renews automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_intelligence_ends_at: Option<String>,
    /// Percentage of payment volume charged while Economic Intelligence is on, such as `3` for 3%. `null` when Economic Intelligence is off.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub economic_intelligence_fee_percentage: Option<f64>,
    /// When auto-renew first renews, as an ISO 8601 timestamp: one period after it was turned on. `null` when auto-renew is off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_intelligence_first_renewal_at: Option<String>,
    /// What the account can choose now to turn on Economic Intelligence. Auto-renew is the only plan: one offer, with its fee and the length of each period, while auto-renew is off, and empty while Economic Intelligence renews automatically.
    #[serde(default)]
    pub economic_intelligence_offers:
        Vec<RetrievePreferencesResponseEconomicIntelligenceOffersItem>,
    /// The account's last Economic Intelligence period, once it has ended. `null` while Economic Intelligence is on, or when it has never been on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_intelligence_previous_period:
        Option<RetrievePreferencesResponseEconomicIntelligencePreviousPeriod>,
    /// Whether Whop emails members when a renewal payment fails or needs them to verify it. `true` by default. Set it to `false` to stop these emails while keeping the account's other customer emails. Subscriptions billed by invoice still get the invoice and its past-due reminders, since that is how the member pays. These emails also stop while the account's `send_customer_emails` is `false`.
    #[serde(default)]
    pub failed_payment_emails: bool,
    /// The account's own cancel reasons, in the order members see them when they cancel. Each is filed under a standard category, saved as the membership's `cancel_option`, so reports keep grouping cancellations. Empty when the account uses Whop's default reasons.
    #[serde(default)]
    pub feedback_options: Vec<RetrievePreferencesResponseFeedbackOptionsItem>,
    /// Lowercase ISO currency code every new sale settles into, regardless of the plan's currency or what the buyer paid in. Sales converted this way carry an additional 0.5% FX fee. `null` when the account settles each sale in its plan's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_settlement_currency:
        Option<RetrievePreferencesResponsePreferredSettlementCurrency>,
    /// When `preferred_settlement_currency` may next be changed or turned off, as an ISO 8601 timestamp. It can change at most once a day. `null` when it may change now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_settlement_currency_changeable_at: Option<String>,
    /// Whether Whop emails members the day before their membership renews or their free trial converts. `true` by default. Set it to `false` to stop that reminder. Members still get the reminders card networks require. They go out 7 days before a trial over 7 days converts, and 30 days before a 6-month or longer plan renews. All stop while `send_customer_emails` is `false`.
    #[serde(default)]
    pub renewal_reminder_emails: bool,
    /// Whether Whop emails customers after a purchase asking them to leave a review. `true` by default. Set it to `false` to stop these emails, including ones already scheduled. They also stop while the account's `send_customer_emails` is `false`.
    #[serde(default)]
    pub review_request_emails: bool,
    /// Whether Whop emails and notifies the teams of this account's subaccounts. `true` by default. Set it to `false` to stop Whop's emails, push, and in-app notifications to them. These include payout, verification, dispute, and card alerts about their accounts. `null` when the account has no subaccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_notifications: Option<bool>,
    /// Whether this account's subaccounts are view-only for their own team members. When `true`, a subaccount's team can still see everything their role allows but every change is refused with a view-only error. `false` by default. `null` when the account has no subaccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_team_view_only: Option<bool>,
    /// What happens to a subscription once every retry of a renewal payment has failed. `cancel` (the default) cancels it. `none` leaves it past due and keeps billing it each period; access follows the account's past-due access setting.
    pub subscription_failure_behavior: RetrievePreferencesResponseSubscriptionFailureBehavior,
}

impl RetrievePreferencesResponse {
    pub fn builder() -> RetrievePreferencesResponseBuilder {
        <RetrievePreferencesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePreferencesResponseBuilder {
    ads_agreement: Option<RetrievePreferencesResponseAdsAgreement>,
    ads_payment_methods: Option<RetrievePreferencesResponseAdsPaymentMethods>,
    ads_reporting_currency: Option<String>,
    ads_scheduling_timezone: Option<String>,
    ads_triple_whale_integration: Option<RetrievePreferencesResponseAdsTripleWhaleIntegration>,
    cancellation_emails: Option<bool>,
    cards_auto_top_up: Option<bool>,
    cards_notifications: Option<bool>,
    covers_subaccount_negative_balances: Option<bool>,
    customer_tracking: Option<CustomerTrackingPreferences>,
    dispute_fighter_enabled: Option<bool>,
    economic_intelligence: Option<bool>,
    economic_intelligence_auto_renew: Option<bool>,
    economic_intelligence_ends_at: Option<String>,
    economic_intelligence_fee_percentage: Option<f64>,
    economic_intelligence_first_renewal_at: Option<String>,
    economic_intelligence_offers:
        Option<Vec<RetrievePreferencesResponseEconomicIntelligenceOffersItem>>,
    economic_intelligence_previous_period:
        Option<RetrievePreferencesResponseEconomicIntelligencePreviousPeriod>,
    failed_payment_emails: Option<bool>,
    feedback_options: Option<Vec<RetrievePreferencesResponseFeedbackOptionsItem>>,
    preferred_settlement_currency: Option<RetrievePreferencesResponsePreferredSettlementCurrency>,
    preferred_settlement_currency_changeable_at: Option<String>,
    renewal_reminder_emails: Option<bool>,
    review_request_emails: Option<bool>,
    subaccount_notifications: Option<bool>,
    subaccount_team_view_only: Option<bool>,
    subscription_failure_behavior: Option<RetrievePreferencesResponseSubscriptionFailureBehavior>,
}

impl RetrievePreferencesResponseBuilder {
    pub fn ads_agreement(mut self, value: RetrievePreferencesResponseAdsAgreement) -> Self {
        self.ads_agreement = Some(value);
        self
    }

    pub fn ads_payment_methods(
        mut self,
        value: RetrievePreferencesResponseAdsPaymentMethods,
    ) -> Self {
        self.ads_payment_methods = Some(value);
        self
    }

    pub fn ads_reporting_currency(mut self, value: impl Into<String>) -> Self {
        self.ads_reporting_currency = Some(value.into());
        self
    }

    pub fn ads_scheduling_timezone(mut self, value: impl Into<String>) -> Self {
        self.ads_scheduling_timezone = Some(value.into());
        self
    }

    pub fn ads_triple_whale_integration(
        mut self,
        value: RetrievePreferencesResponseAdsTripleWhaleIntegration,
    ) -> Self {
        self.ads_triple_whale_integration = Some(value);
        self
    }

    pub fn cancellation_emails(mut self, value: bool) -> Self {
        self.cancellation_emails = Some(value);
        self
    }

    pub fn cards_auto_top_up(mut self, value: bool) -> Self {
        self.cards_auto_top_up = Some(value);
        self
    }

    pub fn cards_notifications(mut self, value: bool) -> Self {
        self.cards_notifications = Some(value);
        self
    }

    pub fn covers_subaccount_negative_balances(mut self, value: bool) -> Self {
        self.covers_subaccount_negative_balances = Some(value);
        self
    }

    pub fn customer_tracking(mut self, value: CustomerTrackingPreferences) -> Self {
        self.customer_tracking = Some(value);
        self
    }

    pub fn dispute_fighter_enabled(mut self, value: bool) -> Self {
        self.dispute_fighter_enabled = Some(value);
        self
    }

    pub fn economic_intelligence(mut self, value: bool) -> Self {
        self.economic_intelligence = Some(value);
        self
    }

    pub fn economic_intelligence_auto_renew(mut self, value: bool) -> Self {
        self.economic_intelligence_auto_renew = Some(value);
        self
    }

    pub fn economic_intelligence_ends_at(mut self, value: impl Into<String>) -> Self {
        self.economic_intelligence_ends_at = Some(value.into());
        self
    }

    pub fn economic_intelligence_fee_percentage(mut self, value: f64) -> Self {
        self.economic_intelligence_fee_percentage = Some(value);
        self
    }

    pub fn economic_intelligence_first_renewal_at(mut self, value: impl Into<String>) -> Self {
        self.economic_intelligence_first_renewal_at = Some(value.into());
        self
    }

    pub fn economic_intelligence_offers(
        mut self,
        value: Vec<RetrievePreferencesResponseEconomicIntelligenceOffersItem>,
    ) -> Self {
        self.economic_intelligence_offers = Some(value);
        self
    }

    pub fn economic_intelligence_previous_period(
        mut self,
        value: RetrievePreferencesResponseEconomicIntelligencePreviousPeriod,
    ) -> Self {
        self.economic_intelligence_previous_period = Some(value);
        self
    }

    pub fn failed_payment_emails(mut self, value: bool) -> Self {
        self.failed_payment_emails = Some(value);
        self
    }

    pub fn feedback_options(
        mut self,
        value: Vec<RetrievePreferencesResponseFeedbackOptionsItem>,
    ) -> Self {
        self.feedback_options = Some(value);
        self
    }

    pub fn preferred_settlement_currency(
        mut self,
        value: RetrievePreferencesResponsePreferredSettlementCurrency,
    ) -> Self {
        self.preferred_settlement_currency = Some(value);
        self
    }

    pub fn preferred_settlement_currency_changeable_at(mut self, value: impl Into<String>) -> Self {
        self.preferred_settlement_currency_changeable_at = Some(value.into());
        self
    }

    pub fn renewal_reminder_emails(mut self, value: bool) -> Self {
        self.renewal_reminder_emails = Some(value);
        self
    }

    pub fn review_request_emails(mut self, value: bool) -> Self {
        self.review_request_emails = Some(value);
        self
    }

    pub fn subaccount_notifications(mut self, value: bool) -> Self {
        self.subaccount_notifications = Some(value);
        self
    }

    pub fn subaccount_team_view_only(mut self, value: bool) -> Self {
        self.subaccount_team_view_only = Some(value);
        self
    }

    pub fn subscription_failure_behavior(
        mut self,
        value: RetrievePreferencesResponseSubscriptionFailureBehavior,
    ) -> Self {
        self.subscription_failure_behavior = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePreferencesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ads_agreement`](RetrievePreferencesResponseBuilder::ads_agreement)
    /// - [`ads_reporting_currency`](RetrievePreferencesResponseBuilder::ads_reporting_currency)
    /// - [`ads_scheduling_timezone`](RetrievePreferencesResponseBuilder::ads_scheduling_timezone)
    /// - [`ads_triple_whale_integration`](RetrievePreferencesResponseBuilder::ads_triple_whale_integration)
    /// - [`cancellation_emails`](RetrievePreferencesResponseBuilder::cancellation_emails)
    /// - [`cards_auto_top_up`](RetrievePreferencesResponseBuilder::cards_auto_top_up)
    /// - [`cards_notifications`](RetrievePreferencesResponseBuilder::cards_notifications)
    /// - [`dispute_fighter_enabled`](RetrievePreferencesResponseBuilder::dispute_fighter_enabled)
    /// - [`economic_intelligence`](RetrievePreferencesResponseBuilder::economic_intelligence)
    /// - [`economic_intelligence_auto_renew`](RetrievePreferencesResponseBuilder::economic_intelligence_auto_renew)
    /// - [`economic_intelligence_offers`](RetrievePreferencesResponseBuilder::economic_intelligence_offers)
    /// - [`failed_payment_emails`](RetrievePreferencesResponseBuilder::failed_payment_emails)
    /// - [`feedback_options`](RetrievePreferencesResponseBuilder::feedback_options)
    /// - [`renewal_reminder_emails`](RetrievePreferencesResponseBuilder::renewal_reminder_emails)
    /// - [`review_request_emails`](RetrievePreferencesResponseBuilder::review_request_emails)
    /// - [`subscription_failure_behavior`](RetrievePreferencesResponseBuilder::subscription_failure_behavior)
    pub fn build(self) -> Result<RetrievePreferencesResponse, BuildError> {
        Ok(RetrievePreferencesResponse {
            ads_agreement: self
                .ads_agreement
                .ok_or_else(|| BuildError::missing_field("ads_agreement"))?,
            ads_payment_methods: self.ads_payment_methods,
            ads_reporting_currency: self
                .ads_reporting_currency
                .ok_or_else(|| BuildError::missing_field("ads_reporting_currency"))?,
            ads_scheduling_timezone: self
                .ads_scheduling_timezone
                .ok_or_else(|| BuildError::missing_field("ads_scheduling_timezone"))?,
            ads_triple_whale_integration: self
                .ads_triple_whale_integration
                .ok_or_else(|| BuildError::missing_field("ads_triple_whale_integration"))?,
            cancellation_emails: self
                .cancellation_emails
                .ok_or_else(|| BuildError::missing_field("cancellation_emails"))?,
            cards_auto_top_up: self
                .cards_auto_top_up
                .ok_or_else(|| BuildError::missing_field("cards_auto_top_up"))?,
            cards_notifications: self
                .cards_notifications
                .ok_or_else(|| BuildError::missing_field("cards_notifications"))?,
            covers_subaccount_negative_balances: self.covers_subaccount_negative_balances,
            customer_tracking: self.customer_tracking,
            dispute_fighter_enabled: self
                .dispute_fighter_enabled
                .ok_or_else(|| BuildError::missing_field("dispute_fighter_enabled"))?,
            economic_intelligence: self
                .economic_intelligence
                .ok_or_else(|| BuildError::missing_field("economic_intelligence"))?,
            economic_intelligence_auto_renew: self
                .economic_intelligence_auto_renew
                .ok_or_else(|| BuildError::missing_field("economic_intelligence_auto_renew"))?,
            economic_intelligence_ends_at: self.economic_intelligence_ends_at,
            economic_intelligence_fee_percentage: self.economic_intelligence_fee_percentage,
            economic_intelligence_first_renewal_at: self.economic_intelligence_first_renewal_at,
            economic_intelligence_offers: self
                .economic_intelligence_offers
                .ok_or_else(|| BuildError::missing_field("economic_intelligence_offers"))?,
            economic_intelligence_previous_period: self.economic_intelligence_previous_period,
            failed_payment_emails: self
                .failed_payment_emails
                .ok_or_else(|| BuildError::missing_field("failed_payment_emails"))?,
            feedback_options: self
                .feedback_options
                .ok_or_else(|| BuildError::missing_field("feedback_options"))?,
            preferred_settlement_currency: self.preferred_settlement_currency,
            preferred_settlement_currency_changeable_at: self
                .preferred_settlement_currency_changeable_at,
            renewal_reminder_emails: self
                .renewal_reminder_emails
                .ok_or_else(|| BuildError::missing_field("renewal_reminder_emails"))?,
            review_request_emails: self
                .review_request_emails
                .ok_or_else(|| BuildError::missing_field("review_request_emails"))?,
            subaccount_notifications: self.subaccount_notifications,
            subaccount_team_view_only: self.subaccount_team_view_only,
            subscription_failure_behavior: self
                .subscription_failure_behavior
                .ok_or_else(|| BuildError::missing_field("subscription_failure_behavior"))?,
        })
    }
}
