use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ProductsClient {
    pub http_client: HttpClient,
}

impl ProductsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of products belonging to an account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The unique identifier of the account to list products for.
    /// * `visibilities` - Filter to only products matching these visibility states.
    /// * `access_pass_types` - Filter to only products matching these types.
    /// * `labels` - Filter to only products carrying all of these labels. Labels are matched lowercased.
    /// * `direction` - The sort direction for results. Defaults to descending.
    /// * `order` - The field to sort results by. Defaults to created_at.
    /// * `first` - The number of products to return (default and max 100).
    /// * `after` - A cursor; returns products after this position.
    /// * `last` - The number of products to return from the end of the range.
    /// * `before` - A cursor; returns products before this position.
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
    ///         .products
    ///         .list(
    ///             &ProductsListQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 visibilities: vec![Some("visible".to_string())],
    ///                 access_pass_types: vec![Some("regular".to_string())],
    ///                 labels: vec![],
    ///                 direction: None,
    ///                 order: None,
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
        request: &ProductsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProductsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "products",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string_array("visibilities", request.visibilities.clone())
                    .string_array("access_pass_types", request.access_pass_types.clone())
                    .string_array("labels", request.labels.clone())
                    .serialize("direction", request.direction.clone())
                    .string("order", request.order.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new product for an account.
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
    ///         .products
    ///         .create(
    ///             &CreateProductsRequest {
    ///                 title: "Interior Deep Clean".to_string(),
    ///                 account_id: None,
    ///                 collect_shipping_address: None,
    ///                 custom_cta: None,
    ///                 custom_cta_url: None,
    ///                 custom_statement_descriptor: None,
    ///                 description: None,
    ///                 global_affiliate_percentage: None,
    ///                 global_affiliate_status: None,
    ///                 headline: None,
    ///                 labels: None,
    ///                 member_affiliate_percentage: None,
    ///                 member_affiliate_status: None,
    ///                 metadata: None,
    ///                 product_tax_code_id: None,
    ///                 redirect_purchase_url: None,
    ///                 route: None,
    ///                 send_welcome_message: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateProductsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "products",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing product. This endpoint is publicly accessible.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product.
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
    ///     client.products.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("products/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a product. Only products with no memberships, entries, reviews, or invoices can be deleted.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product.
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
    ///     client.products.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteProductsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("products/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing product.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product.
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
    ///         .products
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateProductsRequest {
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
        request: &UpdateProductsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("products/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Submits a product to the whop.com marketplace for review. The product moves to `pending_review`; a Whop reviewer approves it before it goes live.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product, prefixed `prod_`.
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
    ///     client.products.publish(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn publish(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("products/{}/publish", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes a product from the whop.com marketplace. The product moves to `not_available`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product, prefixed `prod_`.
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
    ///     client.products.unpublish(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unpublish(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Product, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("products/{}/unpublish", id),
                None,
                None,
                options,
            )
            .await
    }
}
