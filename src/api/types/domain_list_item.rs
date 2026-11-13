pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainListItem {
    /// ID of the account the domain belongs to, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Offering Apple Pay at checkouts served from the domain. `null` when not enrolled or not visible to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<DomainApplePay>,
    /// Whether you can register the hostname and what it costs. Set only when no domain on Whop has it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<DomainAvailability>,
    /// When the domain was created, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Normalized hostname, such as checkout.example.com.
    #[serde(default)]
    pub domain: String,
    /// Sending and receiving email on the domain through Whop. `null` when not wanted, or when the domain belongs to an account you can't read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<DomainEmail>,
    /// Domain ID, prefixed `dom_`. `null` when no domain on Whop has this hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub issues: Vec<DomainIssue>,
    /// Custom string keys and values attached to this domain. Empty when the domain isn't yours.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// The account that has proven it owns this hostname on Whop, which can be your own. `null` when no account has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<DomainOwner>,
    /// Tracking page views on the domain with the Whop pixel. `null` when not tracked or not visible to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel: Option<DomainPixel>,
    /// An address on a zone Whop runs, such as example.whop.site, that follows its app's route. `null` for other domains or when not visible to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<DomainPlatform>,
    /// Buying the domain through Whop, renewing it, and running its DNS. `null` when not wanted or not visible to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<DomainRegistration>,
    /// Traffic on the domain. `null` when the domain isn't on Whop or isn't visible to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<DomainStats>,
    /// The most pressing state of the domain's capabilities: `releasing`, then `action_required`, then `pending`, then `ready`. `idle` when it wants nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DomainListItemStatus>,
    /// A subdomain of a domain this account bought through Whop, whose DNS Whop runs. Whop removes it when the account no longer holds the root domain. `null` for other domains or when not visible to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<DomainSubdomain>,
    /// Moving the domain to another Whop account. `null` when not turned on or not visible to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<DomainTransfer>,
    /// When the domain was last updated, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Proving you control the domain's DNS with a TXT record. `null` when not wanted or not visible to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<DomainVerification>,
    /// Serving a Whop website on the domain. `null` when not wanted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<DomainWebsite>,
}

impl DomainListItem {
    pub fn builder() -> DomainListItemBuilder {
        <DomainListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainListItemBuilder {
    account_id: Option<String>,
    apple_pay: Option<DomainApplePay>,
    availability: Option<DomainAvailability>,
    created_at: Option<String>,
    domain: Option<String>,
    email: Option<DomainEmail>,
    id: Option<String>,
    issues: Option<Vec<DomainIssue>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    owned_by: Option<DomainOwner>,
    pixel: Option<DomainPixel>,
    platform: Option<DomainPlatform>,
    registration: Option<DomainRegistration>,
    stats: Option<DomainStats>,
    status: Option<DomainListItemStatus>,
    subdomain: Option<DomainSubdomain>,
    transfer: Option<DomainTransfer>,
    updated_at: Option<String>,
    verification: Option<DomainVerification>,
    website: Option<DomainWebsite>,
}

impl DomainListItemBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn apple_pay(mut self, value: DomainApplePay) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn availability(mut self, value: DomainAvailability) -> Self {
        self.availability = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn email(mut self, value: DomainEmail) -> Self {
        self.email = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn issues(mut self, value: Vec<DomainIssue>) -> Self {
        self.issues = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn owned_by(mut self, value: DomainOwner) -> Self {
        self.owned_by = Some(value);
        self
    }

    pub fn pixel(mut self, value: DomainPixel) -> Self {
        self.pixel = Some(value);
        self
    }

    pub fn platform(mut self, value: DomainPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn registration(mut self, value: DomainRegistration) -> Self {
        self.registration = Some(value);
        self
    }

    pub fn stats(mut self, value: DomainStats) -> Self {
        self.stats = Some(value);
        self
    }

    pub fn status(mut self, value: DomainListItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subdomain(mut self, value: DomainSubdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn transfer(mut self, value: DomainTransfer) -> Self {
        self.transfer = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn verification(mut self, value: DomainVerification) -> Self {
        self.verification = Some(value);
        self
    }

    pub fn website(mut self, value: DomainWebsite) -> Self {
        self.website = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](DomainListItemBuilder::domain)
    /// - [`issues`](DomainListItemBuilder::issues)
    /// - [`metadata`](DomainListItemBuilder::metadata)
    pub fn build(self) -> Result<DomainListItem, BuildError> {
        Ok(DomainListItem {
            account_id: self.account_id,
            apple_pay: self.apple_pay,
            availability: self.availability,
            created_at: self.created_at,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            email: self.email,
            id: self.id,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            owned_by: self.owned_by,
            pixel: self.pixel,
            platform: self.platform,
            registration: self.registration,
            stats: self.stats,
            status: self.status,
            subdomain: self.subdomain,
            transfer: self.transfer,
            updated_at: self.updated_at,
            verification: self.verification,
            website: self.website,
        })
    }
}
