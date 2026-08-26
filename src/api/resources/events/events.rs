use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists identity-linked events, most recent first by default. Pass identifier for one person's journey, or omit it to list events for an account within an explicit time range. Pass direction=asc to read a journey forwards from where it starts. Events are shaped like the POST /events intake: attribution in context, identity in user.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Any hard identifier of the person: a person ID (prsn_*), user ID, email, phone number, or a tracking cookie value (wuid, anonymous ID, fbp/fbc/ttp/ga). Omit to list recent events for the account.
    /// * `account_id` - Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    /// * `from` - Start of the time range as an ISO 8601 timestamp. Required when identifier is omitted.
    /// * `to` - End of the time range as an ISO 8601 timestamp. Required when identifier is omitted; otherwise defaults to now.
    /// * `first` - The number of events to return.
    /// * `after` - A cursor for fetching events after a previous page.
    /// * `before` - A cursor for fetching events before a later page.
    /// * `direction` - The order events are returned in by time. Defaults to desc (most recent first); asc reads a journey forwards from where it starts. after and before always page forwards and backwards through that order.
    /// * `event` - Full event names to filter by, comma-separated (payment.completed, pixel.lead, pixel.page, pixel.custom:<name>) — the same vocabulary the events / people metrics use.
    /// * `source` - Canonical source path, exact or with a trailing :* prefix (whop:<campaign>:*, ext:meta:*, referrer:<domain>, direct). Restricts the list to conversion targets attributed to that source — the debuggability twin of a metric cell's source parameter.
    /// * `attribution_model` - Attribution model for the source filter (defaults to last_touch).
    /// * `country` - Country codes to filter by, comma-separated.
    /// * `city` - Cities to filter by, comma-separated.
    /// * `device` - Device families to filter by, comma-separated (e.g. iPhone, Mac).
    /// * `browser` - Browser families to filter by, comma-separated (e.g. Chrome, Mobile Safari).
    /// * `os` - Operating system families to filter by, comma-separated (e.g. iOS, Windows).
    /// * `utm_source` - utm_source values to filter by, comma-separated.
    /// * `hostname` - Page hostnames to filter by, comma-separated.
    /// * `page` - Page paths to filter by, comma-separated.
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
    ///         .events
    ///         .list(
    ///             &EventsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &EventsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEventsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "events",
                None,
                QueryBuilder::new()
                    .string("identifier", request.identifier.clone())
                    .string("account_id", request.account_id.clone())
                    .datetime("from", request.from.clone())
                    .datetime("to", request.to.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .serialize("direction", request.direction.clone())
                    .string("event", request.event.clone())
                    .string("source", request.source.clone())
                    .serialize("attribution_model", request.attribution_model.clone())
                    .string("country", request.country.clone())
                    .string("city", request.city.clone())
                    .string("device", request.device.clone())
                    .string("browser", request.browser.clone())
                    .string("os", request.os.clone())
                    .string("utm_source", request.utm_source.clone())
                    .string("hostname", request.hostname.clone())
                    .string("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Tracks a conversion or engagement event for an account.
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
    ///     client
    ///         .events
    ///         .create(
    ///             &CreateEventsRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 event_name: "coating_deposit_paid".to_string(),
    ///                 action_source: None,
    ///                 context: None,
    ///                 currency: None,
    ///                 custom_name: None,
    ///                 duration: None,
    ///                 event_id: None,
    ///                 event_time: None,
    ///                 plan_id: None,
    ///                 product_id: None,
    ///                 referrer_url: None,
    ///                 resumed: None,
    ///                 source: None,
    ///                 title: None,
    ///                 url: None,
    ///                 user: None,
    ///                 value: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateEventsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateEventsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "events",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a fully anonymized feed of recent platform-wide money movement, most recent first: purchases, affiliate commissions, card and ad spend, app revenue, off-platform sales, wallet deposits, card loads, claimed drops, transfers between accounts, and referral bonuses. Items carry only a `type`, the underlying event name, a USD amount, a coarse location under `user`, and a timestamp coarsened to the start of the minute; missing fields are omitted, not nulled. The payload is identical for every caller; no auth is required.
    ///
    /// # Arguments
    ///
    /// * `event` - Filter to one or more types, comma separated — for example `purchase,card_spend`. These are the item's `type`, not its `event_name`: several types share the `ledger_line.created` event name. Omit for every type in the feed. Values outside the feed's own set are rejected.
    /// * `first` - The number of events to return.
    /// * `after` - A cursor for fetching events after a previous page.
    /// * `before` - A cursor for fetching events before a later page.
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
    ///         .events
    ///         .pulse(
    ///             &PulseQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pulse(
        &self,
        request: &PulseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PulseEventsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "events/pulse",
                None,
                QueryBuilder::new()
                    .string("event", request.event.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Checks whether the Whop pixel is installed for an account. Recent pixel events count as proof on their own, so an account that has sent data lately comes back installed without a `url`. Pass a `url` and events from that page settle it; conversion events are also read across the hostname because they commonly fire on a later confirmation page. If the requested page hasn't sent any events lately, it is fetched and read for the pixel and conversion events wired on it. `installed` is only true when the pixel was actually seen — in the account's events or in the page.
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
    ///     client
    ///         .events
    ///         .validate_pixel(
    ///             &ValidatePixelEventsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn validate_pixel(
        &self,
        request: &ValidatePixelEventsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PixelValidation, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "events/validate_pixel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
