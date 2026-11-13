pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountPreferences {
    /// The account's Whop Ads services and payment authorization agreement. `status` is `not_required`, `pending_signature` (a signature has been requested and campaign launch is blocked until it is provided), or `signed`. While pending, read the fields to answer from `GET /verifications/{id}` and sign by submitting them via `PATCH /verifications/{id}`.
    #[serde(default)]
    pub ads_agreement: HashMap<String, serde_json::Value>,
    /// How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when it fails. Each entry has a `type` of `platform_balance` (id `ldgr_`) or `card` (id `payt_`), plus display fields so the configured source renders even for a viewer who doesn't own it. `backup` is `null` when only one method is configured. `null` until ads billing has been configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_payment_methods: Option<HashMap<String, serde_json::Value>>,
    /// Lowercase ISO currency code, such as `usd` or `eur`, used to display ad spend and stats. Defaults to `usd`.
    #[serde(default)]
    pub ads_reporting_currency: String,
    /// IANA timezone (e.g. `America/New_York`) used to interpret campaign start/end times and to bucket reports. Defaults to `America/New_York` until explicitly overridden.
    #[serde(default)]
    pub ads_scheduling_timezone: String,
    /// The account's Triple Whale integration, which pushes Whop ad spend to Triple Whale's Data-In API so it reports as a `whop` channel, and tags ad click-through URLs with `tw_source`/`tw_adid` so Triple Whale's pixel attributes conversions back to the right ad. `status` is `connected`, `not_connected`, or `requires_shop_domain` (Triple Whale keys records by shop, so spend only flows once `shop_domain` is set). `masked_api_key` shows the leading characters of the stored key; the full key is never returned. `shop_domain` is the shop spend is reported for.
    #[serde(default)]
    pub ads_triple_whale_integration: HashMap<String, serde_json::Value>,
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
    /// Customer tracking settings for the hosted tracking page and Element, with defaults applied. `null` until set; tracking is off unless `enabled` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tracking: Option<CustomerTrackingPreferences>,
    /// Whether Whop assembles and files the evidence response when this account's payments are disputed. Off by default; enabling it also opts the account into the success fee charged only on disputes it wins.
    #[serde(default)]
    pub dispute_fighter_enabled: bool,
    /// Whether Economic Intelligence is on for the account. It turns off automatically at `economic_intelligence_ends_at`. While it is off, the account can't activate an experiment or run a recommendation.
    #[serde(default)]
    pub economic_intelligence: bool,
    /// Whether Economic Intelligence renews itself every period, with no end date. Set it to `true` to turn Economic Intelligence on. Set it to `false` to stop renewing: Economic Intelligence stays on, and its fee is charged, until the end of the current period, shown in `economic_intelligence_ends_at`. Set it back to `true` before then to keep renewing.
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
    #[serde(default)]
    pub economic_intelligence_offers: Vec<AccountEconomicIntelligenceOffer>,
    /// The account's last Economic Intelligence period, once it has ended. `null` while Economic Intelligence is on, or when it has never been on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_intelligence_previous_period: Option<AccountEconomicIntelligencePreviousPeriod>,
    /// Whether Whop emails members when a renewal payment fails or needs them to verify it. `true` by default. Set it to `false` to stop these emails while keeping the account's other customer emails. Subscriptions billed by invoice still get the invoice and its past-due reminders, since that is how the member pays. These emails also stop while the account's `send_customer_emails` is `false`.
    #[serde(default)]
    pub failed_payment_emails: bool,
    #[serde(default)]
    pub feedback_options: Vec<FeedbackOption>,
    /// Lowercase ISO currency code every new sale settles into, regardless of the plan's currency or what the buyer paid in. Sales converted this way carry an additional 0.5% FX fee. `null` when the account settles each sale in its plan's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_settlement_currency: Option<AccountPreferencesPreferredSettlementCurrency>,
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
    pub subscription_failure_behavior: AccountPreferencesSubscriptionFailureBehavior,
}

