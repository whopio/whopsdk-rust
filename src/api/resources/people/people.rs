use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PeopleClient {
    pub http_client: HttpClient,
}

impl PeopleClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the people (visitors and customers) of an account: the identity-linked person profiles aggregated from every pixel, payment, and platform event — identities, purchases and LTV, geo/device profile, traffic sources, and first/last marketing touches.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    /// * `query` - Search people by name, email, phone, or whop user ID (case-insensitive substring match).
    /// * `source` - Only include people acquired from any of these sources — canonical paths (whop:<campaign>:<group>:<ad>, ext:<platform>:..., referrer:<domain>, direct, other), exact or with a trailing :* prefix. The same vocabulary the events / people metrics use.
    /// * `attribution_model` - Attribution model the source filter matches against (defaults to last_touch).
    /// * `event_name` - Only include people who fired any of these events, e.g. payment.completed or page.checkout.view.
    /// * `custom_event` - Only include people who fired this custom pixel event.
    /// * `event_from` - With event_to plus an event or source filter, switches to exact-population mode: person ids are resolved and paginated on the events side within this window (the same query the people metric counts), then hydrated per page.
    /// * `event_to` - The inclusive end of the event window for exact-population mode.
    /// * `audience_id` - Only include people in this audience. An audience that keeps itself up to date resolves to the People filters that define it, so this always reflects who matches now; uploaded lists and point-in-time snapshots match their recorded members.
    /// * `user_id` - Only include the person linked to this whop user ID.
    /// * `email` - Only include the person linked to this email address.
    /// * `phone` - Only include the person linked to this phone number.
    /// * `country` - Only include people whose most recent visit came from this ISO 3166-1 alpha-2 country code.
    /// * `has_purchased` - true for customers only, false for people who have never purchased.
    /// * `contactable` - true for people who have an email address or phone number — the ones an ad platform can match.
    /// * `first_seen_within_days` - Only include people first seen within this many days, as a rolling window.
    /// * `last_seen_within_days` - Only include people last seen within this many days, as a rolling window.
    /// * `first_seen_after` - Only include people first seen at or after this ISO 8601 timestamp.
    /// * `first_seen_before` - Only include people first seen before this ISO 8601 timestamp.
    /// * `last_seen_after` - Only include people last seen at or after this ISO 8601 timestamp.
    /// * `last_seen_before` - Only include people last seen before this ISO 8601 timestamp.
    /// * `first` - The number of people to return (default 100, max 100).
    /// * `after` - A cursor for fetching people after a previous page.
    /// * `before` - A cursor for fetching people before a later page.
    /// * `order` - Column to sort by. Defaults to last_seen_at.
    /// * `direction` - Sort direction. Defaults to desc.
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
    ///         .people
    ///         .list(
    ///             &PeopleListQueryRequest {
    ///                 source: vec![Some("direct".to_string())],
    ///                 event_name: vec![Some("payment.completed".to_string())],
    ///                 account_id: None,
    ///                 query: None,
    ///                 attribution_model: None,
    ///                 custom_event: None,
    ///                 event_from: None,
    ///                 event_to: None,
    ///                 audience_id: None,
    ///                 user_id: None,
    ///                 email: None,
    ///                 phone: None,
    ///                 country: None,
    ///                 has_purchased: None,
    ///                 contactable: None,
    ///                 first_seen_within_days: None,
    ///                 last_seen_within_days: None,
    ///                 first_seen_after: None,
    ///                 first_seen_before: None,
    ///                 last_seen_after: None,
    ///                 last_seen_before: None,
    ///                 first: None,
    ///                 after: None,
    ///                 before: None,
    ///                 order: None,
    ///                 direction: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PeopleListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPeopleResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "people",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .structured_query("query", request.query.clone())
                    .string_array("source", request.source.clone())
                    .serialize("attribution_model", request.attribution_model.clone())
                    .string_array("event_name", request.event_name.clone())
                    .string("custom_event", request.custom_event.clone())
                    .datetime("event_from", request.event_from.clone())
                    .datetime("event_to", request.event_to.clone())
                    .string("audience_id", request.audience_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("email", request.email.clone())
                    .string("phone", request.phone.clone())
                    .string("country", request.country.clone())
                    .bool("has_purchased", request.has_purchased.clone())
                    .bool("contactable", request.contactable.clone())
                    .int(
                        "first_seen_within_days",
                        request.first_seen_within_days.clone(),
                    )
                    .int(
                        "last_seen_within_days",
                        request.last_seen_within_days.clone(),
                    )
                    .datetime("first_seen_after", request.first_seen_after.clone())
                    .datetime("first_seen_before", request.first_seen_before.clone())
                    .datetime("last_seen_after", request.last_seen_after.clone())
                    .datetime("last_seen_before", request.last_seen_before.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves one person for an account. The identifier can be a person ID (prefixed `prsn_`), a user ID (prefixed `user_`), an email address, or a phone number — merged people resolve to the surviving profile.
    ///
    /// # Arguments
    ///
    /// * `id` - The person ID, user ID, email address, or phone number to look up.
    /// * `account_id` - Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
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
    ///         .people
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &PeopleRetrieveQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        request: &PeopleRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RetrievePeopleResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("people/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
