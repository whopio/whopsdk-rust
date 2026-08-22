pub use crate::prelude::*;

/// A company is a seller on Whop. Companies own products, manage members, and receive payouts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompanyListItem {
    /// The datetime the company was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// A promotional pitch written by the company creator, displayed to potential customers on the store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The company's logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<CompanyListItemLogo>,
    /// The total number of users who currently hold active memberships across all of this company's products.
    #[serde(default)]
    pub member_count: i64,
    /// A key-value JSON object of custom metadata for this company, managed by the platform that created the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The user who owns and has full administrative control over this company.
    #[serde(default)]
    pub owner_user: CompanyListItemOwnerUser,
    /// The total number of published customer reviews across all products for this company.
    #[serde(default)]
    pub published_reviews_count: i64,
    /// URL slug for the account's store page, e.g. `pickaxe` in whop.com/pickaxe.
    #[serde(default)]
    pub route: String,
    /// Whether Whop sends transactional emails (receipts, updates) to customers on behalf of this company.
    #[serde(default)]
    pub send_customer_emails: bool,
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

impl CompanyListItem {
    pub fn builder() -> CompanyListItemBuilder {
        <CompanyListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyListItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    logo: Option<CompanyListItemLogo>,
    member_count: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    owner_user: Option<CompanyListItemOwnerUser>,
    published_reviews_count: Option<i64>,
    route: Option<String>,
    send_customer_emails: Option<bool>,
    target_audience: Option<String>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    verified: Option<bool>,
}

impl CompanyListItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo(mut self, value: CompanyListItemLogo) -> Self {
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

    pub fn owner_user(mut self, value: CompanyListItemOwnerUser) -> Self {
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

    /// Consumes the builder and constructs a [`CompanyListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CompanyListItemBuilder::created_at)
    /// - [`id`](CompanyListItemBuilder::id)
    /// - [`member_count`](CompanyListItemBuilder::member_count)
    /// - [`owner_user`](CompanyListItemBuilder::owner_user)
    /// - [`published_reviews_count`](CompanyListItemBuilder::published_reviews_count)
    /// - [`route`](CompanyListItemBuilder::route)
    /// - [`send_customer_emails`](CompanyListItemBuilder::send_customer_emails)
    /// - [`title`](CompanyListItemBuilder::title)
    /// - [`updated_at`](CompanyListItemBuilder::updated_at)
    /// - [`verified`](CompanyListItemBuilder::verified)
    pub fn build(self) -> Result<CompanyListItem, BuildError> {
        Ok(CompanyListItem {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
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