impl AccountPreferences {
    pub fn builder() -> AccountPreferencesBuilder {
        <AccountPreferencesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountPreferencesBuilder {
    ads_agreement: Option<HashMap<String, serde_json::Value>>,
    ads_payment_methods: Option<HashMap<String, serde_json::Value>>,
    ads_reporting_currency: Option<String>,
    ads_scheduling_timezone: Option<String>,
    ads_triple_whale_integration: Option<HashMap<String, serde_json::Value>>,
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
    economic_intelligence_offers: Option<Vec<AccountEconomicIntelligenceOffer>>,
    economic_intelligence_previous_period: Option<AccountEconomicIntelligencePreviousPeriod>,
    failed_payment_emails: Option<bool>,
    feedback_options: Option<Vec<FeedbackOption>>,
    preferred_settlement_currency: Option<AccountPreferencesPreferredSettlementCurrency>,
    preferred_settlement_currency_changeable_at: Option<String>,
    renewal_reminder_emails: Option<bool>,
    review_request_emails: Option<bool>,
    subaccount_notifications: Option<bool>,
    subaccount_team_view_only: Option<bool>,
    subscription_failure_behavior: Option<AccountPreferencesSubscriptionFailureBehavior>,
}

impl AccountPreferencesBuilder {
    pub fn ads_agreement(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.ads_agreement = Some(value);
        self
    }

    pub fn ads_payment_methods(mut self, value: HashMap<String, serde_json::Value>) -> Self {
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
        value: HashMap<String, serde_json::Value>,
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
        value: Vec<AccountEconomicIntelligenceOffer>,
    ) -> Self {
        self.economic_intelligence_offers = Some(value);
        self
    }

    pub fn economic_intelligence_previous_period(
        mut self,
        value: AccountEconomicIntelligencePreviousPeriod,
    ) -> Self {
        self.economic_intelligence_previous_period = Some(value);
        self
    }

    pub fn failed_payment_emails(mut self, value: bool) -> Self {
        self.failed_payment_emails = Some(value);
        self
    }

    pub fn feedback_options(mut self, value: Vec<FeedbackOption>) -> Self {
        self.feedback_options = Some(value);
        self
    }

    pub fn preferred_settlement_currency(
        mut self,
        value: AccountPreferencesPreferredSettlementCurrency,
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
        value: AccountPreferencesSubscriptionFailureBehavior,
    ) -> Self {
        self.subscription_failure_behavior = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountPreferences`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ads_agreement`](AccountPreferencesBuilder::ads_agreement)
    /// - [`ads_reporting_currency`](AccountPreferencesBuilder::ads_reporting_currency)
    /// - [`ads_scheduling_timezone`](AccountPreferencesBuilder::ads_scheduling_timezone)
    /// - [`ads_triple_whale_integration`](AccountPreferencesBuilder::ads_triple_whale_integration)
    /// - [`cancellation_emails`](AccountPreferencesBuilder::cancellation_emails)
    /// - [`cards_auto_top_up`](AccountPreferencesBuilder::cards_auto_top_up)
    /// - [`cards_notifications`](AccountPreferencesBuilder::cards_notifications)
    /// - [`dispute_fighter_enabled`](AccountPreferencesBuilder::dispute_fighter_enabled)
    /// - [`economic_intelligence`](AccountPreferencesBuilder::economic_intelligence)
    /// - [`economic_intelligence_auto_renew`](AccountPreferencesBuilder::economic_intelligence_auto_renew)
    /// - [`economic_intelligence_offers`](AccountPreferencesBuilder::economic_intelligence_offers)
    /// - [`failed_payment_emails`](AccountPreferencesBuilder::failed_payment_emails)
    /// - [`feedback_options`](AccountPreferencesBuilder::feedback_options)
    /// - [`renewal_reminder_emails`](AccountPreferencesBuilder::renewal_reminder_emails)
    /// - [`review_request_emails`](AccountPreferencesBuilder::review_request_emails)
    /// - [`subscription_failure_behavior`](AccountPreferencesBuilder::subscription_failure_behavior)
    pub fn build(self) -> Result<AccountPreferences, BuildError> {
        Ok(AccountPreferences {
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
