pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequest {
    /// How the account pays for Whop Ads spend. Requires `primary`; `backup` is optional and covers the charge when the primary fails. Requires the `ad_campaign:create` scope on your API key. Configuring a `card` requires a user token; account API keys can configure only `platform_balance` sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_payment_methods: Option<UpdatePreferencesRequestAdsPaymentMethods>,
    /// Lowercase ISO currency code, such as `usd` or `eur`, used to display ad spend and stats. Defaults to `usd`. Requires the `ad_campaign:create` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_reporting_currency: Option<String>,
    /// IANA timezone (e.g. `America/New_York`) used to interpret campaign start/end times and to bucket reports. Cannot be cleared once set — pass a new value to change it. Requires the `ad_campaign:create` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_scheduling_timezone: Option<String>,
    /// Connects or disconnects the Triple Whale integration, or changes the shop it reports to. Requires the `ad_campaign:create` scope on your API key. Connecting requires a `shop_domain` to report spend against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_triple_whale_integration: Option<UpdatePreferencesRequestAdsTripleWhaleIntegration>,
    /// Whether Whop emails members when their membership is canceled or terminated. Set it to `false` to stop these emails while keeping the account's other customer emails. Requires the `company:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_emails: Option<bool>,
    /// Whether incoming funds are automatically moved to the account's cards balance. Requires a cards balance on the account and the `payout:account:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_auto_top_up: Option<bool>,
    /// Whether Whop Card notifications reach this account's team. Set it to `false` to stop every card email and push notification for the account — application status, verification and action-required alerts, card-ready alerts, declines, large charges, and cashback summaries. Cardholder onboarding invitations still send, because they carry the only link an invited cardholder can onboard with. Requesting a card is rejected while notifications are off, since the request reaches nobody. Cards on personal accounts are unaffected. Requires a cards balance on the account and the `payout:account:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_notifications: Option<bool>,
    /// Whether this account covers its subaccounts' negative balances. When a subaccount's balance drops below zero (from refunds, disputes, or fees), this account's available balance in the same currency pays it back, up to what is available. Only accounts with subaccounts can set it. Requires the `payout:account:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covers_subaccount_negative_balances: Option<bool>,
    /// Customer tracking settings for the hosted tracking page and Element, replaced as a whole: fields left out are reset to their defaults. Pass null to clear them, which turns tracking off. Requires the `company:manage_checkout` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tracking: Option<UpdatePreferencesRequestCustomerTracking>,
    /// Whether Whop assembles and files the evidence response when this account's payments are disputed. Off by default; enabling it also opts the account into the success fee charged only on disputes it wins. Requires the `payment:dispute` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_fighter_enabled: Option<bool>,
    /// `false` stops renewing: Economic Intelligence stays on until the end of the current period, and the fee is charged until then. `true` keeps it renewing again before then, or turns it on like the `3_days` offer. Can't be combined with `economic_intelligence_duration_key`. Requires the `company:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_intelligence_auto_renew: Option<bool>,
    /// Turns on Economic Intelligence with the offer that has this `key` in `economic_intelligence_offers`, at that offer's fee. Only `3_days`, the auto-renew offer, is accepted. Any other key is rejected. Returns `403` for an account Economic Intelligence is not offered to. Requires the `company:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economic_intelligence_duration_key:
        Option<UpdatePreferencesRequestEconomicIntelligenceDurationKey>,
    /// Whether Whop emails members when a renewal payment fails or needs them to verify it. Set it to `false` to stop these emails while keeping the account's other customer emails. Subscriptions billed by invoice still get their invoice emails. Requires the `company:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_payment_emails: Option<bool>,
    /// Replaces the account's own cancel reasons, shown to members in this order when they cancel. Pass an option's `id` to keep it, for example when renaming it; options without an `id` are added, and options left out are removed. Pass an empty array to go back to Whop's default reasons. Members who already picked an option keep the label they saw. Requires the `company:manage_checkout` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_options: Option<Vec<UpdatePreferencesRequestFeedbackOptionsItem>>,
    /// Settle every new sale into this currency, regardless of the plan's currency or what the buyer paid in; sales converted this way carry an additional 0.5% FX fee. Pass `null` to go back to settling each sale in its plan's currency. Changing it never converts money already in your balances, and it can change at most once a day (see `preferred_settlement_currency_changeable_at`). Requires the `payout:account:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_settlement_currency: Option<UpdatePreferencesRequestPreferredSettlementCurrency>,
    /// Whether Whop emails members the day before their membership renews or their free trial converts. Set it to `false` to stop that reminder. Members still get the reminders card networks require. Requires the `company:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_reminder_emails: Option<bool>,
    /// Whether Whop emails customers after a purchase asking them to leave a review. Set it to `false` to stop these emails, including ones already scheduled. Requires the `company:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_request_emails: Option<bool>,
    /// Whether Whop emails and notifies the teams of this account's subaccounts. Set it to `false` to stop Whop's emails, push, and in-app notifications to them. These include payout, verification, dispute, and card alerts about their accounts. Only accounts with subaccounts can set it. Requires the `company:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_notifications: Option<bool>,
    /// Whether this account's subaccounts are view-only for their own team members. Set it to `true` to refuse every change a subaccount's team tries to make while still letting them see what their role allows. Only accounts with subaccounts can set it. Requires the `company:update` scope on your API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_team_view_only: Option<bool>,
    /// What happens to a subscription once every retry of a renewal payment has failed. `cancel` (the default) cancels it. `none` leaves it past due and keeps billing it each period; access follows the account's past-due access setting. Requires company:manage_checkout permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_failure_behavior: Option<UpdatePreferencesRequestSubscriptionFailureBehavior>,
}

