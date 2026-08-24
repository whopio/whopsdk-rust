pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateProductsRequest {
    /// The unique identifier of the account to create this product for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Whether to collect a shipping address at checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_shipping_address: Option<bool>,
    /// The call-to-action button label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cta: Option<CreateProductsRequestCustomCta>,
    /// A URL the call-to-action button links to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cta_url: Option<String>,
    /// Custom bank statement descriptor. Must start with WHOP*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_statement_descriptor: Option<String>,
    /// A written description displayed on the product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The commission rate affiliates earn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_affiliate_percentage: Option<f64>,
    /// The enrollment status in the global affiliate program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_affiliate_status: Option<CreateProductsRequestGlobalAffiliateStatus>,
    /// A short marketing headline for the product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// Labels used to group products into collections. Stored lowercased and de-duplicated. Maximum 20 labels, 50 characters each.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// The commission rate members earn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_affiliate_percentage: Option<f64>,
    /// The enrollment status in the member affiliate program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_affiliate_status: Option<CreateProductsRequestMemberAffiliateStatus>,
    /// Custom key-value pairs to store on the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The unique identifier of the tax classification code. See the available [product categories](https://docs.numeral.com/essentials/product-categories).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code_id: Option<String>,
    /// A URL to redirect the customer to after purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_purchase_url: Option<String>,
    /// The URL slug for the product's public link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Whether to send an automated welcome message via support chat when a user joins this product. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_welcome_message: Option<bool>,
    /// The display name of the product. Maximum 80 characters.
    #[serde(default)]
    pub title: String,
    /// Whether the product is visible to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl CreateProductsRequest {
    pub fn builder() -> CreateProductsRequestBuilder {
        <CreateProductsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductsRequestBuilder {
    account_id: Option<String>,
    collect_shipping_address: Option<bool>,
    custom_cta: Option<CreateProductsRequestCustomCta>,
    custom_cta_url: Option<String>,
    custom_statement_descriptor: Option<String>,
    description: Option<String>,
    global_affiliate_percentage: Option<f64>,
    global_affiliate_status: Option<CreateProductsRequestGlobalAffiliateStatus>,
    headline: Option<String>,
    labels: Option<Vec<String>>,
    member_affiliate_percentage: Option<f64>,
    member_affiliate_status: Option<CreateProductsRequestMemberAffiliateStatus>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    product_tax_code_id: Option<String>,
    redirect_purchase_url: Option<String>,
    route: Option<String>,
    send_welcome_message: Option<bool>,
    title: Option<String>,
    visibility: Option<String>,
}

impl CreateProductsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn collect_shipping_address(mut self, value: bool) -> Self {
        self.collect_shipping_address = Some(value);
        self
    }

    pub fn custom_cta(mut self, value: CreateProductsRequestCustomCta) -> Self {
        self.custom_cta = Some(value);
        self
    }

    pub fn custom_cta_url(mut self, value: impl Into<String>) -> Self {
        self.custom_cta_url = Some(value.into());
        self
    }

    pub fn custom_statement_descriptor(mut self, value: impl Into<String>) -> Self {
        self.custom_statement_descriptor = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn global_affiliate_percentage(mut self, value: f64) -> Self {
        self.global_affiliate_percentage = Some(value);
        self
    }

    pub fn global_affiliate_status(
        mut self,
        value: CreateProductsRequestGlobalAffiliateStatus,
    ) -> Self {
        self.global_affiliate_status = Some(value);
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

    pub fn member_affiliate_percentage(mut self, value: f64) -> Self {
        self.member_affiliate_percentage = Some(value);
        self
    }

    pub fn member_affiliate_status(
        mut self,
        value: CreateProductsRequestMemberAffiliateStatus,
    ) -> Self {
        self.member_affiliate_status = Some(value);
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

    pub fn redirect_purchase_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_purchase_url = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateProductsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateProductsRequestBuilder::title)
    pub fn build(self) -> Result<CreateProductsRequest, BuildError> {
        Ok(CreateProductsRequest {
            account_id: self.account_id,
            collect_shipping_address: self.collect_shipping_address,
            custom_cta: self.custom_cta,
            custom_cta_url: self.custom_cta_url,
            custom_statement_descriptor: self.custom_statement_descriptor,
            description: self.description,
            global_affiliate_percentage: self.global_affiliate_percentage,
            global_affiliate_status: self.global_affiliate_status,
            headline: self.headline,
            labels: self.labels,
            member_affiliate_percentage: self.member_affiliate_percentage,
            member_affiliate_status: self.member_affiliate_status,
            metadata: self.metadata,
            product_tax_code_id: self.product_tax_code_id,
            redirect_purchase_url: self.redirect_purchase_url,
            route: self.route,
            send_welcome_message: self.send_welcome_message,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            visibility: self.visibility,
        })
    }
}
