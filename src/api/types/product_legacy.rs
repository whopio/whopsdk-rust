pub use crate::prelude::*;

/// A product is a digital good or service sold on Whop. Products contain plans for pricing and experiences for content delivery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductLegacy {
    /// The company this product belongs to.
    #[serde(default)]
    pub company: ProductLegacyCompany,
    /// The datetime the product was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Call-to-action button label shown on the product purchase page.
    pub custom_cta: CustomCtas,
    /// An optional URL that the call-to-action button links to instead of the default checkout flow. Null if no custom URL is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cta_url: Option<String>,
    /// Custom bank statement descriptor for product purchases. Maximum 22 characters, including required `WHOP*` prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_statement_descriptor: Option<String>,
    /// A brief summary of what the product offers, displayed on product pages and search results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// External identifier for the product. Providing it on a product creation endpoint updates the existing product with this identifier instead of creating a new one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identifier: Option<String>,
    /// The gallery images for this product, ordered by position.
    #[serde(default)]
    pub gallery_images: Vec<ProductLegacyGalleryImagesItem>,
    /// Marketplace affiliate commission percentage for this product, or `null` if program is inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub global_affiliate_percentage: Option<f64>,
    /// The enrollment status of this product in the Whop marketplace global affiliate program.
    pub global_affiliate_status: GlobalAffiliateStatuses,
    /// A short marketing headline displayed prominently on the product's product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// Member referral commission percentage for this product, or `null` if program is inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub member_affiliate_percentage: Option<f64>,
    /// The enrollment status of this product in the member affiliate program.
    pub member_affiliate_status: GlobalAffiliateStatuses,
    /// Active memberships for this product. Returns `0` if the account has disabled public member counts.
    #[serde(default)]
    pub member_count: i64,
    /// Custom key-value pairs stored on the product and included in payment and membership webhook payloads. Max 50 keys, 100 characters per key, 500 characters per string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The user who owns the company that sells this product.
    #[serde(default)]
    pub owner_user: ProductLegacyOwnerUser,
    /// The tax classification code applied to purchases of this product for sales tax calculation. Null if no tax code is assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code: Option<ProductLegacyProductTaxCode>,
    /// The total number of published customer reviews for this product's company.
    #[serde(default)]
    pub published_reviews_count: i64,
    /// URL slug in the product's public link, e.g. `pickaxe-analytics` in whop.com/company/pickaxe-analytics.
    #[serde(default)]
    pub route: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
    /// The datetime the product was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Whether this company has been verified by Whop's trust and safety team.
    #[serde(default)]
    pub verified: bool,
    /// Controls whether the product is visible to customers. When set to 'hidden', the product is only accessible via direct link.
    pub visibility: Visibility,
}

impl ProductLegacy {
    pub fn builder() -> ProductLegacyBuilder {
        <ProductLegacyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductLegacyBuilder {
    company: Option<ProductLegacyCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    custom_cta: Option<CustomCtas>,
    custom_cta_url: Option<String>,
    custom_statement_descriptor: Option<String>,
    description: Option<String>,
    external_identifier: Option<String>,
    gallery_images: Option<Vec<ProductLegacyGalleryImagesItem>>,
    global_affiliate_percentage: Option<f64>,
    global_affiliate_status: Option<GlobalAffiliateStatuses>,
    headline: Option<String>,
    id: Option<String>,
    member_affiliate_percentage: Option<f64>,
    member_affiliate_status: Option<GlobalAffiliateStatuses>,
    member_count: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    owner_user: Option<ProductLegacyOwnerUser>,
    product_tax_code: Option<ProductLegacyProductTaxCode>,
    published_reviews_count: Option<i64>,
    route: Option<String>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    verified: Option<bool>,
    visibility: Option<Visibility>,
}

impl ProductLegacyBuilder {
    pub fn company(mut self, value: ProductLegacyCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn custom_cta(mut self, value: CustomCtas) -> Self {
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

    pub fn external_identifier(mut self, value: impl Into<String>) -> Self {
        self.external_identifier = Some(value.into());
        self
    }

    pub fn gallery_images(mut self, value: Vec<ProductLegacyGalleryImagesItem>) -> Self {
        self.gallery_images = Some(value);
        self
    }

    pub fn global_affiliate_percentage(mut self, value: f64) -> Self {
        self.global_affiliate_percentage = Some(value);
        self
    }

    pub fn global_affiliate_status(mut self, value: GlobalAffiliateStatuses) -> Self {
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

    pub fn member_affiliate_percentage(mut self, value: f64) -> Self {
        self.member_affiliate_percentage = Some(value);
        self
    }

    pub fn member_affiliate_status(mut self, value: GlobalAffiliateStatuses) -> Self {
        self.member_affiliate_status = Some(value);
        self
    }

    pub fn member_count(mut self, value: i64) -> Self {
        self.member_count = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn owner_user(mut self, value: ProductLegacyOwnerUser) -> Self {
        self.owner_user = Some(value);
        self
    }

    pub fn product_tax_code(mut self, value: ProductLegacyProductTaxCode) -> Self {
        self.product_tax_code = Some(value);
        self
    }

    pub fn published_reviews_count(mut self, value: i64) -> Self {
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn visibility(mut self, value: Visibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductLegacy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company`](ProductLegacyBuilder::company)
    /// - [`created_at`](ProductLegacyBuilder::created_at)
    /// - [`custom_cta`](ProductLegacyBuilder::custom_cta)
    /// - [`gallery_images`](ProductLegacyBuilder::gallery_images)
    /// - [`global_affiliate_status`](ProductLegacyBuilder::global_affiliate_status)
    /// - [`id`](ProductLegacyBuilder::id)
    /// - [`member_affiliate_status`](ProductLegacyBuilder::member_affiliate_status)
    /// - [`member_count`](ProductLegacyBuilder::member_count)
    /// - [`owner_user`](ProductLegacyBuilder::owner_user)
    /// - [`published_reviews_count`](ProductLegacyBuilder::published_reviews_count)
    /// - [`route`](ProductLegacyBuilder::route)
    /// - [`title`](ProductLegacyBuilder::title)
    /// - [`updated_at`](ProductLegacyBuilder::updated_at)
    /// - [`verified`](ProductLegacyBuilder::verified)
    /// - [`visibility`](ProductLegacyBuilder::visibility)
    pub fn build(self) -> Result<ProductLegacy, BuildError> {
        Ok(ProductLegacy {
            company: self
                .company
                .ok_or_else(|| BuildError::missing_field("company"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            custom_cta: self
                .custom_cta
                .ok_or_else(|| BuildError::missing_field("custom_cta"))?,
            custom_cta_url: self.custom_cta_url,
            custom_statement_descriptor: self.custom_statement_descriptor,
            description: self.description,
            external_identifier: self.external_identifier,
            gallery_images: self
                .gallery_images
                .ok_or_else(|| BuildError::missing_field("gallery_images"))?,
            global_affiliate_percentage: self.global_affiliate_percentage,
            global_affiliate_status: self
                .global_affiliate_status
                .ok_or_else(|| BuildError::missing_field("global_affiliate_status"))?,
            headline: self.headline,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            member_affiliate_percentage: self.member_affiliate_percentage,
            member_affiliate_status: self
                .member_affiliate_status
                .ok_or_else(|| BuildError::missing_field("member_affiliate_status"))?,
            member_count: self
                .member_count
                .ok_or_else(|| BuildError::missing_field("member_count"))?,
            metadata: self.metadata,
            owner_user: self
                .owner_user
                .ok_or_else(|| BuildError::missing_field("owner_user"))?,
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
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
