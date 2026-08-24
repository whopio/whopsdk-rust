pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatsRetrieveQueryRequest {
    /// The account this query concerns, for example biz_AbC123.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The user this query concerns, for example user_AbC123. Available on metrics that support user subjects, such as account_balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Start of the range — a date (YYYY-MM-DD), expanded to the start of that day, or an ISO 8601 timestamp (for example 2026-07-16T16:37:00Z), used exactly.
    #[serde(default)]
    pub from: String,
    /// End of the range — a date (YYYY-MM-DD), expanded to the end of that day, or an ISO 8601 timestamp (for example 2026-07-17T16:37:00Z), used exactly.
    #[serde(default)]
    pub to: String,
    /// How wide each point is. Defaults to day. Snapshot metrics are day-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<RetrieveStatsRequestInterval>,
    /// Split the metric out by one of its properties — each point gets a breakdown array. For example breakdown_by=currency returns an entry for usd, an entry for eur, and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown_by: Option<String>,
    /// Display currency for money metrics — every amount is converted into this ISO currency using the exchange rate on each period's date. Defaults to usd. For the ads metrics (ad_spend, ad_delivery), pass the account's ads reporting currency to match the ad entity endpoints. On transaction metrics, it is ignored when you filter or break down by currency (those report the original transaction currency, unconverted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_to: Option<String>,
    /// Select the source currency or asset on metrics that list currency. For transaction metrics, for example currency=eur, values are reported without conversion. For market_prices, use btc or xaut and convert_to=usd. Pair with breakdown_by=currency to split a metric by currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// IANA time zone to bucket the series in, for example America/New_York. Defaults to UTC. Not accepted by snapshot metrics, which are UTC only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Filter to a single payment method, for example card or crypto. Available on metrics that list payment_method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Filter to a single card brand, for example visa. A refinement of payment_method=card. Available on metrics that list card_network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_network: Option<String>,
    /// Filter disputes to a normalized reason, for example product_not_received. Pair with breakdown_by=dispute_reason to split dispute counts by reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_reason: Option<String>,
    /// Filter to a single GMV source, for example payments — or, on the traffic metrics, a visit source (whop_ads, direct, or a utm_source value). Pair with breakdown_by=source to split by source. Available on metrics that list source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Filter traffic metrics to one website hostname, for example shop.example.com. Pair with breakdown_by=hostname to split by website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Filter traffic metrics to one page — a hostname plus normalized path, for example shop.example.com/pricing. Pair with breakdown_by=page to split by page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Filter traffic metrics to one device type: desktop, mobile, tablet, or unknown. Pair with breakdown_by=device_type to split by device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Filter traffic metrics to one visitor country (uppercase ISO 3166-1 alpha-2, for example US). Pair with breakdown_by=country_code to split by country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Filter the events metric to one tracked event name, for example pixel.page or pixel.custom. Pair with breakdown_by=event_name to split by event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// Filter the events metric to a canonical group of events: page_view (pixel page views plus whop.com store views), checkout_start (hosted and embedded checkout views), or other. Pair with breakdown_by=event_type to split by group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<RetrieveStatsRequestEventType>,
    /// Filter the events metric to one merchant-defined custom event name. Only valid alongside event_name=pixel.custom. Pair with breakdown_by=custom_name to split custom events by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// Filter to a single wallet-balance segment, for example available. Pair with breakdown_by=segment to split the balance. Available on metrics that list segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<String>,
    /// Filter to a single balance-activity category, for example payments. Pair with breakdown_by=category to split the activity. Available on metrics that list category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Filter to a single cashback merchant bucket, for example whop-ads. Pair with breakdown_by=merchant to split cashback by merchant. Available on metrics that list merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,
    /// Filter to a single fee type. Pair with breakdown_by=fee_type to split fees by type. Available on metrics that list fee_type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<String>,
    /// Filter to a single product (access pass id), for example prod_AbC123. Pair with breakdown_by=product. Available on metrics that list product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Filter to a single membership status. Pair with breakdown_by=status. Available on metrics that list status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter to a single access level. Pair with breakdown_by=access_level. Available on metrics that list access_level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    /// Filter to a single most-recent member action. Pair with breakdown_by=most_recent_action. Available on metrics that list most_recent_action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_action: Option<String>,
    /// Filter a referral metric to the businesses attributed to one person you referred, for example user_AbC123. Available on metrics that list referred_user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referred_user_id: Option<String>,
    /// Ad campaign ids (adcamp_...) to scope the report to; stats are summed across them. Available on metrics that list ad_campaign_ids.
    #[serde(default)]
    pub ad_campaign_ids: Vec<Option<String>>,
    /// Ad group ids (adgrp_...) to scope the report to; stats are summed across them. Available on metrics that list ad_group_ids.
    #[serde(default)]
    pub ad_group_ids: Vec<Option<String>>,
    /// Ad ids (ad_...) to scope the report to; stats are summed across them. Available on metrics that list ad_ids.
    #[serde(default)]
    pub ad_ids: Vec<Option<String>>,
    /// Window used by a snapshot metric. Ordinary snapshots accept 30d as their trailing activity window. Cohorted dispute metrics accept 7d or 28d as the sales-transaction pool; their attribution window is fixed in the metric name. Each metric lists its accepted values in the catalog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<RetrieveStatsRequestSnapshotWindow>,
    /// Filter the events metric to one or more full event names, for example payment.completed or pixel.lead. Comma-separate several to break the metric down by each event. Available on metrics that list event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

impl StatsRetrieveQueryRequest {
    pub fn builder() -> StatsRetrieveQueryRequestBuilder {
        <StatsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatsRetrieveQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    from: Option<String>,
    to: Option<String>,
    interval: Option<RetrieveStatsRequestInterval>,
    breakdown_by: Option<String>,
    convert_to: Option<String>,
    currency: Option<String>,
    time_zone: Option<String>,
    payment_method: Option<String>,
    card_network: Option<String>,
    dispute_reason: Option<String>,
    source: Option<String>,
    hostname: Option<String>,
    page: Option<String>,
    device_type: Option<String>,
    country_code: Option<String>,
    event_name: Option<String>,
    event_type: Option<RetrieveStatsRequestEventType>,
    custom_name: Option<String>,
    segment: Option<String>,
    category: Option<String>,
    merchant: Option<String>,
    fee_type: Option<String>,
    product: Option<String>,
    status: Option<String>,
    access_level: Option<String>,
    most_recent_action: Option<String>,
    referred_user_id: Option<String>,
    ad_campaign_ids: Option<Vec<Option<String>>>,
    ad_group_ids: Option<Vec<Option<String>>>,
    ad_ids: Option<Vec<Option<String>>>,
    snapshot_window: Option<RetrieveStatsRequestSnapshotWindow>,
    event: Option<String>,
}

impl StatsRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn interval(mut self, value: RetrieveStatsRequestInterval) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn breakdown_by(mut self, value: impl Into<String>) -> Self {
        self.breakdown_by = Some(value.into());
        self
    }

    pub fn convert_to(mut self, value: impl Into<String>) -> Self {
        self.convert_to = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
        self
    }

    pub fn card_network(mut self, value: impl Into<String>) -> Self {
        self.card_network = Some(value.into());
        self
    }

    pub fn dispute_reason(mut self, value: impl Into<String>) -> Self {
        self.dispute_reason = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn page(mut self, value: impl Into<String>) -> Self {
        self.page = Some(value.into());
        self
    }

    pub fn device_type(mut self, value: impl Into<String>) -> Self {
        self.device_type = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: RetrieveStatsRequestEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn segment(mut self, value: impl Into<String>) -> Self {
        self.segment = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn merchant(mut self, value: impl Into<String>) -> Self {
        self.merchant = Some(value.into());
        self
    }

    pub fn fee_type(mut self, value: impl Into<String>) -> Self {
        self.fee_type = Some(value.into());
        self
    }

    pub fn product(mut self, value: impl Into<String>) -> Self {
        self.product = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn access_level(mut self, value: impl Into<String>) -> Self {
        self.access_level = Some(value.into());
        self
    }

    pub fn most_recent_action(mut self, value: impl Into<String>) -> Self {
        self.most_recent_action = Some(value.into());
        self
    }

    pub fn referred_user_id(mut self, value: impl Into<String>) -> Self {
        self.referred_user_id = Some(value.into());
        self
    }

    pub fn ad_campaign_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ad_campaign_ids = Some(value);
        self
    }

    pub fn ad_group_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ad_group_ids = Some(value);
        self
    }

    pub fn ad_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ad_ids = Some(value);
        self
    }

    pub fn snapshot_window(mut self, value: RetrieveStatsRequestSnapshotWindow) -> Self {
        self.snapshot_window = Some(value);
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StatsRetrieveQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](StatsRetrieveQueryRequestBuilder::from)
    /// - [`to`](StatsRetrieveQueryRequestBuilder::to)
    /// - [`ad_campaign_ids`](StatsRetrieveQueryRequestBuilder::ad_campaign_ids)
    /// - [`ad_group_ids`](StatsRetrieveQueryRequestBuilder::ad_group_ids)
    /// - [`ad_ids`](StatsRetrieveQueryRequestBuilder::ad_ids)
    pub fn build(self) -> Result<StatsRetrieveQueryRequest, BuildError> {
        Ok(StatsRetrieveQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            interval: self.interval,
            breakdown_by: self.breakdown_by,
            convert_to: self.convert_to,
            currency: self.currency,
            time_zone: self.time_zone,
            payment_method: self.payment_method,
            card_network: self.card_network,
            dispute_reason: self.dispute_reason,
            source: self.source,
            hostname: self.hostname,
            page: self.page,
            device_type: self.device_type,
            country_code: self.country_code,
            event_name: self.event_name,
            event_type: self.event_type,
            custom_name: self.custom_name,
            segment: self.segment,
            category: self.category,
            merchant: self.merchant,
            fee_type: self.fee_type,
            product: self.product,
            status: self.status,
            access_level: self.access_level,
            most_recent_action: self.most_recent_action,
            referred_user_id: self.referred_user_id,
            ad_campaign_ids: self
                .ad_campaign_ids
                .ok_or_else(|| BuildError::missing_field("ad_campaign_ids"))?,
            ad_group_ids: self
                .ad_group_ids
                .ok_or_else(|| BuildError::missing_field("ad_group_ids"))?,
            ad_ids: self
                .ad_ids
                .ok_or_else(|| BuildError::missing_field("ad_ids"))?,
            snapshot_window: self.snapshot_window,
            event: self.event,
        })
    }
}
