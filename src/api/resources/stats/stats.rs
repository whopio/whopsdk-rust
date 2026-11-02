use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct StatsClient {
    pub http_client: HttpClient,
}

impl StatsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists every metric you can query, with its unit and the properties you can filter or break it down by.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use whop_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Whop::new(config).expect("Failed to build client");
    ///     client.stats.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListStatsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, "stats", None, None, options)
            .await
    }

    /// Retrieves a metric as a time series of points for an account or user over a time range. The `market_prices` metric is public and requires no authentication.
    ///
    /// # Arguments
    ///
    /// * `metric` - The metric to retrieve, for example net_revenue. Use GET /stats to see every metric key. The metric sets the unit and the properties you can filter or break down by.
    /// * `account_id` - The account this query concerns, for example biz_AbC123.
    /// * `user_id` - The user this query concerns, for example user_AbC123. Available on metrics that support user subjects, such as account_balance.
    /// * `from` - Start of the range — a date (YYYY-MM-DD), expanded to the start of that day, or an ISO 8601 timestamp (for example 2026-07-16T16:37:00Z), used exactly.
    /// * `to` - End of the range — a date (YYYY-MM-DD), expanded to the end of that day, or an ISO 8601 timestamp (for example 2026-07-17T16:37:00Z), used exactly.
    /// * `interval` - How wide each point is. Defaults to day. Snapshot metrics are day-only.
    /// * `breakdown_by` - Split the metric out by one of its properties — each point gets a breakdown array. For example breakdown_by=currency returns an entry for usd, an entry for eur, and so on.
    /// * `convert_to` - Display currency for money metrics — every amount is converted into this ISO currency using the exchange rate on each period's date. Defaults to usd. For the ads metrics (ad_spend, ad_delivery), pass the account's ads reporting currency to match the ad entity endpoints. On transaction metrics, it is ignored when you filter or break down by currency (those report the original transaction currency, unconverted).
    /// * `currency` - Select the source currency or asset on metrics that list currency. For transaction metrics, for example currency=eur, values are reported without conversion. For market_prices, use btc or xaut and convert_to=usd. Pair with breakdown_by=currency to split a metric by currency.
    /// * `time_zone` - IANA time zone to bucket the series in, for example America/New_York. Defaults to UTC. Not accepted by snapshot metrics, which are UTC only.
    /// * `payment_method` - Filter to a single payment method, for example card or crypto. Available on metrics that list payment_method.
    /// * `card_network` - Filter to a single card brand, for example visa. A refinement of payment_method=card. Available on metrics that list card_network.
    /// * `dispute_reason` - Filter disputes to a normalized reason, for example product_not_received. Pair with breakdown_by=dispute_reason to split dispute counts by reason.
    /// * `source` - Filter to a single GMV source, for example payments — or, on the traffic metrics, a visit source (whop_ads, direct, or a utm_source value). Pair with breakdown_by=source to split by source. Available on metrics that list source.
    /// * `hostname` - Filter traffic metrics to one website hostname, for example shop.example.com. Pair with breakdown_by=hostname to split by website.
    /// * `page` - Filter traffic metrics to one page — a hostname plus normalized path, for example shop.example.com/pricing. Pair with breakdown_by=page to split by page.
    /// * `device_type` - Filter traffic metrics to one device type: desktop, mobile, tablet, or unknown. Pair with breakdown_by=device_type to split by device.
    /// * `country_code` - Filter traffic metrics to one visitor country (uppercase ISO 3166-1 alpha-2, for example US). Pair with breakdown_by=country_code to split by country.
    /// * `event_name` - Filter the events metric to one tracked event name, for example pixel.page or pixel.custom. Pair with breakdown_by=event_name to split by event.
    /// * `event_type` - Filter the events metric to a canonical group of events: page_view (pixel page views plus whop.com store views), checkout_start (hosted and embedded checkout views), or other. Pair with breakdown_by=event_type to split by group.
    /// * `custom_name` - Filter the events metric to one merchant-defined custom event name. Only valid alongside event_name=pixel.custom. Pair with breakdown_by=custom_name to split custom events by name.
    /// * `segment` - Filter to a single wallet-balance segment, for example available. Pair with breakdown_by=segment to split the balance. Available on metrics that list segment.
    /// * `category` - Filter to a single balance-activity category, for example payments. Pair with breakdown_by=category to split the activity. Available on metrics that list category.
    /// * `merchant` - Filter to a single cashback merchant bucket, for example whop-ads. Pair with breakdown_by=merchant to split cashback by merchant. Available on metrics that list merchant.
    /// * `fee_type` - Filter to a single fee type. Pair with breakdown_by=fee_type to split fees by type. Available on metrics that list fee_type.
    /// * `product` - Filter to a single product (access pass id), for example prod_AbC123. Pair with breakdown_by=product. Available on metrics that list product.
    /// * `status` - Filter to a single membership status. Pair with breakdown_by=status. Available on metrics that list status.
    /// * `access_level` - Filter to a single access level. Pair with breakdown_by=access_level. Available on metrics that list access_level.
    /// * `most_recent_action` - Filter to a single most-recent member action. Pair with breakdown_by=most_recent_action. Available on metrics that list most_recent_action.
    /// * `referred_user_id` - Filter a referral metric to the businesses attributed to one person you referred, for example user_AbC123. Available on metrics that list referred_user_id.
    /// * `ad_campaign_ids` - Ad campaign ids (adcamp_...) to scope the report to; stats are summed across them. Available on metrics that list ad_campaign_ids.
    /// * `ad_group_ids` - Ad group ids (adgrp_...) to scope the report to; stats are summed across them. Available on metrics that list ad_group_ids.
    /// * `ad_ids` - Ad ids (ad_...) to scope the report to; stats are summed across them. Available on metrics that list ad_ids.
    /// * `snapshot_window` - Window used by a snapshot metric. Ordinary snapshots accept 30d as their trailing activity window. Cohorted dispute metrics accept 7d or 28d as the sales-transaction pool; their attribution window is fixed in the metric name. Each metric lists its accepted values in the catalog.
    /// * `event` - Filter the events metric to one or more full event names, for example payment.completed or pixel.lead. Comma-separated names match any listed event. Use group_by=event for separate groups. Available on metrics that list event.
    /// * `contactable` - People metric only: contactable equals this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `has_purchased` - People metric only: has_purchased equals this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `first_seen_after` - People metric only: first_seen_at greater than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `first_seen_before` - People metric only: first_seen_at less than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `last_seen_after` - People metric only: last_seen_at greater than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `last_seen_before` - People metric only: last_seen_at less than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `first_seen_within_days` - People metric only: first_seen_at within this many days of now. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `last_seen_within_days` - People metric only: last_seen_at within this many days of now. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `known` - People metric only: known equals this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `has_email` - People metric only: has_email equals this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `has_phone` - People metric only: has_phone equals this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `ltv_gt` - People metric only: ltv greater than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `ltv_gte` - People metric only: ltv greater than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `ltv_lt` - People metric only: ltv less than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `ltv_lte` - People metric only: ltv less than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `aov_gt` - People metric only: aov greater than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `aov_gte` - People metric only: aov greater than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `aov_lt` - People metric only: aov less than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `aov_lte` - People metric only: aov less than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `purchase_count_gt` - People metric only: purchase_count greater than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `purchase_count_gte` - People metric only: purchase_count greater than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `purchase_count_lt` - People metric only: purchase_count less than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `purchase_count_lte` - People metric only: purchase_count less than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `event_count_gt` - People metric only: event_count greater than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `event_count_gte` - People metric only: event_count greater than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `event_count_lt` - People metric only: event_count less than this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `event_count_lte` - People metric only: event_count less than or equal this value. Applies to the current person profile for every time bucket. LTV and AOV are in USD. Not accepted by the Events metric.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use whop_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Whop::new(config).expect("Failed to build client");
    ///     client
    ///         .stats
    ///         .retrieve(
    ///             &"metric".to_string(),
    ///             &StatsRetrieveQueryRequest {
    ///                 from: "from".to_string(),
    ///                 to: "to".to_string(),
    ///                 ad_campaign_ids: vec![Some("adcamp_xxxxxxxxxxxxxx".to_string())],
    ///                 ad_group_ids: vec![Some("adgrp_xxxxxxxxxxxxxx".to_string())],
    ///                 ad_ids: vec![Some("ad_xxxxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 user_id: None,
    ///                 interval: None,
    ///                 breakdown_by: None,
    ///                 convert_to: None,
    ///                 currency: None,
    ///                 time_zone: None,
    ///                 payment_method: None,
    ///                 card_network: None,
    ///                 dispute_reason: None,
    ///                 source: None,
    ///                 hostname: None,
    ///                 page: None,
    ///                 device_type: None,
    ///                 country_code: None,
    ///                 event_name: None,
    ///                 event_type: None,
    ///                 custom_name: None,
    ///                 segment: None,
    ///                 category: None,
    ///                 merchant: None,
    ///                 fee_type: None,
    ///                 product: None,
    ///                 status: None,
    ///                 access_level: None,
    ///                 most_recent_action: None,
    ///                 referred_user_id: None,
    ///                 snapshot_window: None,
    ///                 event: None,
    ///                 contactable: None,
    ///                 has_purchased: None,
    ///                 first_seen_after: None,
    ///                 first_seen_before: None,
    ///                 last_seen_after: None,
    ///                 last_seen_before: None,
    ///                 first_seen_within_days: None,
    ///                 last_seen_within_days: None,
    ///                 known: None,
    ///                 has_email: None,
    ///                 has_phone: None,
    ///                 ltv_gt: None,
    ///                 ltv_gte: None,
    ///                 ltv_lt: None,
    ///                 ltv_lte: None,
    ///                 aov_gt: None,
    ///                 aov_gte: None,
    ///                 aov_lt: None,
    ///                 aov_lte: None,
    ///                 purchase_count_gt: None,
    ///                 purchase_count_gte: None,
    ///                 purchase_count_lt: None,
    ///                 purchase_count_lte: None,
    ///                 event_count_gt: None,
    ///                 event_count_gte: None,
    ///                 event_count_lt: None,
    ///                 event_count_lte: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        metric: &str,
        request: &StatsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveStatsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("stats/{}", metric),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("from", request.from.clone())
                    .string("to", request.to.clone())
                    .serialize("interval", request.interval.clone())
                    .string("breakdown_by", request.breakdown_by.clone())
                    .string("convert_to", request.convert_to.clone())
                    .string("currency", request.currency.clone())
                    .string("time_zone", request.time_zone.clone())
                    .string("payment_method", request.payment_method.clone())
                    .string("card_network", request.card_network.clone())
                    .string("dispute_reason", request.dispute_reason.clone())
                    .string("source", request.source.clone())
                    .string("hostname", request.hostname.clone())
                    .string("page", request.page.clone())
                    .string("device_type", request.device_type.clone())
                    .string("country_code", request.country_code.clone())
                    .string("event_name", request.event_name.clone())
                    .serialize("event_type", request.event_type.clone())
                    .string("custom_name", request.custom_name.clone())
                    .string("segment", request.segment.clone())
                    .string("category", request.category.clone())
                    .string("merchant", request.merchant.clone())
                    .string("fee_type", request.fee_type.clone())
                    .string("product", request.product.clone())
                    .string("status", request.status.clone())
                    .string("access_level", request.access_level.clone())
                    .string("most_recent_action", request.most_recent_action.clone())
                    .string("referred_user_id", request.referred_user_id.clone())
                    .string_array("ad_campaign_ids", request.ad_campaign_ids.clone())
                    .string_array("ad_group_ids", request.ad_group_ids.clone())
                    .string_array("ad_ids", request.ad_ids.clone())
                    .serialize("snapshot_window", request.snapshot_window.clone())
                    .string("event", request.event.clone())
                    .bool("contactable", request.contactable.clone())
                    .bool("has_purchased", request.has_purchased.clone())
                    .datetime("first_seen_after", request.first_seen_after.clone())
                    .datetime("first_seen_before", request.first_seen_before.clone())
                    .datetime("last_seen_after", request.last_seen_after.clone())
                    .datetime("last_seen_before", request.last_seen_before.clone())
                    .int(
                        "first_seen_within_days",
                        request.first_seen_within_days.clone(),
                    )
                    .int(
                        "last_seen_within_days",
                        request.last_seen_within_days.clone(),
                    )
                    .bool("known", request.known.clone())
                    .bool("has_email", request.has_email.clone())
                    .bool("has_phone", request.has_phone.clone())
                    .float("ltv_gt", request.ltv_gt.clone())
                    .float("ltv_gte", request.ltv_gte.clone())
                    .float("ltv_lt", request.ltv_lt.clone())
                    .float("ltv_lte", request.ltv_lte.clone())
                    .float("aov_gt", request.aov_gt.clone())
                    .float("aov_gte", request.aov_gte.clone())
                    .float("aov_lt", request.aov_lt.clone())
                    .float("aov_lte", request.aov_lte.clone())
                    .float("purchase_count_gt", request.purchase_count_gt.clone())
                    .float("purchase_count_gte", request.purchase_count_gte.clone())
                    .float("purchase_count_lt", request.purchase_count_lt.clone())
                    .float("purchase_count_lte", request.purchase_count_lte.clone())
                    .float("event_count_gt", request.event_count_gt.clone())
                    .float("event_count_gte", request.event_count_gte.clone())
                    .float("event_count_lt", request.event_count_lt.clone())
                    .float("event_count_lte", request.event_count_lte.clone())
                    .build(),
                options,
            )
            .await
    }
}
