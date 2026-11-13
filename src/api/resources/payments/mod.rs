use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod levels_rebate;
pub use levels_rebate::LevelsRebateClient;
pub struct PaymentsClient {
    pub http_client: HttpClient,
    pub levels_rebate: LevelsRebateClient,
}

impl PaymentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            levels_rebate: LevelsRebateClient::new(config.clone())?,
        })
    }

    /// Lists payments, newest first. By default, returns sales for the accounts your credential can read: an account credential's own account, or every account a user can read payments for. Set `mode` to `user_sales` to list the sales the signed-in user received personally, outside any account.
    ///
    /// # Arguments
    ///
    /// * `mode` - Which sales to list. `account_sales` returns sales for the accounts the caller can read. `user_sales` returns only sales the signed-in user received personally, outside any account; it requires the user's own Whop login session and cannot be combined with `account_id`.
    /// * `account_id` - Only payments charged by this account, prefixed `biz_`.
    /// * `status` - Only payments in this lifecycle state.
    /// * `tracking_status` - Only payments whose most recently attached kept shipment is in this tracking state. `missing_tracking` selects paid payments with a captured shipping address and no kept shipment, excluding fully refunded payments. `needs_tracking` selects payments with an open pre-shipment funds hold and no kept shipment. Shipment states do not imply a payment or refund state. Requires `shipment:basic:read` as well as `payment:basic:read`; user-wide lists include only accounts readable under both scopes. Only supported with `mode=account_sales`.
    /// * `billing_reason` - Only payments charged for this reason. `subscription_cycle` also matches renewals recorded as `subscription_update`.
    /// * `currency` - Only payments presented in this three-letter currency, such as `usd`.
    /// * `user_id` - Only payments made by this buyer, prefixed `user_`. This filters sales the caller can read, so `me` is not accepted; list the caller's own purchases with `GET /memberships?user_id=me`.
    /// * `query` - Search payments by payment ID, dispute ID, user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    /// * `include_subaccounts` - Also cover the account's subaccounts. Requires `account_id`, and is a 400 without it. Without `query`, lists the payments of the account and every subaccount, newest first by `created_at` only; another `order` is a 400. The listing is cut off after 10 seconds, which a rare filter on an account with little recent activity can reach; add a `query` to narrow it. With `query`, matches on payment, dispute, user or membership ID, buyer email, or a card's first 6 or 8 digits and last 4 separated by a space or dash (`424242 4242`); free-text queries such as a name search only the named account. Subaccounts' payments are returned only for subaccounts you can read payments on.
    /// * `member_id` - Only payments made by this member, prefixed `mber_`.
    /// * `membership_id` - Only payments billed under this membership, prefixed `mem_`.
    /// * `product_id` - Only payments for this product, prefixed `prod_`.
    /// * `variant_id` - Only payments priced by this variant, prefixed `plan_`.
    /// * `created_before` - Only payments created before this ISO 8601 timestamp.
    /// * `created_after` - Only payments created after this ISO 8601 timestamp.
    /// * `statuses` - Only payments in any of these lifecycle states, with the same meanings as `status`. Repeat the parameter to pass several. Combined with `status` when both are sent.
    /// * `substatuses` - Only payments whose `substatus` is any of these. Repeat the parameter to pass several. `incomplete` also matches payments that report `incomplete` because no finer status was recorded.
    /// * `billing_reasons` - Only payments charged for any of these reasons, with the same meanings as `billing_reason`. Repeat the parameter to pass several. Combined with `billing_reason` when both are sent.
    /// * `currencies` - Only payments presented in any of these three-letter currencies. Repeat the parameter to pass several. Combined with `currency` when both are sent.
    /// * `min_amount` - Only payments charged at least this amount: what the buyer was charged after discounts and including tax, in the currency each payment was presented in, the same amount `order=amount` sorts by. Where tax is added on top of the price (`tax_behavior` is `exclusive`), that is the price plus the tax, so a 995 price with tax added does not match `995`: match a range instead. A failed payment that charged nothing matches 0, not its price. Inclusive and compared to the cent. Pair it with `currencies` when the account sells in several. Send the same value as `max_amount` to match one exact amount.
    /// * `max_amount` - Only payments charged at most this amount, measured like `min_amount`: after discounts and including tax, in the currency each payment was presented in. Inclusive and compared to the cent. Must not be less than `min_amount`.
    /// * `variant_ids` - Only payments priced by any of these variants, each prefixed `plan_`. Repeat the parameter to pass several. Combined with `variant_id` when both are sent.
    /// * `product_ids` - Only payments for any of these products, each prefixed `prod_`. Repeat the parameter to pass several. Combined with `product_id` when both are sent.
    /// * `payment_methods` - Only payments paid with any of these methods: a card brand such as `amex`, a wallet such as `apple_pay`, or a payment method type such as `paypal`. Repeat the parameter to pass several.
    /// * `tracking_states` - Only payments whose shipment is in any of these tracking states, or that still need a tracking number (`needs_tracking`). Repeat the parameter to pass several. Requires `account_id` and the shipment:basic:read permission.
    /// * `refunded_after` - Only payments refunded after this ISO 8601 timestamp.
    /// * `refunded_before` - Only payments refunded before this ISO 8601 timestamp.
    /// * `dispute_alerted_after` - Only payments with a dispute alert after this ISO 8601 timestamp.
    /// * `dispute_alerted_before` - Only payments with a dispute alert before this ISO 8601 timestamp.
    /// * `dispute_created_after` - Only payments with a dispute opened after this ISO 8601 timestamp.
    /// * `dispute_created_before` - Only payments with a dispute opened before this ISO 8601 timestamp.
    /// * `order` - The field to sort by. `amount` sorts by the amount charged, in the currency it was charged in.
    /// * `direction` - The sort direction.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `include_total_count` - Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
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
    ///         .payments
    ///         .list(
    ///             &PaymentsListQueryRequest {
    ///                 currencies: vec![Some("eur".to_string())],
    ///                 min_amount: Some(995.0),
    ///                 max_amount: Some(995.0),
    ///                 variant_ids: vec![Some("plan_xxxxxxxxxxxxx".to_string())],
    ///                 product_ids: vec![Some("prod_xxxxxxxxxxxxx".to_string())],
    ///                 mode: None,
    ///                 account_id: None,
    ///                 status: None,
    ///                 tracking_status: None,
    ///                 billing_reason: None,
    ///                 currency: None,
    ///                 user_id: None,
    ///                 query: None,
    ///                 include_subaccounts: None,
    ///                 member_id: None,
    ///                 membership_id: None,
    ///                 product_id: None,
    ///                 variant_id: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 statuses: vec![],
    ///                 substatuses: vec![],
    ///                 billing_reasons: vec![],
    ///                 payment_methods: vec![],
    ///                 tracking_states: vec![],
    ///                 refunded_after: None,
    ///                 refunded_before: None,
    ///                 dispute_alerted_after: None,
    ///                 dispute_alerted_before: None,
    ///                 dispute_created_after: None,
    ///                 dispute_created_before: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 include_total_count: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PaymentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaymentsResponse, ApiError> {
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
                "payments",
                None,
                QueryBuilder::new()
                    .serialize("mode", request.mode.clone())
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("tracking_status", request.tracking_status.clone())
                    .serialize("billing_reason", request.billing_reason.clone())
                    .string("currency", request.currency.clone())
                    .string("user_id", request.user_id.clone())
                    .structured_query("query", request.query.clone())
                    .bool("include_subaccounts", request.include_subaccounts.clone())
                    .string("member_id", request.member_id.clone())
                    .string("membership_id", request.membership_id.clone())
                    .string("product_id", request.product_id.clone())
                    .string("variant_id", request.variant_id.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .serialize_array("statuses", request.statuses.clone())
                    .serialize_array("substatuses", request.substatuses.clone())
                    .serialize_array("billing_reasons", request.billing_reasons.clone())
                    .string_array("currencies", request.currencies.clone())
                    .float("min_amount", request.min_amount.clone())
                    .float("max_amount", request.max_amount.clone())
                    .string_array("variant_ids", request.variant_ids.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .serialize_array("payment_methods", request.payment_methods.clone())
                    .serialize_array("tracking_states", request.tracking_states.clone())
                    .datetime("refunded_after", request.refunded_after.clone())
                    .datetime("refunded_before", request.refunded_before.clone())
                    .datetime(
                        "dispute_alerted_after",
                        request.dispute_alerted_after.clone(),
                    )
                    .datetime(
                        "dispute_alerted_before",
                        request.dispute_alerted_before.clone(),
                    )
                    .datetime(
                        "dispute_created_after",
                        request.dispute_created_after.clone(),
                    )
                    .datetime(
                        "dispute_created_before",
                        request.dispute_created_before.clone(),
                    )
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .bool("include_total_count", request.include_total_count.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &PaymentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .serialize("mode", request.mode.clone())
            .string("account_id", request.account_id.clone())
            .serialize("status", request.status.clone())
            .serialize("tracking_status", request.tracking_status.clone())
            .serialize("billing_reason", request.billing_reason.clone())
            .string("currency", request.currency.clone())
            .string("user_id", request.user_id.clone())
            .structured_query("query", request.query.clone())
            .bool("include_subaccounts", request.include_subaccounts.clone())
            .string("member_id", request.member_id.clone())
            .string("membership_id", request.membership_id.clone())
            .string("product_id", request.product_id.clone())
            .string("variant_id", request.variant_id.clone())
            .datetime("created_before", request.created_before.clone())
            .datetime("created_after", request.created_after.clone())
            .serialize_array("statuses", request.statuses.clone())
            .serialize_array("substatuses", request.substatuses.clone())
            .serialize_array("billing_reasons", request.billing_reasons.clone())
            .string_array("currencies", request.currencies.clone())
            .float("min_amount", request.min_amount.clone())
            .float("max_amount", request.max_amount.clone())
            .string_array("variant_ids", request.variant_ids.clone())
            .string_array("product_ids", request.product_ids.clone())
            .serialize_array("payment_methods", request.payment_methods.clone())
            .serialize_array("tracking_states", request.tracking_states.clone())
            .datetime("refunded_after", request.refunded_after.clone())
            .datetime("refunded_before", request.refunded_before.clone())
            .datetime(
                "dispute_alerted_after",
                request.dispute_alerted_after.clone(),
            )
            .datetime(
                "dispute_alerted_before",
                request.dispute_alerted_before.clone(),
            )
            .datetime(
                "dispute_created_after",
                request.dispute_created_after.clone(),
            )
            .datetime(
                "dispute_created_before",
                request.dispute_created_before.clone(),
            )
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .bool("include_total_count", request.include_total_count.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, cursor_value| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();
                if let Some(cursor) = cursor_value {
                    // Add cursor parameter based on pagination configuration
                    query_params.push(("after".to_string(), cursor));
                }
                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "payments",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction using pagination configuration
                    let items: Vec<serde_json::Value> = response
                        .get("data")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let next_cursor: Option<String> = response
                        .get("page_info")
                        .and_then(|v| v.get("end_cursor"))
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    let has_next_page = next_cursor.is_some();

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with no cursor
        )
    }

    /// Charges a buyer for one or more variants with a payment method already on file (`member_id` and `payment_method_id`), or with a `confirmation_token` for a method the buyer just supplied. Collection runs in the background, so the response is the payment as created, not its outcome: poll Retrieve payment status for how far it has got and what the buyer must still do.
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
    ///         .payments
    ///         .create(
    ///             &CreatePaymentsRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 promo_code: None,
    ///                 promo_code_id: None,
    ///                 variant: None,
    ///                 variant_id: None,
    ///                 affiliate_code: None,
    ///                 auto_capture_after_minutes: None,
    ///                 billing_address: None,
    ///                 capture: None,
    ///                 confirmation_token: None,
    ///                 email: None,
    ///                 ip_address: None,
    ///                 line_items: None,
    ///                 member_id: None,
    ///                 metadata: None,
    ///                 off_session: None,
    ///                 payment_method: None,
    ///                 payment_method_id: None,
    ///                 quote_id: None,
    ///                 return_url: None,
    ///                 setup_future_usage: None,
    ///                 shipping_address: None,
    ///                 statement_descriptor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePaymentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
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
                Method::POST,
                "payments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Totals up the same payments the list returns, so you can build status tabs and filter options without paging through them. Takes the same filters as List Payments.
    ///
    /// # Arguments
    ///
    /// * `groups` - Which breakdowns to return, keyed by these names under `groups`. Repeat the parameter to ask for several; omit it for all of them.
    /// * `account_id` - Only payments charged by this account, prefixed `biz_`.
    /// * `statuses` - Only payments in any of these lifecycle states. Repeat the parameter to pass several.
    /// * `substatuses` - Only payments whose `substatus` is any of these. Repeat the parameter to pass several.
    /// * `billing_reasons` - Only payments charged for any of these reasons. Repeat the parameter to pass several.
    /// * `currencies` - Only payments presented in any of these three-letter currencies. Repeat the parameter to pass several.
    /// * `min_amount` - Only payments charged at least this amount: what the buyer was charged after discounts and including tax, in the currency each payment was presented in. Where tax is added on top of the price (`tax_behavior` is `exclusive`), that is the price plus the tax. A failed payment that charged nothing counts as 0. Inclusive and compared to the cent.
    /// * `max_amount` - Only payments charged at most this amount, measured like `min_amount`: after discounts and including tax. Inclusive and compared to the cent. Must not be less than `min_amount`.
    /// * `variant_ids` - Only payments priced by any of these variants, each prefixed `plan_`. Repeat the parameter to pass several.
    /// * `product_ids` - Only payments for any of these products, each prefixed `prod_`. Repeat the parameter to pass several.
    /// * `payment_methods` - Only payments paid with any of these methods. Repeat the parameter to pass several.
    /// * `tracking_states` - Only payments in any of these tracking states. Requires `account_id` and the shipment:basic:read permission.
    /// * `created_after` - Only payments created after this ISO 8601 timestamp.
    /// * `created_before` - Only payments created before this ISO 8601 timestamp.
    /// * `refunded_after` - Only payments refunded after this ISO 8601 timestamp.
    /// * `refunded_before` - Only payments refunded before this ISO 8601 timestamp.
    /// * `dispute_alerted_after` - Only payments with a dispute alert after this ISO 8601 timestamp.
    /// * `dispute_alerted_before` - Only payments with a dispute alert before this ISO 8601 timestamp.
    /// * `dispute_created_after` - Only payments with a dispute opened after this ISO 8601 timestamp.
    /// * `dispute_created_before` - Only payments with a dispute opened before this ISO 8601 timestamp.
    /// * `query` - Search payments by payment ID, dispute ID, user ID, membership ID, user email, name, or username.
    /// * `include_subaccounts` - Only for searches: with `query`, also search the account's subaccounts. Requires `account_id` and `query`, and is a 400 without either; it never lists subaccounts' payments on its own. Matches on payment, dispute, user or membership ID, buyer email, or a card's first 6 or 8 digits and last 4 separated by a space or dash (`424242 4242`). Free-text queries such as a name search only the named account. Subaccounts' payments are returned only for subaccounts you can read payments on.
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
    ///         .payments
    ///         .summary(
    ///             &PaymentsSummaryQueryRequest {
    ///                 currencies: vec![Some("usd".to_string())],
    ///                 min_amount: Some(995.0),
    ///                 max_amount: Some(995.0),
    ///                 variant_ids: vec![Some("plan_xxxxxxxxxxxxx".to_string())],
    ///                 product_ids: vec![Some("prod_xxxxxxxxxxxxx".to_string())],
    ///                 groups: vec![],
    ///                 account_id: None,
    ///                 statuses: vec![],
    ///                 substatuses: vec![],
    ///                 billing_reasons: vec![],
    ///                 payment_methods: vec![],
    ///                 tracking_states: vec![],
    ///                 created_after: None,
    ///                 created_before: None,
    ///                 refunded_after: None,
    ///                 refunded_before: None,
    ///                 dispute_alerted_after: None,
    ///                 dispute_alerted_before: None,
    ///                 dispute_created_after: None,
    ///                 dispute_created_before: None,
    ///                 query: None,
    ///                 include_subaccounts: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn summary(
        &self,
        request: &PaymentsSummaryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SummaryPaymentsResponse, ApiError> {
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
                "payments/summary",
                None,
                QueryBuilder::new()
                    .serialize_array("groups", request.groups.clone())
                    .string("account_id", request.account_id.clone())
                    .serialize_array("statuses", request.statuses.clone())
                    .serialize_array("substatuses", request.substatuses.clone())
                    .serialize_array("billing_reasons", request.billing_reasons.clone())
                    .string_array("currencies", request.currencies.clone())
                    .float("min_amount", request.min_amount.clone())
                    .float("max_amount", request.max_amount.clone())
                    .string_array("variant_ids", request.variant_ids.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .serialize_array("payment_methods", request.payment_methods.clone())
                    .serialize_array("tracking_states", request.tracking_states.clone())
                    .datetime("created_after", request.created_after.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("refunded_after", request.refunded_after.clone())
                    .datetime("refunded_before", request.refunded_before.clone())
                    .datetime(
                        "dispute_alerted_after",
                        request.dispute_alerted_after.clone(),
                    )
                    .datetime(
                        "dispute_alerted_before",
                        request.dispute_alerted_before.clone(),
                    )
                    .datetime(
                        "dispute_created_after",
                        request.dispute_created_after.clone(),
                    )
                    .datetime(
                        "dispute_created_before",
                        request.dispute_created_before.clone(),
                    )
                    .structured_query("query", request.query.clone())
                    .bool("include_subaccounts", request.include_subaccounts.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns one payment, including every purchased line item with its quantity and subtotal. Related records are ids — resolve a variant, membership, member or shipment on its own endpoint, and list this payment's refunds, disputes or Resolution Center cases with `?payment_id=`.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment, prefixed `pay_`.
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
    ///     client.payments.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
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
                &format!("payments/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a payment's `shipping_address` or `return_url`.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment, prefixed `pay_`.
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
    ///         .payments
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdatePaymentsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdatePaymentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
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
                Method::PATCH,
                &format!("payments/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Captures the full amount of a card payment created with `capture: false`. The payment must still be in `requires_capture` before `capture_expires_at`. Partial capture, multiple captures, capturing more than the authorized amount, and tips are not supported.
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
    ///     client.payments.capture(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn capture(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentStatus, ApiError> {
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
                Method::POST,
                &format!("payments/{}/capture", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the fee breakdown of one payment — Whop's fee, processing, affiliate and other lines — each in the currency it was collected in and converted to the payment's settlement currency. The list is complete in one page.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment whose fees to list, prefixed `pay_`.
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
    ///     client.payments.list_fees(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn list_fees(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListFeesPaymentsResponse, ApiError> {
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
                &format!("payments/{}/fees", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Generates the payment's receipt (invoice) as a PDF and returns a short-lived link to download it. Each call generates a new file and link, so this endpoint does not replay `Idempotency-Key` responses.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment whose receipt to generate, prefixed `pay_`.
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
    ///     client.payments.generate_pdf(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn generate_pdf(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentPdf, ApiError> {
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
                Method::POST,
                &format!("payments/{}/generate_pdf", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Refunds all or part of a payment through the processor that charged it, and updates its membership to match. The buyer is emailed, the affiliate commission on the payment is clawed back, and any open Resolution Center case on the payment is closed.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment to refund, prefixed `pay_`.
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
    ///         .payments
    ///         .refund(
    ///             &"id".to_string(),
    ///             &RefundPaymentsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refund(
        &self,
        id: &str,
        request: &RefundPaymentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
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
                Method::POST,
                &format!("payments/{}/refund", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Charges an unpaid payment again with its original payment method and variant. A payment can typically be retried once, and only while its membership is active, trialing or past due, or when it is a membership's failed first payment.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment to retry, prefixed `pay_`.
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
    ///     client.payments.retry(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retry(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
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
                Method::POST,
                &format!("payments/{}/retry", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Voids or cancels an eligible payment. The request is rejected if the payment is no longer eligible. Some processors confirm the release of a card authorization asynchronously: the payment is then returned still `authorized`, and a `payment.canceled` webhook follows once the hold is released.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment to void, prefixed `pay_`.
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
    ///     client.payments.void(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn void(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Payment, ApiError> {
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
                Method::POST,
                &format!("payments/{}/void", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Charges the saved card again, with the buyer present, for a payment their bank declined until they authenticate (such as with 3DS). Retrieve the payment afterward to get the bank's challenge from `next_action`; once the buyer completes it, they land on `return_url`, or on Whop's hosted verification page if none was given. Only the buyer can call this, with the payment's `client_secret` in the body, an OAuth token for their own account with the `member:payment_methods:use` scope, or the token from a verified `payment` session intent. An API key gets a `403`. Has no effect on a payment that is not waiting on authentication.
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
    ///         .payments
    ///         .resume(
    ///             &"payment_id".to_string(),
    ///             &ResumePaymentsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resume(
        &self,
        payment_id: &str,
        request: &ResumePaymentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentStatus, ApiError> {
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
                Method::POST,
                &format!("payments/{}/resume", payment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes where the buyer lands after completing an off-site step, up until they return. Accepts either a secret key or the payment's own `client_secret` in the body, so the surface that knows the final destination can set it.
    ///
    /// # Arguments
    ///
    /// * `payment_id` - The unique identifier of the payment.
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
    ///         .payments
    ///         .update_return_url(
    ///             &"payment_id".to_string(),
    ///             &UpdateReturnUrlPaymentsRequest {
    ///                 return_url: "https://shinetime.example/checkout/thanks".to_string(),
    ///                 client_secret: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_return_url(
        &self,
        payment_id: &str,
        request: &UpdateReturnUrlPaymentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentStatus, ApiError> {
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
                Method::PATCH,
                &format!("payments/{}/return_url", payment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves how far a payment has got and what the buyer must do next, if anything. A payment is collected in the background, so poll this rather than reading the create response. Accepts either a secret key or the payment's own `client_secret` as a query parameter, so the surface collecting the payment can poll it directly.
    ///
    /// # Arguments
    ///
    /// * `payment_id` - The unique identifier of the payment.
    /// * `client_secret` - The payment's own `client_secret`, for a caller without a secret key.
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
    ///         .payments
    ///         .retrieve_status(
    ///             &"payment_id".to_string(),
    ///             &PaymentsRetrieveStatusQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve_status(
        &self,
        payment_id: &str,
        request: &PaymentsRetrieveStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentStatus, ApiError> {
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
                &format!("payments/{}/status", payment_id),
                None,
                QueryBuilder::new()
                    .string("client_secret", request.client_secret.clone())
                    .build(),
                options,
            )
            .await
    }
}
