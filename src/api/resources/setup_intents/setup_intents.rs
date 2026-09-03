use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SetupIntentsClient {
    pub http_client: HttpClient,
}

impl SetupIntentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of setup intents for a company, with optional filtering by creation date. A setup intent securely collects and stores a member's payment method for future use without charging them immediately.
    ///
    /// Required permissions:
    /// - `payment:setup_intent:read`
    /// - `member:basic:read`
    /// - `member:email:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `created_before` - Only return setup intents created before this timestamp.
    /// * `created_after` - Only return setup intents created after this timestamp.
    /// * `account_id` - The unique identifier of the company to list setup intents for.
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
    ///         .setup_intents
    ///         .list(
    ///             &SetupIntentsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 after: None,
    ///                 before: None,
    ///                 direction: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SetupIntentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSetupIntentsResponse, ApiError> {
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
                "setup_intents",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .serialize("direction", request.direction.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Save a buyer's payment method for later without charging it. Provide a confirmation token for a method the buyer just supplied, or an existing payment method to re-verify. The buyer may still have a step to complete — 3D Secure, a hosted enrollment, linking a bank account — so poll the setup intent's status endpoint for what to do next.
    ///
    /// Required permissions:
    /// - `payment:charge`
    /// - `member:basic:read`
    /// - `member:email:read`
    ///
    /// # Arguments
    ///
    /// * `request` - Parameters for CreateSetupIntent
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
    ///         .setup_intents
    ///         .create(
    ///             &CreateSetupIntentsRequestBody::CreateSetupIntentsRequestBodyConfirmationToken(
    ///                 CreateSetupIntentsRequestBodyConfirmationToken {
    ///                     account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                     confirmation_token: "ctok_xxxxxxxxxxxxxx".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateSetupIntentsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CreateSetupIntentsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "setup_intents",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing setup intent.
    ///
    /// Required permissions:
    /// - `payment:setup_intent:read`
    /// - `member:basic:read`
    /// - `member:email:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the setup intent.
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
    ///         .setup_intents
    ///         .retrieve(&"sint_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SetupIntent, ApiError> {
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
                &format!("setup_intents/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Changes where the buyer lands after completing an off-site step, up until they return. Accepts either a secret key or the setup's own `client_secret`, so the surface that knows the final destination can set it.
    ///
    /// # Arguments
    ///
    /// * `setup_intent_id` - The unique identifier of the setup intent.
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
    ///         .setup_intents
    ///         .update_return_url(
    ///             &"setup_intent_id".to_string(),
    ///             &UpdateReturnURLSetupIntentsRequest {
    ///                 return_url: "https://shinetime.example/checkout/thanks".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_return_url(
        &self,
        setup_intent_id: &str,
        request: &UpdateReturnUrlSetupIntentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SetupStatus, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("setup_intents/{}/return_url", setup_intent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves how far a setup has got and what the buyer must do next, if anything. Collection runs in the background, so poll this rather than reading the create response. Accepts either a secret key or the setup's own `client_secret`, so the surface collecting the payment method can poll it directly.
    ///
    /// # Arguments
    ///
    /// * `setup_intent_id` - The unique identifier of the setup intent.
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
    ///         .setup_intents
    ///         .retrieve_status(&"setup_intent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve_status(
        &self,
        setup_intent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SetupStatus, ApiError> {
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
                &format!("setup_intents/{}/status", setup_intent_id),
                None,
                None,
                options,
            )
            .await
    }
}