impl UpdatePreferencesRequest {
    pub fn builder() -> UpdatePreferencesRequestBuilder {
        <UpdatePreferencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestBuilder {
    ads_payment_methods: Option<UpdatePreferencesRequestAdsPaymentMethods>,
    ads_reporting_currency: Option<String>,
    ads_scheduling_timezone: Option<String>,
    ads_triple_whale_integration: Option<UpdatePreferencesRequestAdsTripleWhaleIntegration>,
    cancellation_emails: Option<bool>,
    cards_auto_top_up: Option<bool>,
    cards_notifications: Option<bool>,
    covers_subaccount_negative_balances: Option<bool>,
    customer_tracking: Option<UpdatePreferencesRequestCustomerTracking>,
    dispute_fighter_enabled: Option<bool>,
    economic_intelligence_auto_renew: Option<bool>,
    economic_intelligence_duration_key:
        Option<UpdatePreferencesRequestEconomicIntelligenceDurationKey>,
    failed_payment_emails: Option<bool>,
    feedback_options: Option<Vec<UpdatePreferencesRequestFeedbackOptionsItem>>,
    preferred_settlement_currency: Option<UpdatePreferencesRequestPreferredSettlementCurrency>,
    renewal_reminder_emails: Option<bool>,
    review_request_emails: Option<bool>,
    subaccount_notifications: Option<bool>,
    subaccount_team_view_only: Option<bool>,
    subscription_failure_behavior: Option<UpdatePreferencesRequestSubscriptionFailureBehavior>,
}

impl UpdatePreferencesRequestBuilder {
    pub fn ads_payment_methods(mut self, value: UpdatePreferencesRequestAdsPaymentMethods) -> Self {
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
        value: UpdatePreferencesRequestAdsTripleWhaleIntegration,
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

    pub fn customer_tracking(mut self, value: UpdatePreferencesRequestCustomerTracking) -> Self {
        self.customer_tracking = Some(value);
        self
    }

    pub fn dispute_fighter_enabled(mut self, value: bool) -> Self {
        self.dispute_fighter_enabled = Some(value);
        self
    }

    pub fn economic_intelligence_auto_renew(mut self, value: bool) -> Self {
        self.economic_intelligence_auto_renew = Some(value);
        self
    }

    pub fn economic_intelligence_duration_key(
        mut self,
        value: UpdatePreferencesRequestEconomicIntelligenceDurationKey,
    ) -> Self {
        self.economic_intelligence_duration_key = Some(value);
        self
    }

    pub fn failed_payment_emails(mut self, value: bool) -> Self {
        self.failed_payment_emails = Some(value);
        self
    }

    pub fn feedback_options(
        mut self,
        value: Vec<UpdatePreferencesRequestFeedbackOptionsItem>,
    ) -> Self {
        self.feedback_options = Some(value);
        self
    }

    pub fn preferred_settlement_currency(
        mut self,
        value: UpdatePreferencesRequestPreferredSettlementCurrency,
    ) -> Self {
        self.preferred_settlement_currency = Some(value);
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
        value: UpdatePreferencesRequestSubscriptionFailureBehavior,
    ) -> Self {
        self.subscription_failure_behavior = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesRequest`].
    pub fn build(self) -> Result<UpdatePreferencesRequest, BuildError> {
        Ok(UpdatePreferencesRequest {
            ads_payment_methods: self.ads_payment_methods,
            ads_reporting_currency: self.ads_reporting_currency,
            ads_scheduling_timezone: self.ads_scheduling_timezone,
            ads_triple_whale_integration: self.ads_triple_whale_integration,
            cancellation_emails: self.cancellation_emails,
            cards_auto_top_up: self.cards_auto_top_up,
            cards_notifications: self.cards_notifications,
            covers_subaccount_negative_balances: self.covers_subaccount_negative_balances,
            customer_tracking: self.customer_tracking,
            dispute_fighter_enabled: self.dispute_fighter_enabled,
            economic_intelligence_auto_renew: self.economic_intelligence_auto_renew,
            economic_intelligence_duration_key: self.economic_intelligence_duration_key,
            failed_payment_emails: self.failed_payment_emails,
            feedback_options: self.feedback_options,
            preferred_settlement_currency: self.preferred_settlement_currency,
            renewal_reminder_emails: self.renewal_reminder_emails,
            review_request_emails: self.review_request_emails,
            subaccount_notifications: self.subaccount_notifications,
            subaccount_team_view_only: self.subaccount_team_view_only,
            subscription_failure_behavior: self.subscription_failure_behavior,
        })
    }
}
