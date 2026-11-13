use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TimeSeriesClient {
    pub http_client: HttpClient,
}

impl TimeSeriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the metrics you can chart over time, with the unit each reports and the properties you can filter or break it down by. Aggregates that are not bucketed over time are reports, listed at `GET /stats/reports`.
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
    ///     client.stats.time_series.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListTimeSeriesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                "stats/time_series",
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a metric as a series of points over a time range for an account or user. For an aggregate that is not bucketed over time, use a report from `GET /stats/reports`. The `market_prices` metric is public and requires no authentication. Each metric takes its own scope: most need `stats:read`, the earnings, markup revenue and `platform_markup_amount` metrics need `company:balance:read`, `markup_earning_estimate` needs `company:update_child_fees`, and `ad_delivery`, `ad_campaign_score`, `people` and `events` also accept `ad_campaign:read`.
    ///
    /// # Arguments
    ///
    /// * `metric` - The metric to retrieve, for example net_revenue. Use GET /stats/time_series to see every metric key. The metric sets the unit and the properties you can filter or break down by.
    /// * `account_id` - Account this query concerns, prefixed `biz_`, or `platform` for all of Whop.
    /// * `user_id` - The user this query concerns, for example user_AbC123. Available on metrics that support user subjects, such as account_balance.
    /// * `from` - Start of the range — a date (YYYY-MM-DD), expanded to the start of that day, or an ISO 8601 timestamp (for example 2026-07-16T16:37:00Z), used exactly.
    /// * `to` - End of the range — a date (YYYY-MM-DD), expanded to the end of that day, or an ISO 8601 timestamp (for example 2026-07-17T16:37:00Z), used exactly. Funnel entry ranges cannot exceed 90 days.
    /// * `interval` - How wide each point is. Defaults to day. Snapshot metrics are day-only. Funnels support at most 2,000 first-entry cohort buckets.
    /// * `breakdown_by` - Split the metric out by one of its properties — each point gets a breakdown array. For example breakdown_by=currency returns an entry for usd, an entry for eur, and so on. Funnels use a property of the first matched event, with at most 300 groups. experiment_id and variant require an exposure as step 1. For funnel source breakdowns, steps[1][source]=whop:* groups by campaign, whop:<campaign>:* by ad group, and whop:<campaign>:<group>:* by ad. See the metric catalog for supported breakdowns.
    /// * `convert_to` - Display currency for money metrics — every amount is converted into this ISO currency using the exchange rate on each period's date. Defaults to usd. For the ads metrics (ad_spend, ad_delivery), pass the account's ads reporting currency to match the ad entity endpoints. On transaction metrics, it is ignored when you filter or break down by currency (those report the original transaction currency, unconverted).
    /// * `currency` - Select the source currency or asset on metrics that list currency. For transaction metrics, for example currency=eur, values are reported without conversion. For market_prices, use btc or xaut and convert_to=usd. Pair with breakdown_by=currency to split a metric by currency.
    /// * `time_zone` - IANA time zone to bucket the series in, for example America/New_York. Defaults to UTC. Not accepted by snapshot metrics, which are UTC only.
    /// * `payment_method` - Filter to a single payment method, for example card or crypto. Available on metrics that list payment_method.
    /// * `card_network` - Filter to a single card brand, for example visa. A refinement of payment_method=card. Available on metrics that list card_network.
    /// * `dispute_reason` - Filter disputes to a normalized reason, for example product_not_received. Pair with breakdown_by=dispute_reason to split dispute counts by reason.
    /// * `source` - Filter to a single GMV source, for example payments — or, on the traffic metrics, a visit source (whop_ads, direct, or a utm_source value). Pair with breakdown_by=source to split by source. Available on metrics that list source.
    /// * `hostname` - Filter traffic metrics to one website hostname, for example shop.example.com. On the events and people metrics, comma-separated hostnames match any listed hostname. Pair with breakdown_by=hostname to split by website.
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
    /// * `snapshot_window` - Window used by a snapshot metric. Ordinary snapshots accept 30d as their trailing activity window. Cohorted dispute metrics accept 7d or 28d as the sales-transaction pool; their attribution window is fixed in the metric name. Card network program metrics accept 7d, 14d, 28d, or 56d. Each metric lists its accepted values in the catalog.
    /// * `event` - Filter the events metric to one or more full event names, for example payment.completed or pixel.lead. Comma-separated names match any listed event. Use group_by=event for separate groups. Available on metrics that list event.
    /// * `business_type` - Filter to one business type, for example `education`. Pair with `breakdown_by=business_type` to split by business type. Available on metrics that list `business_type`.
    /// * `industry_type` - Filter to one industry type, for example `options_trading`. Pair with `breakdown_by=industry_type` to split by industry type. Available on metrics that list `industry_type`.
    /// * `product_category` - Filter to one product category, for example `algo_trading_course`. Pair with `breakdown_by=product_category` to split by product category. Available on metrics that list `product_category`.
    /// * `account_country` - Filter to accounts located in one country, as an ISO 3166-1 alpha-2 code such as `US`. Pair with `breakdown_by=account_country` to split by account country. Available on metrics that list `account_country`.
    /// * `issuer` - Filter to one card-issuing bank, named as the processor reports it, for example JPMORGAN CHASE BANK N A, or unknown for payments with no reported bank. Available on metrics that list issuer.
    /// * `webhook_id` - The webhook to count deliveries for, for example hook_AbC123. Required by webhook_deliveries, and must belong to account_id or one of its apps.
    /// * `conversion_window` - Funnel only. Time allowed from the first event to the final event: integer minutes, hours, or days, up to 30d.
    /// * `mature_only` - Funnel only. Include only entrants whose full conversion window has elapsed. Required for confidence intervals and comparisons.
    /// * `steps` - Funnel only. Required when metric=funnel. 2 to 10 ordered events per person, as consecutive one-based bracket parameters such as `steps[1][event]=pixel.page&steps[1][page]=/pricing*&steps[2][event]=payment.completed`. Values are scalar strings, never JSON. A person's first step 1 match inside `from`/`to` anchors their cohort, breakdown, and conversion window; later matches do not restart it. Other events may occur between steps, and conversions may land after `to`. For an experiment funnel, make `experiment.exposure` step 1 and pass `breakdown_by=variant`.
    /// * `compare_to` - Funnel only. The breakdown value to use as baseline for whole-window final conversion. Requires breakdown_by and mature_only=true; defaults confidence_level to 0.95.
    /// * `confidence_level` - Funnel only. Confidence level for whole-window final conversion intervals, for example 0.95. Requires mature_only=true. Exposure steps must each filter one user- or visitor-randomized experiment.
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
    /// * `referring_account_id` - Read a referral metric for a business instead of the caller: the referrals its team members made, past members included, for example biz_AbC123. Requires company:basic:read on that business and cannot be combined with user_id. Available on metrics that list referring_account_id.
    /// * `team_lead_id` - Read a partner metric for one team lead and everyone under them in today's org chart, for example user_AbC123. Needs referring_account_id or a global query. As breakdown_by=team_lead_id, figures sit under the highest lead above the partner who earned them, so each group matches that lead's team_lead_id filter and the groups add up to the total. With team_lead_id set, or for a caller who reads only their own team, the groups are the leads under that person, who keeps the rest. Anyone with no lead is in a null group. Available on metrics that list team_lead_id.
    /// * `activity` - On partner_reps, all counts the period's roster, earning counts those who earned in the period, and not_earning counts the roster minus them. On the other partner metrics, earning keeps the figures of the people partner_reps counts as earning over the whole range and not_earning keeps the figures of everyone else; those need referring_account_id or a global query. Defaults to all. Available on metrics that list activity.
    /// * `breakdown_limit` - With a single breakdown_by, keep the groups with the highest whole-range values, up to this many, and fold every other group into one entry named other. A rate or an average ranks its groups by the deals it divides by. The other entry is measured from the underlying rows, so a distinct count or a rate stays exact rather than being added up from groups. Available on partner metrics only.
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
    ///         .time_series
    ///         .retrieve(
    ///             &"metric".to_string(),
    ///             &StatsTimeSeriesRetrieveQueryRequest {
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
    ///                 business_type: None,
    ///                 industry_type: None,
    ///                 product_category: None,
    ///                 account_country: None,
    ///                 issuer: None,
    ///                 webhook_id: None,
    ///                 conversion_window: None,
    ///                 mature_only: None,
    ///                 steps: None,
    ///                 compare_to: None,
    ///                 confidence_level: None,
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
    ///                 referring_account_id: None,
    ///                 team_lead_id: None,
    ///                 activity: None,
    ///                 breakdown_limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        metric: &str,
        request: &StatsTimeSeriesRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveTimeSeriesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("stats/time_series/{}", metric),
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
                    .string("business_type", request.business_type.clone())
                    .string("industry_type", request.industry_type.clone())
                    .string("product_category", request.product_category.clone())
                    .string("account_country", request.account_country.clone())
                    .string("issuer", request.issuer.clone())
                    .string("webhook_id", request.webhook_id.clone())
                    .string("conversion_window", request.conversion_window.clone())
                    .bool("mature_only", request.mature_only.clone())
                    .serialize("steps", request.steps.clone())
                    .string("compare_to", request.compare_to.clone())
                    .float("confidence_level", request.confidence_level.clone())
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
                    .string("referring_account_id", request.referring_account_id.clone())
                    .string("team_lead_id", request.team_lead_id.clone())
                    .serialize("activity", request.activity.clone())
                    .int("breakdown_limit", request.breakdown_limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
