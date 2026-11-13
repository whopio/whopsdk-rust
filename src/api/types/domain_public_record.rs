pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainPublicRecord {
    /// Whether the domain's delegation is signed with DNSSEC, or `null` when the registry doesn't say.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec: Option<bool>,
    /// When the current registration expires, as an ISO 8601 timestamp, or `null` when the record omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub name_servers: Vec<String>,
    /// When the domain was first registered, as an ISO 8601 timestamp, or `null` when the record omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<String>,
    /// Who the domain is registered to, often a privacy service or `Redacted For Privacy`. `null` when the record publishes no registrant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant: Option<DomainRegistrant>,
    /// The registrar that manages the domain, or `null` when the record doesn't name one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar: Option<DomainRegistrar>,
    #[serde(default)]
    pub statuses: Vec<String>,
    /// When the registration record last changed, as an ISO 8601 timestamp, or `null` when the record omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl DomainPublicRecord {
    pub fn builder() -> DomainPublicRecordBuilder {
        <DomainPublicRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainPublicRecordBuilder {
    dnssec: Option<bool>,
    expires_at: Option<String>,
    name_servers: Option<Vec<String>>,
    registered_at: Option<String>,
    registrant: Option<DomainRegistrant>,
    registrar: Option<DomainRegistrar>,
    statuses: Option<Vec<String>>,
    updated_at: Option<String>,
}

impl DomainPublicRecordBuilder {
    pub fn dnssec(mut self, value: bool) -> Self {
        self.dnssec = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn name_servers(mut self, value: Vec<String>) -> Self {
        self.name_servers = Some(value);
        self
    }

    pub fn registered_at(mut self, value: impl Into<String>) -> Self {
        self.registered_at = Some(value.into());
        self
    }

    pub fn registrant(mut self, value: DomainRegistrant) -> Self {
        self.registrant = Some(value);
        self
    }

    pub fn registrar(mut self, value: DomainRegistrar) -> Self {
        self.registrar = Some(value);
        self
    }

    pub fn statuses(mut self, value: Vec<String>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainPublicRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name_servers`](DomainPublicRecordBuilder::name_servers)
    /// - [`statuses`](DomainPublicRecordBuilder::statuses)
    pub fn build(self) -> Result<DomainPublicRecord, BuildError> {
        Ok(DomainPublicRecord {
            dnssec: self.dnssec,
            expires_at: self.expires_at,
            name_servers: self
                .name_servers
                .ok_or_else(|| BuildError::missing_field("name_servers"))?,
            registered_at: self.registered_at,
            registrant: self.registrant,
            registrar: self.registrar,
            statuses: self
                .statuses
                .ok_or_else(|| BuildError::missing_field("statuses"))?,
            updated_at: self.updated_at,
        })
    }
}
