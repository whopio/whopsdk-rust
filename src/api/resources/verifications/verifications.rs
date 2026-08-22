use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct VerificationsClient {
    pub http_client: HttpClient,
}

impl VerificationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns verifications for an account, including their status and any required actions.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account or user ID whose verifications you want to list. Use a `biz_` account ID, or the caller's `user_` ID for personal verifications.
    /// * `order` - Field used to sort returned verifications.
    /// * `direction` - Sort direction for returned verifications.
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
    ///         .verifications
    ///         .list(
    ///             &VerificationsListQueryRequest {
    ///                 account_id: "account_id".to_string(),
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
        request: &VerificationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListVerificationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "verifications",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Starts a hosted verification session for an account or user, or returns the active session when one already exists. Any fields you include in the request body are used to prefill the session. Send `documents` (with `document_type`) to instead verify the person from identity documents included in this request — no hosted session involved. Send `share_token` to reuse a verification another Sumsub account has already completed for this person, instead of verifying them again. If the account already has an `approved` verification the request is rejected; unlink it first to start a new one.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account or user ID whose identity you want to verify. Use a `biz_` account ID for account verifications, or the caller's `user_` ID for personal verification.
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
    ///         .verifications
    ///         .create(
    ///             &CreateRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 body: CreateVerificationsRequestBody::Individual {
    ///                     data: CreateVerificationsRequestBodyIndividual {
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateVerificationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "verifications",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns verifications for an account, including their status and any required actions.
    ///
    /// # Arguments
    ///
    /// * `id` - Verification profile ID, prefixed `idpf_`.
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
    ///     client.verifications.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveVerificationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("verifications/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates editable profile details or submits answers for items returned in `requested_information`. Once a verification is `approved` its profile details are locked and can no longer be edited.
    ///
    /// # Arguments
    ///
    /// * `id` - Verification profile ID, prefixed `idpf_`.
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
    ///         .verifications
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateVerificationsRequestBody::UpdateVerificationsRequestBodyPersonalAddress(
    ///                 UpdateVerificationsRequestBodyPersonalAddress {
    ///                     ..Default::default()
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateVerificationsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<UpdateVerificationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("verifications/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
