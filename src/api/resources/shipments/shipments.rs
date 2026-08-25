use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ShipmentsClient {
    pub http_client: HttpClient,
}

impl ShipmentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of shipments for an account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account to list shipments for. Defaults to the acting account.
    /// * `status` - Filter to shipments with this delivery status.
    /// * `payment_id` - Only shipments fulfilling these payments, each prefixed `pay_`. Repeat the parameter to pass several, up to 100 per request — one paginated list covers all of them.
    /// * `created_before` - Return shipments created before this ISO 8601 timestamp.
    /// * `created_after` - Return shipments created after this ISO 8601 timestamp.
    /// * `order` - The field to sort by.
    /// * `direction` - The sort direction.
    /// * `first` - The number of shipments to return.
    /// * `after` - A cursor; returns shipments after this position.
    /// * `last` - The number of shipments to return from the end of the range.
    /// * `before` - A cursor; returns shipments before this position.
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
    ///         .shipments
    ///         .list(
    ///             &ShipmentsListQueryRequest {
    ///                 payment_id: vec![Some("pay_xxxxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 status: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ShipmentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListShipmentsResponse, ApiError> {
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
                "shipments",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .string_array("payment_id", request.payment_id.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Attaches a carrier tracking number to a payment and begins tracking it.
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
    ///         .shipments
    ///         .create(
    ///             &CreateShipmentsRequest {
    ///                 payment_id: "pay_xxxxxxxxxxxxxx".to_string(),
    ///                 tracking_number: "1Z999AA10123456784".to_string(),
    ///                 account_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateShipmentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Shipment, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "shipments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a shipment by its id, or by the payment id it fulfills.
    ///
    /// # Arguments
    ///
    /// * `id` - The shipment id (`ship_`), or the payment id (`pay_`) it fulfills.
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
    ///     client.shipments.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Shipment, ApiError> {
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
                &format!("shipments/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a shipment's tracking number and re-tracks it with the carrier.
    ///
    /// # Arguments
    ///
    /// * `id` - The shipment id (`ship_`), or the payment id (`pay_`) it fulfills.
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
    ///         .shipments
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateShipmentsRequest {
    ///                 tracking_number: "9400111899223456789012".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateShipmentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Shipment, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("shipments/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
