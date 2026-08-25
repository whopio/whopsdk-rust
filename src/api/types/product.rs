pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    /// Account that sells this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<HashMap<String, serde_json::Value>>,
    /// When the product was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Call-to-action button label shown on the product purchase page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cta: Option<ProductCustomCta>,
    /// URL the call-to-action button links to instead of checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cta_url: Option<String>,
    /// Custom text label on customer's bank statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_statement_descriptor: Option<String>,
    /// Buyable plan to show and check out with. The configured default when that plan is buyable, otherwise the first buyable plan in product-page order. `null` when none is buyable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_plan: Option<ProductPublicPlan>,
    /// Written description displayed on the product page. `null` if none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// External identifier stored on the product for your own reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identifier: Option<String>,
    #[serde(default)]
    pub gallery_images: Vec<ProductGalleryImage>,
    /// Commission rate affiliates earn through the global affiliate program.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub global_affiliate_percentage: Option<f64>,
    /// Enrollment status in the global affiliate program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_affiliate_status: Option<ProductGlobalAffiliateStatus>,
    /// Short marketing headline displayed on product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// Product ID, prefixed `prod_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub labels: Vec<String>,
    /// Listing state on the whop.com marketplace. `pending_review` means submitted and awaiting review; `live_marketplace` means approved and discoverable.
    pub marketplace_status: ProductMarketplaceStatus,
    /// Commission rate members earn through the member affiliate program.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub member_affiliate_percentage: Option<f64>,
    /// Enrollment status in the member affiliate program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_affiliate_status: Option<ProductMemberAffiliateStatus>,
    /// Active memberships for this product; 0 if public member counts are disabled.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub member_count: f64,
    /// Custom key-value pairs stored on the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// User who owns the account selling this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user: Option<HashMap<String, serde_json::Value>>,
    /// Tax classification code for this product, or `null` if no tax code is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code: Option<HashMap<String, serde_json::Value>>,
    /// Published customer reviews for this product.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub published_reviews_count: f64,
    /// URL slug for the product's public link.
    #[serde(default)]
    pub route: String,
    /// Product display name shown to customers.
    #[serde(default)]
    pub title: String,
    /// When the product was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// Whether the product has been verified by Whop.
    #[serde(default)]
    pub verified: bool,
    /// Whether the product is publicly visible, hidden, or archived.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl Product {
    pub fn builder() -> ProductBuilder {
        <ProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductBuilder {
    account: Option<HashMap<String, serde_json::Value>>,
    created_at: Option<String>,
    custom_cta: Option<ProductCustomCta>,
    custom_cta_url: Option<String>,
    custom_statement_descriptor: Option<String>,
    default_plan: Option<ProductPublicPlan>,
    description: Option<String>,
    external_identifier: Option<String>,
    gallery_images: Option<Vec<ProductGalleryImage>>,
    global_affiliate_percentage: Option<f64>,
    global_affiliate_status: Option<ProductGlobalAffiliateStatus>,
    headline: Option<String>,
    id: Option<String>,
    labels: Option<Vec<String>>,
    marketplace_status: Option<ProductMarketplaceStatus>,
    member_affiliate_percentage: Option<f64>,
    member_affiliate_status: Option<ProductMemberAffiliateStatus>,
    member_count: Option<f64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    owner_user: Option<HashMap<String, serde_json::Value>>,
    product_tax_code: Option<HashMap<String, serde_json::Value>>,
    published_reviews_count: Option<f64>,
    route: Option<String>,
    title: Option<String>,
    updated_at: Option<String>,
    verified: Option<bool>,
    visibility: Option<String>,
}

impl ProductBuilder {
    pub fn account(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.account = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn custom_cta(mut self, value: ProductCustomCta) -> Self {
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

    pub fn default_plan(mut self, value: ProductPublicPlan) -> Self {
        self.default_plan = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn external_identifier(mut self, value: impl Into<String>) -> Self {
        self.external_identifier = Some(value.into());
        self
    }

    pub fn gallery_images(mut self, value: Vec<ProductGalleryImage>) -> Self {
        self.gallery_images = Some(value);
        self
    }

    pub fn global_affiliate_percentage(mut self, value: f64) -> Self {
        self.global_affiliate_percentage = Some(value);
        self
    }

    pub fn global_affiliate_status(mut self, value: ProductGlobalAffiliateStatus) -> Self {
        self.global_affiliate_status = Some(value);
        self
    }

    pub fn headline(mut self, value: impl Into<String>) -> Self {
        self.headline = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn marketplace_status(mut self, value: ProductMarketplaceStatus) -> Self {
        self.marketplace_status = Some(value);
        self
    }

    pub fn member_affiliate_percentage(mut self, value: f64) -> Self {
        self.member_affiliate_percentage = Some(value);
        self
    }

    pub fn member_affiliate_status(mut self, value: ProductMemberAffiliateStatus) -> Self {
        self.member_affiliate_status = Some(value);
        self
    }

    pub fn member_count(mut self, value: f64) -> Self {
        self.member_count = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn owner_user(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.owner_user = Some(value);
        self
    }

    pub fn product_tax_code(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.product_tax_code = Some(value);
        self
    }

    pub fn published_reviews_count(mut self, value: f64) -> Self {
        self.published_reviews_count = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn visibility(mut self, value: impl Into<String>) -> Self {
        self.visibility = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Product`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ProductBuilder::created_at)
    /// - [`gallery_images`](ProductBuilder::gallery_images)
    /// - [`id`](ProductBuilder::id)
    /// - [`labels`](ProductBuilder::labels)
    /// - [`marketplace_status`](ProductBuilder::marketplace_status)
    /// - [`member_count`](ProductBuilder::member_count)
    /// - [`published_reviews_count`](ProductBuilder::published_reviews_count)
    /// - [`route`](ProductBuilder::route)
    /// - [`title`](ProductBuilder::title)
    /// - [`updated_at`](ProductBuilder::updated_at)
    /// - [`verified`](ProductBuilder::verified)
    pub fn build(self) -> Result<Product, BuildError> {
        Ok(Product {
            account: self.account,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            custom_cta: self.custom_cta,
            custom_cta_url: self.custom_cta_url,
            custom_statement_descriptor: self.custom_statement_descriptor,
            default_plan: self.default_plan,
            description: self.description,
            external_identifier: self.external_identifier,
            gallery_images: self
                .gallery_images
                .ok_or_else(|| BuildError::missing_field("gallery_images"))?,
            global_affiliate_percentage: self.global_affiliate_percentage,
            global_affiliate_status: self.global_affiliate_status,
            headline: self.headline,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            labels: self
                .labels
                .ok_or_else(|| BuildError::missing_field("labels"))?,
            marketplace_status: self
                .marketplace_status
                .ok_or_else(|| BuildError::missing_field("marketplace_status"))?,
            member_affiliate_percentage: self.member_affiliate_percentage,
            member_affiliate_status: self.member_affiliate_status,
            member_count: self
                .member_count
                .ok_or_else(|| BuildError::missing_field("member_count"))?,
            metadata: self.metadata,
            owner_user: self.owner_user,
            product_tax_code: self.product_tax_code,
            published_reviews_count: self
                .published_reviews_count
                .ok_or_else(|| BuildError::missing_field("published_reviews_count"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
            visibility: self.visibility,
        })
    }
}
