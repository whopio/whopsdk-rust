pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Domain {
    /// ID of the account claiming or owning this domain, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// ID of the app assigned to this domain, prefixed `app_`.
    #[serde(default)]
    pub app_id: String,
    /// Cloudflare's latest certificate issuance status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<String>,
    /// When the domain claim was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub dns_records: Vec<DomainDnsRecord>,
    /// Result of the most recent DNS routing check. Ownership is verified separately.
    pub dns_status: DomainDnsStatus,
    /// Normalized hostname, such as checkout.example.com.
    #[serde(default)]
    pub domain: String,
    /// Cloudflare's latest hostname activation status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_status: Option<String>,
    /// Domain ID, prefixed `dom_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub issues: Vec<DomainIssue>,
    /// When DNS and provider state were last checked, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_checked_at: Option<String>,
    /// Custom string keys and values attached to this domain.
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    /// Domain lifecycle. Only active domains resolve to their app.
    pub status: DomainStatus,
    /// When the domain was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// When an unverified claim is automatically deleted, 48 hours after creation, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_expires_at: Option<String>,
    /// When Whop verified the ownership TXT record, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

impl Domain {
    pub fn builder() -> DomainBuilder {
        <DomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainBuilder {
    account_id: Option<String>,
    app_id: Option<String>,
    certificate_status: Option<String>,
    created_at: Option<String>,
    dns_records: Option<Vec<DomainDnsRecord>>,
    dns_status: Option<DomainDnsStatus>,
    domain: Option<String>,
    hostname_status: Option<String>,
    id: Option<String>,
    issues: Option<Vec<DomainIssue>>,
    last_checked_at: Option<String>,
    metadata: Option<HashMap<String, String>>,
    status: Option<DomainStatus>,
    updated_at: Option<String>,
    verification_expires_at: Option<String>,
    verified_at: Option<String>,
}

impl DomainBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn certificate_status(mut self, value: impl Into<String>) -> Self {
        self.certificate_status = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn dns_records(mut self, value: Vec<DomainDnsRecord>) -> Self {
        self.dns_records = Some(value);
        self
    }

    pub fn dns_status(mut self, value: DomainDnsStatus) -> Self {
        self.dns_status = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn hostname_status(mut self, value: impl Into<String>) -> Self {
        self.hostname_status = Some(value.into());
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

    pub fn last_checked_at(mut self, value: impl Into<String>) -> Self {
        self.last_checked_at = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn status(mut self, value: DomainStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn verification_expires_at(mut self, value: impl Into<String>) -> Self {
        self.verification_expires_at = Some(value.into());
        self
    }

    pub fn verified_at(mut self, value: impl Into<String>) -> Self {
        self.verified_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Domain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](DomainBuilder::account_id)
    /// - [`app_id`](DomainBuilder::app_id)
    /// - [`created_at`](DomainBuilder::created_at)
    /// - [`dns_records`](DomainBuilder::dns_records)
    /// - [`dns_status`](DomainBuilder::dns_status)
    /// - [`domain`](DomainBuilder::domain)
    /// - [`id`](DomainBuilder::id)
    /// - [`issues`](DomainBuilder::issues)
    /// - [`metadata`](DomainBuilder::metadata)
    /// - [`status`](DomainBuilder::status)
    /// - [`updated_at`](DomainBuilder::updated_at)
    pub fn build(self) -> Result<Domain, BuildError> {
        Ok(Domain {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            app_id: self
                .app_id
                .ok_or_else(|| BuildError::missing_field("app_id"))?,
            certificate_status: self.certificate_status,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            dns_records: self
                .dns_records
                .ok_or_else(|| BuildError::missing_field("dns_records"))?,
            dns_status: self
                .dns_status
                .ok_or_else(|| BuildError::missing_field("dns_status"))?,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            hostname_status: self.hostname_status,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            last_checked_at: self.last_checked_at,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            verification_expires_at: self.verification_expires_at,
            verified_at: self.verified_at,
        })
    }
}
