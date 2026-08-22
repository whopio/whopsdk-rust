pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateProductsRequest {
    /// A wide image for the product, shown on the product page and on listing cards. Pass `{ id }` for an existing attachment or `{ direct_upload_id }` for a completed direct upload; `null` removes it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<UpdateProductsRequestBannerImage>,
    /// A written description displayed on the product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A short marketing headline for the product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// Labels used to group products into collections. Replaces the existing labels. Send an empty array to clear them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Custom key-value pairs to store on the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The unique identifier of the tax classification code. See the available [product categories](https://docs.numeral.com/essentials/product-categories).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code_id: Option<String>,
    /// Whether to send an automated welcome message via support chat when a user joins this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_welcome_message: Option<bool>,
    /// The display name of the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether the product is visible to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl UpdateProductsRequest {
    pub fn builder() -> UpdateProductsRequestBuilder {
        <UpdateProductsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateProductsRequestBuilder {
    banner_image: Option<UpdateProductsRequestBannerImage>,
    description: Option<String>,
    headline: Option<String>,
    labels: Option<Vec<String>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    product_tax_code_id: Option<String>,
    send_welcome_message: Option<bool>,
    title: Option<String>,
    visibility: Option<String>,
}

impl UpdateProductsRequestBuilder {
    pub fn banner_image(mut self, value: UpdateProductsRequestBannerImage) -> Self {
        self.banner_image = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn headline(mut self, value: impl Into<String>) -> Self {
        self.headline = Some(value.into());
        self
    }

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn product_tax_code_id(mut self, value: impl Into<String>) -> Self {
        self.product_tax_code_id = Some(value.into());
        self
    }

    pub fn send_welcome_message(mut self, value: bool) -> Self {
        self.send_welcome_message = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: impl Into<String>) -> Self {
        self.visibility = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateProductsRequest`].
    pub fn build(self) -> Result<UpdateProductsRequest, BuildError> {
        Ok(UpdateProductsRequest {
            banner_image: self.banner_image,
            description: self.description,
            headline: self.headline,
            labels: self.labels,
            metadata: self.metadata,
            product_tax_code_id: self.product_tax_code_id,
            send_welcome_message: self.send_welcome_message,
            title: self.title,
            visibility: self.visibility,
        })
    }
}
