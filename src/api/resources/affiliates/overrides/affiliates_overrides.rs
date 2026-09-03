use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct OverridesClient {
    pub http_client: HttpClient,
}

impl OverridesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of overrides for an affiliate.
    ///
    /// Required permissions:
    /// - `affiliate:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The affiliate ID.
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
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
    ///         .affiliates
    ///         .overrides
    ///         .list(
    ///             &"aff_xxxxxxxxxxxxxx".to_string(),
    ///             &AffiliatesOverridesListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        id: &str,
        request: &AffiliatesOverridesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOverridesResponse, ApiError> {
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
                &format!("affiliates/{}/overrides", id),
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .serialize("override_type", request.override_type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a commission override for an affiliate.
    ///
    /// Required permissions:
    /// - `affiliate:create`
    ///
    /// # Arguments
    ///
    /// * `id` - The affiliate ID.
    /// * `request` - Parameters for CreateAffiliateOverride
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
    ///         .affiliates
    ///         .overrides
    ///         .create(
    ///             &"aff_xxxxxxxxxxxxxx".to_string(),
    ///             &CreateOverridesRequestBody::Standard {
    ///                 data: CreateOverridesRequestBodyStandard {
    ///                     commission_value: 6.9,
    ///                     id: "id".to_string(),
    ///                     plan_id: "plan_xxxxxxxxxxxxx".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        id: &str,
        request: &CreateOverridesRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CreateOverridesResponse, ApiError> {
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
                &format!("affiliates/{}/overrides", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of a specific affiliate override.
    ///
    /// Required permissions:
    /// - `affiliate:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The affiliate ID.
    /// * `override_id` - The override ID.
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
    ///         .affiliates
    ///         .overrides
    ///         .retrieve(
    ///             &"aff_xxxxxxxxxxxxxx".to_string(),
    ///             &"override_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        override_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveOverridesResponse, ApiError> {
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
                &format!("affiliates/{}/overrides/{}", id, override_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes an affiliate override.
    ///
    /// Required permissions:
    /// - `affiliate:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The affiliate ID.
    /// * `override_id` - The override ID.
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
    ///         .affiliates
    ///         .overrides
    ///         .delete(
    ///             &"aff_xxxxxxxxxxxxxx".to_string(),
    ///             &"override_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        override_id: &str,
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
                &format!("affiliates/{}/overrides/{}", id, override_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing affiliate override.
    ///
    /// Required permissions:
    /// - `affiliate:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The affiliate ID.
    /// * `override_id` - The override ID.
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
    ///         .affiliates
    ///         .overrides
    ///         .update(
    ///             &"aff_xxxxxxxxxxxxxx".to_string(),
    ///             &"override_id".to_string(),
    ///             &UpdateOverridesRequest {
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
        override_id: &str,
        request: &UpdateOverridesRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateOverridesResponse, ApiError> {
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
                &format!("affiliates/{}/overrides/{}", id, override_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
