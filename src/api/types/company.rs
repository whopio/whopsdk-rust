pub use crate::prelude::*;

/// A company is a seller on Whop. Companies own products, manage members, and receive payouts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Company {
    /// Guidelines and instructions provided to affiliates explaining how to promote this company's products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_instructions: Option<String>,
    /// The datetime the company was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// A promotional pitch written by the company creator, displayed to potential customers on the store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The product featured for affiliates to promote on this company's affiliate page. Null if none is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_affiliate_product: Option<CompanyFeaturedAffiliateProduct>,
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The company's logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<CompanyLogo>,
    /// The total number of users who currently hold active memberships across all of this company's products.
    #[serde(default)]
    pub member_count: i64,
    /// A key-value JSON object of custom metadata for this company, managed by the platform that created the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The user who owns and has full administrative control over this company.
    #[serde(default)]
    pub owner_user: CompanyOwnerUser,
    /// The total number of published customer reviews across all products for this company.
    #[serde(default)]
    pub published_reviews_count: i64,
    /// URL slug for the account's store page, e.g. `pickaxe` in whop.com/pickaxe.
    #[serde(default)]
    pub route: String,
    /// Whether Whop sends transactional emails (receipts, updates) to customers on behalf of this company.
    #[serde(default)]
    pub send_customer_emails: bool,
    /// The list of social media accounts and external links associated with this company.
    #[serde(default)]
    pub social_links: Vec<CompanySocialLinksItem>,
    /// The target audience for the company. Null if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<String>,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
    /// The datetime the company was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Whether this company has been verified by Whop's trust and safety team.
    #[serde(default)]
    pub verified: bool,
}

impl Company {
    pub fn builder() -> CompanyBuilder {
        <CompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyBuilder {
    affiliate_instructions: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    featured_affiliate_product: Option<CompanyFeaturedAffiliateProduct>,
    id: Option<String>,
    logo: Option<CompanyLogo>,
    member_count: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    owner_user: Option<CompanyOwnerUser>,
    published_reviews_count: Option<i64>,
    route: Option<String>,
    send_customer_emails: Option<bool>,
    social_links: Option<Vec<CompanySocialLinksItem>>,
    target_audience: Option<String>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    verified: Option<bool>,
}

impl CompanyBuilder {
    pub fn affiliate_instructions(mut self, value: impl Into<String>) -> Self {
        self.affiliate_instructions = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn featured_affiliate_product(mut self, value: CompanyFeaturedAffiliateProduct) -> Self {
        self.featured_affiliate_product = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo(mut self, value: CompanyLogo) -> Self {
        self.logo = Some(value);
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

    pub fn owner_user(mut self, value: CompanyOwnerUser) -> Self {
        self.owner_user = Some(value);
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

    pub fn send_customer_emails(mut self, value: bool) -> Self {
        self.send_customer_emails = Some(value);
        self
    }

    pub fn social_links(mut self, value: Vec<CompanySocialLinksItem>) -> Self {
        self.social_links = Some(value);
        self
    }

    pub fn target_audience(mut self, value: impl Into<String>) -> Self {
        self.target_audience = Some(value.into());
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

    /// Consumes the builder and constructs a [`Company`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CompanyBuilder::created_at)
    /// - [`id`](CompanyBuilder::id)
    /// - [`member_count`](CompanyBuilder::member_count)
    /// - [`owner_user`](CompanyBuilder::owner_user)
    /// - [`published_reviews_count`](CompanyBuilder::published_reviews_count)
    /// - [`route`](CompanyBuilder::route)
    /// - [`send_customer_emails`](CompanyBuilder::send_customer_emails)
    /// - [`social_links`](CompanyBuilder::social_links)
    /// - [`title`](CompanyBuilder::title)
    /// - [`updated_at`](CompanyBuilder::updated_at)
    /// - [`verified`](CompanyBuilder::verified)
    pub fn build(self) -> Result<Company, BuildError> {
        Ok(Company {
            affiliate_instructions: self.affiliate_instructions,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            featured_affiliate_product: self.featured_affiliate_product,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo: self.logo,
            member_count: self
                .member_count
                .ok_or_else(|| BuildError::missing_field("member_count"))?,
            metadata: self.metadata,
            owner_user: self
                .owner_user
                .ok_or_else(|| BuildError::missing_field("owner_user"))?,
            published_reviews_count: self
                .published_reviews_count
                .ok_or_else(|| BuildError::missing_field("published_reviews_count"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            send_customer_emails: self
                .send_customer_emails
                .ok_or_else(|| BuildError::missing_field("send_customer_emails"))?,
            social_links: self
                .social_links
                .ok_or_else(|| BuildError::missing_field("social_links"))?,
            target_audience: self.target_audience,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
        })
    }
}
