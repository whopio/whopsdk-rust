use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaymentMethodsClient {
    pub http_client: HttpClient,
}

impl PaymentMethodsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of payment methods for a member or company, or for the authenticated user when neither is given, with optional filtering by creation date. A payment method is a stored representation of how a customer intends to pay, such as a card, bank account, or digital wallet.
    ///
    /// Required permissions:
    /// - `member:payment_methods:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `member_id` - The unique identifier of the member to list payment methods for. Omit this and account_id to list your own saved payment methods.
    /// * `created_before` - Only return payment methods created before this timestamp.
    /// * `created_after` - Only return payment methods created after this timestamp.
    /// * `payment_method_types` - Only return payment methods of these types. Pass the eligible `type` values from the payment method types catalogue so the list holds nothing the purchase cannot take. An empty list returns no payment methods.
    /// * `card_brands` - Only return cards on these networks, such as the networks the seller accepts. Payment methods that are not cards are unaffected.
    /// * `card_funding_types` - Only return cards funded this way. A card whose funding could not be determined is excluded, and payment methods that are not cards are unaffected.
    /// * `has_payer_document` - Filter cards by whether they carry the payer identity document their payment provider requires. Payment methods that are not cards are unaffected.
    /// * `expired` - Filter by expiry. Only a card can expire, so `false` keeps every payment method that is not past its expiration month and `true` returns expired cards alone.
    /// * `broken` - Filter by whether the stored credential has permanently stopped charging, such as a vault entry its provider closed.
    /// * `account_id` - The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
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
    ///         .payment_methods
    ///         .list(
    ///             &PaymentMethodsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 member_id: Some("mber_xxxxxxxxxxxxx".to_string()),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 account_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 after: None,
    ///                 before: None,
    ///                 direction: None,
    ///                 future_usage: None,
    ///                 payment_method_types: vec![],
    ///                 card_brands: vec![],
    ///                 card_funding_types: vec![],
    ///                 has_payer_document: None,
    ///                 expired: None,
    ///                 broken: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PaymentMethodsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaymentMethodsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "payment_methods",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("member_id", request.member_id.clone())
                    .serialize("direction", request.direction.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .serialize("future_usage", request.future_usage.clone())
                    .serialize_array("payment_method_types", request.payment_method_types.clone())
                    .serialize_array("card_brands", request.card_brands.clone())
                    .serialize_array("card_funding_types", request.card_funding_types.clone())
                    .bool("has_payer_document", request.has_payer_document.clone())
                    .bool("expired", request.expired.clone())
                    .bool("broken", request.broken.clone())
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the details of an existing payment method. Addresses a member's wallet when member_id or account_id is given, otherwise your own.
    ///
    /// Required permissions:
    /// - `member:payment_methods:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment method.
    /// * `member_id` - The unique identifier of the member. Provide either this or account_id, not both. Omit both to address your own saved payment methods.
    /// * `account_id` - The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
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
    ///         .payment_methods
    ///         .retrieve(
    ///             &"payt_xxxxxxxxxxxxx".to_string(),
    ///             &PaymentMethodsRetrieveQueryRequest {
    ///                 member_id: Some("mber_xxxxxxxxxxxxx".to_string()),
    ///                 account_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
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
        request: &PaymentMethodsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethod, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("payment_methods/{}", id),
                None,
                QueryBuilder::new()
                    .string("member_id", request.member_id.clone())
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a saved payment method. Cannot delete a payment method attached to an active subscription.
    ///
    /// Required permissions:
    /// - `member:payment_methods:manage`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment method to delete.
    /// * `member_id` - The unique identifier of the member. Provide either this or account_id, not both. Omit both to address your own saved payment methods.
    /// * `account_id` - The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
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
    ///         .payment_methods
    ///         .delete_payment_method(
    ///             &"payt_xxxxxxxxxxxxx".to_string(),
    ///             &DeletePaymentMethodQueryRequest {
    ///                 member_id: Some("mber_xxxxxxxxxxxxx".to_string()),
    ///                 account_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_payment_method(
        &self,
        id: &str,
        request: &DeletePaymentMethodQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("payment_methods/{}", id),
                None,
                QueryBuilder::new()
                    .string("member_id", request.member_id.clone())
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
