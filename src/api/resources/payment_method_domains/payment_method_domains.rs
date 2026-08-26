use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaymentMethodDomainsClient {
    pub http_client: HttpClient,
}

impl PaymentMethodDomainsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists payment method domains. Without `account_id`, returns the caller's own domains and those of every connected account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only domains registered for this account (`biz_` tag). Defaults to the caller's account plus its connected accounts.
    /// * `hostname` - Only the domain with this exact hostname.
    /// * `status` - Only domains with this verification status.
    /// * `provider` - Only domains registered with this wallet provider.
    /// * `created_before` - Only domains created before this ISO 8601 timestamp.
    /// * `created_after` - Only domains created after this ISO 8601 timestamp.
    /// * `order` - Sort field.
    /// * `direction` - Sort direction.
    /// * `first` - Number of domains to return from the start of the window.
    /// * `after` - Cursor to paginate forwards from.
    /// * `last` - Number of domains to return from the end of the window.
    /// * `before` - Cursor to paginate backwards from.
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
    ///         .payment_method_domains
    ///         .list(
    ///             &PaymentMethodDomainsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PaymentMethodDomainsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaymentMethodDomainsResponse, ApiError> {
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
                "payment_method_domains",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("hostname", request.hostname.clone())
                    .serialize("status", request.status.clone())
                    .serialize("provider", request.provider.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
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

    /// Registers a hostname with the wallet provider and attempts verification inline. Returns `verified` when the provider fetched the domain-association file (for Apple Pay, `/.well-known/apple-developer-merchantid-domain-association`), or `pending` when it could not — host the file, then retry with the verify endpoint.
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
    ///         .payment_method_domains
    ///         .create(
    ///             &CreatePaymentMethodDomainsRequest {
    ///                 hostname: "pending.shinetime.example".to_string(),
    ///                 account_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePaymentMethodDomainsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomain, ApiError> {
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
                "payment_method_domains",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a payment method domain to check its verification status.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment method domain, prefixed `pmd_`.
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
    ///         .payment_method_domains
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomain, ApiError> {
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
                &format!("payment_method_domains/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Unregisters a payment method domain so its wallet payment methods stop rendering there.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment method domain, prefixed `pmd_`.
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
    ///         .payment_method_domains
    ///         .delete(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeletePaymentMethodDomainsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("payment_method_domains/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Re-attempts provider verification of a pending domain once the association file is hosted. Fails with a `bad_request` explaining what to fix; verifying an already `verified` domain is a no-op.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the payment method domain, prefixed `pmd_`.
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
    ///         .payment_method_domains
    ///         .verify(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn verify(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodDomain, ApiError> {
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
                &format!("payment_method_domains/{}/verify", id),
                None,
                None,
                options,
            )
            .await
    }
}
