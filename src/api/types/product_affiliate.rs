pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProductAffiliate {
    /// When the affiliate enrollment was created, as an ISO 8601 timestamp. Includes automatically created enrollments.
    #[serde(default)]
    pub created_at: String,
    /// Product affiliate ID, prefixed `apaf_`.
    #[serde(default)]
    pub id: String,
    /// The product the affiliate is enrolled to promote.
    #[serde(default)]
    pub product: PromoCodeProduct,
    /// Number of distinct users referred by the affiliate to this product.
    #[serde(default)]
    pub referred_users_count: i64,
    /// Whether the affiliate's enrollment is active or archived.
    pub status: ProductAffiliateStatus,
    /// Net rewards earned by the affiliate for this product after fees and reversals, in USD.
    #[serde(default)]
    pub total_reward_amount_usd: Money,
    /// The user enrolled as an affiliate.
    #[serde(default)]
    pub user: ProductAffiliateUser,
}

impl ProductAffiliate {
    pub fn builder() -> ProductAffiliateBuilder {
        <ProductAffiliateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductAffiliateBuilder {
    created_at: Option<String>,
    id: Option<String>,
    product: Option<PromoCodeProduct>,
    referred_users_count: Option<i64>,
    status: Option<ProductAffiliateStatus>,
    total_reward_amount_usd: Option<Money>,
    user: Option<ProductAffiliateUser>,
}

impl ProductAffiliateBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn product(mut self, value: PromoCodeProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn referred_users_count(mut self, value: i64) -> Self {
        self.referred_users_count = Some(value);
        self
    }

    pub fn status(mut self, value: ProductAffiliateStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total_reward_amount_usd(mut self, value: Money) -> Self {
        self.total_reward_amount_usd = Some(value);
        self
    }

    pub fn user(mut self, value: ProductAffiliateUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductAffiliate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ProductAffiliateBuilder::created_at)
    /// - [`id`](ProductAffiliateBuilder::id)
    /// - [`product`](ProductAffiliateBuilder::product)
    /// - [`referred_users_count`](ProductAffiliateBuilder::referred_users_count)
    /// - [`status`](ProductAffiliateBuilder::status)
    /// - [`total_reward_amount_usd`](ProductAffiliateBuilder::total_reward_amount_usd)
    /// - [`user`](ProductAffiliateBuilder::user)
    pub fn build(self) -> Result<ProductAffiliate, BuildError> {
        Ok(ProductAffiliate {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            product: self
                .product
                .ok_or_else(|| BuildError::missing_field("product"))?,
            referred_users_count: self
                .referred_users_count
                .ok_or_else(|| BuildError::missing_field("referred_users_count"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total_reward_amount_usd: self
                .total_reward_amount_usd
                .ok_or_else(|| BuildError::missing_field("total_reward_amount_usd"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
