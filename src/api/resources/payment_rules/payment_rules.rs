use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaymentRulesClient {
    pub http_client: HttpClient,
}

impl PaymentRulesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
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
    ///         .payment_rules
    ///         .list(
    ///             &PaymentRulesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PaymentRulesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPaymentRulesResponse, ApiError> {
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
                "payment_rules",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("action", request.action.clone())
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
    ///         .payment_rules
    ///         .create(
    ///             &CreatePaymentRulesRequest {
    ///                 action: CreatePaymentRulesRequestAction::Allow,
    ///                 conditions: CreatePaymentRulesRequestConditions {
    ///                     all: vec![CreatePaymentRulesRequestConditionsAllItem {
    ///                         field: CreatePaymentRulesRequestConditionsAllItemField::RiskScore,
    ///                         operator: CreatePaymentRulesRequestConditionsAllItemOperator::Gte,
    ///                         value: PaymentRuleConditionValue::Integer(70),
    ///                     }],
    ///                     ..Default::default()
    ///                 },
    ///                 name: "Review risky cards".to_string(),
    ///                 account_id: None,
    ///                 metadata: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePaymentRulesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentRule, ApiError> {
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
                "payment_rules",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Small and returned in full on one page.
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
    ///     client.payment_rules.list_fields(None).await;
    /// }
    /// ```
    pub async fn list_fields(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListFieldsPaymentRulesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, "payment_rules/fields", None, None, options)
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
    ///     client.payment_rules.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentRule, ApiError> {
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
                &format!("payment_rules/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// The rule stops applying to new payments and is kept, so the payments it already decided still name it.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment rule ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///     client.payment_rules.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(&self, id: &str, options: Option<RequestOptions>) -> Result<(), ApiError> {
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
                &format!("payment_rules/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Changes the rule's name or metadata, keeping its ID and everything recorded against it. What the rule *does* is fixed once created, so the payments it decided keep naming the rule that decided them; use replace to change that.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment rule ID.
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
    ///         .payment_rules
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdatePaymentRulesRequest {
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
        request: &UpdatePaymentRulesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentRule, ApiError> {
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
                &format!("payment_rules/{}", id),
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
    ///     client.payment_rules.activate(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn activate(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentRule, ApiError> {
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
                &format!("payment_rules/{}/activate", id),
                None,
                None,
                options,
            )
            .await
    }

    /// The rule stops applying to new payments. It keeps its ID and can be activated again.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment rule ID.
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
    ///         .payment_rules
    ///         .deactivate(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn deactivate(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentRule, ApiError> {
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
                &format!("payment_rules/{}/deactivate", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes this rule and creates its successor in one step. The successor carries a new ID and the metadata of the rule it replaced,.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment rule ID.
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
    ///         .payment_rules
    ///         .replace(
    ///             &"id".to_string(),
    ///             &ReplacePaymentRulesRequest {
    ///                 action: ReplacePaymentRulesRequestAction::Allow,
    ///                 conditions: ReplacePaymentRulesRequestConditions {
    ///                     all: vec![ReplacePaymentRulesRequestConditionsAllItem {
    ///                         field: ReplacePaymentRulesRequestConditionsAllItemField::RiskScore,
    ///                         operator: ReplacePaymentRulesRequestConditionsAllItemOperator::Gte,
    ///                         value: PaymentRuleConditionValue::Integer(70),
    ///                     }],
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn replace(
        &self,
        id: &str,
        request: &ReplacePaymentRulesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentRule, ApiError> {
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
                &format!("payment_rules/{}/replace", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
