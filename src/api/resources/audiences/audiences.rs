use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AudiencesClient {
    pub http_client: HttpClient,
}

impl AudiencesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists uploaded customer-list audiences for an account. Pass `audience_id` to return a specific audience.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account ID, prefixed `biz_`.
    /// * `audience_id` - Audience ID, prefixed `adaud_`, used to filter the response to one audience.
    /// * `audience_type` - Filter by audience type: `custom` (uploaded lists) or `lookalike`.
    /// * `source_type` - Filter by member source: `csv_upload` (uploaded lists) or `people_filter` (automatic audiences built from saved People filters).
    /// * `first` - Number of audiences to return. Defaults to 20; maximum 100.
    /// * `after` - Cursor for the next page of audiences.
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
    ///         .audiences
    ///         .list(
    ///             &AudiencesListQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 audience_id: None,
    ///                 audience_type: None,
    ///                 source_type: None,
    ///                 first: None,
    ///                 after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AudiencesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAudiencesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "audiences",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("audience_id", request.audience_id.clone())
                    .serialize("audience_type", request.audience_type.clone())
                    .serialize("source_type", request.source_type.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an audience. Default (`audience_type` omitted or `custom`): creates one audience from an uploaded customer identity CSV file (`name`, `column_mapping`, and `file_id` required) and starts processing it; responds with the audience object. With `filters`: creates an audience from saved People filters (`name` required) — membership is built from the account's People data, and `auto_refresh` decides whether it keeps tracking the filters or keeps whoever matched at creation. With `audience_type: lookalike`: creates a ladder of Meta lookalike audiences from an existing ready custom audience (`source_audience_id`, `count`, and `percentage` required) — `count` equal similarity bands slicing the top `percentage`% (3 audiences at 6% = 0–2%, 2–4%, 4–6%), each returned as its own audience in a `{ data: [...] }` envelope.
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
    ///         .audiences
    ///         .create(
    ///             &CreateAudiencesRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 audience_type: None,
    ///                 auto_refresh: None,
    ///                 column_mapping: None,
    ///                 count: None,
    ///                 file_id: None,
    ///                 filters: None,
    ///                 name: None,
    ///                 percentage: None,
    ///                 source_audience_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAudiencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateAudiencesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "audiences",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an audience so it is no longer available for targeting.
    ///
    /// # Arguments
    ///
    /// * `id` - Audience ID, prefixed `adaud_`.
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
    ///     client.audiences.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteAudiencesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("audiences/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Renames an audience. For an audience built from People filters that keeps itself up to date, pass `filters` to replace them, which rebuilds membership immediately. Whether an audience auto refreshes is set when it is created.
    ///
    /// # Arguments
    ///
    /// * `id` - Audience ID, prefixed `adaud_`.
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
    ///         .audiences
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAudiencesRequest {
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
        request: &UpdateAudiencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Audience, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("audiences/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds users from a new CSV file to an existing uploaded custom audience. The file uses the audience's saved column mapping, processing happens in the background, and existing audience members remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - Audience ID, prefixed `adaud_`.
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
    ///         .audiences
    ///         .add_people(
    ///             &"id".to_string(),
    ///             &AddPeopleAudiencesRequest {
    ///                 file_id: "file_xxxxxxxxxxxxxx".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_people(
        &self,
        id: &str,
        request: &AddPeopleAudiencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Audience, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("audiences/{}/add_people", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
