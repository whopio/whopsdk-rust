use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AdConversionValueRulesClient {
    pub http_client: HttpClient,
}

impl AdConversionValueRulesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List saved rules the caller can read. Filter by business with account_id.
    ///
    /// # Arguments
    ///
    /// * `resource_id` - Campaign, ad group, or ad ID. Return rules covering this item, its ancestors, or its descendants.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
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
    ///         .ad_conversion_value_rules
    ///         .list(
    ///             &AdConversionValueRulesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AdConversionValueRulesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAdConversionValueRulesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "ad_conversion_value_rules",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("platform", request.platform.clone())
                    .string("resource_id", request.resource_id.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create one rule covering every selected target and event combination. Active rules cannot overlap for the same platform and event. Customer prices and Whop revenue stay unchanged.
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
    ///         .ad_conversion_value_rules
    ///         .create(
    ///             &CreateAdConversionValueRulesRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 adjustment_type: CreateAdConversionValueRulesRequestAdjustmentType::Fixed,
    ///                 events: vec![CreateAdConversionValueRulesRequestEventsItem {
    ///                     custom_name: None,
    ///                     event_name: CreateAdConversionValueRulesRequestEventsItemEventName::Purchase,
    ///                 }],
    ///                 targets: vec![CreateAdConversionValueRulesRequestTargetsItem {
    ///                     platform: CreateAdConversionValueRulesRequestTargetsItemPlatform::Tiktok,
    ///                     resource_id: None,
    ///                     scope: CreateAdConversionValueRulesRequestTargetsItemScope::Business,
    ///                 }],
    ///                 fixed_value: None,
    ///                 metadata: None,
    ///                 percentage_change: None,
    ///                 replace_rule_ids: None,
    ///                 status: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAdConversionValueRulesRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdConversionValueRule, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "ad_conversion_value_rules",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

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
    ///         .ad_conversion_value_rules
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdConversionValueRule, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("ad_conversion_value_rules/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Soft-delete a rule and deactivate all its coverage. Preserve its stored settings.
    ///
    /// # Arguments
    ///
    /// * `id` - Conversion value rule ID.
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
    ///         .ad_conversion_value_rules
    ///         .delete(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteAdConversionValueRulesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("ad_conversion_value_rules/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Edit a rule without changing its status. Supplied targets or events replace that selection in full. Omitted fields stay unchanged. All changes succeed or fail together.
    ///
    /// # Arguments
    ///
    /// * `id` - Conversion value rule ID.
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
    ///         .ad_conversion_value_rules
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAdConversionValueRulesRequest {
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
        request: &UpdateAdConversionValueRulesRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdConversionValueRule, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("ad_conversion_value_rules/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pause the rule across all selected targets and events.
    ///
    /// # Arguments
    ///
    /// * `id` - Conversion value rule ID.
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
    ///         .ad_conversion_value_rules
    ///         .pause(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn pause(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdConversionValueRule, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ad_conversion_value_rules/{}/pause", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resume the rule and automatically replace overlapping selections in the same transaction. Other selections keep their values, and broader rules remain as defaults. Rules with no remaining selections are paused. Resuming an already-active rule makes no changes.
    ///
    /// # Arguments
    ///
    /// * `id` - Conversion value rule ID.
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
    ///         .ad_conversion_value_rules
    ///         .unpause(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn unpause(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdConversionValueRule, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ad_conversion_value_rules/{}/unpause", id),
                None,
                None,
                options,
            )
            .await
    }
}
