pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProductListItem {
    /// Account that sells this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<HashMap<String, serde_json::Value>>,
    /// Average star rating across published reviews for this product, from `1.0` to `5.0`. Returns `0.0` when no published-review rating is available.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub average_review_rating: f64,
    /// When the product was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Buyable variant to show and check out with. The configured default when that variant is buyable, otherwise the first buyable variant in product-page order. `null` when none is buyable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_variant: Option<ProductPublicVariant>,
    /// Written description displayed on the product page. `null` if none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub experiences: Vec<ProductExperience>,
    /// External identifier stored on the product for your own reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identifier: Option<String>,
    #[serde(default)]
    pub gallery_images: Vec<ProductGalleryImage>,
    /// Short marketing headline displayed on product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// Product ID, prefixed `prod_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub labels: Vec<String>,
    /// Logo image URL shown on product pages and in search results. `null` when the product has no logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Active memberships for this product; 0 if public member counts are disabled.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub member_count: f64,
    /// Custom key-value pairs stored on the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
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
    /// The option set the product's variants span, as a map of attribute name to the values in use, e.g. `{"color": ["Blue", "Red"], "size": ["S", "M", "L"]}`. Derived from the visible, non-invoice variants that carry `attributes`: keys alphabetical, values in the order the variants were created. Read-only. `null` when the product has no variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_attributes: Option<HashMap<String, serde_json::Value>>,
    /// Whether the product has been verified by Whop.
    #[serde(default)]
    pub verified: bool,
    /// Whether the product is publicly visible, hidden, or archived.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl ProductListItem {
    pub fn builder() -> ProductListItemBuilder {
        <ProductListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductListItemBuilder {
    account: Option<HashMap<String, serde_json::Value>>,
    average_review_rating: Option<f64>,
    created_at: Option<String>,
    default_variant: Option<ProductPublicVariant>,
    description: Option<String>,
    experiences: Option<Vec<ProductExperience>>,
    external_identifier: Option<String>,
    gallery_images: Option<Vec<ProductGalleryImage>>,
    headline: Option<String>,
    id: Option<String>,
    labels: Option<Vec<String>>,
    logo_url: Option<String>,
    member_count: Option<f64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    published_reviews_count: Option<f64>,
    route: Option<String>,
    title: Option<String>,
    updated_at: Option<String>,
    variant_attributes: Option<HashMap<String, serde_json::Value>>,
    verified: Option<bool>,
    visibility: Option<String>,
}

impl ProductListItemBuilder {
    pub fn account(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.account = Some(value);
        self
    }

    pub fn average_review_rating(mut self, value: f64) -> Self {
        self.average_review_rating = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn default_variant(mut self, value: ProductPublicVariant) -> Self {
        self.default_variant = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn experiences(mut self, value: Vec<ProductExperience>) -> Self {
        self.experiences = Some(value);
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

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
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

    pub fn variant_attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.variant_attributes = Some(value);
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

    /// Consumes the builder and constructs a [`ProductListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`average_review_rating`](ProductListItemBuilder::average_review_rating)
    /// - [`created_at`](ProductListItemBuilder::created_at)
    /// - [`experiences`](ProductListItemBuilder::experiences)
    /// - [`gallery_images`](ProductListItemBuilder::gallery_images)
    /// - [`id`](ProductListItemBuilder::id)
    /// - [`labels`](ProductListItemBuilder::labels)
    /// - [`member_count`](ProductListItemBuilder::member_count)
    /// - [`published_reviews_count`](ProductListItemBuilder::published_reviews_count)
    /// - [`route`](ProductListItemBuilder::route)
    /// - [`title`](ProductListItemBuilder::title)
    /// - [`updated_at`](ProductListItemBuilder::updated_at)
    /// - [`verified`](ProductListItemBuilder::verified)
    pub fn build(self) -> Result<ProductListItem, BuildError> {
        Ok(ProductListItem {
            account: self.account,
            average_review_rating: self
                .average_review_rating
                .ok_or_else(|| BuildError::missing_field("average_review_rating"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            default_variant: self.default_variant,
            description: self.description,
            experiences: self
                .experiences
                .ok_or_else(|| BuildError::missing_field("experiences"))?,
            external_identifier: self.external_identifier,
            gallery_images: self
                .gallery_images
                .ok_or_else(|| BuildError::missing_field("gallery_images"))?,
            headline: self.headline,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            labels: self
                .labels
                .ok_or_else(|| BuildError::missing_field("labels"))?,
            logo_url: self.logo_url,
            member_count: self
                .member_count
                .ok_or_else(|| BuildError::missing_field("member_count"))?,
            metadata: self.metadata,
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
            variant_attributes: self.variant_attributes,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
            visibility: self.visibility,
        })
    }
}
