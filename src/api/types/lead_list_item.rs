pub use crate::prelude::*;

/// A prospective customer who has expressed interest in a company or product but has not yet purchased.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LeadListItem {
    /// The datetime the lead was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the lead.
    #[serde(default)]
    pub id: String,
    /// The company member record if this lead has converted into a paying customer. Null if the lead has not converted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<LeadListItemMember>,
    /// Custom key-value pairs attached to this lead. Null if no metadata was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The product the lead expressed interest in. Null if the lead is not associated with a specific product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<LeadListItemProduct>,
    /// The URL of the page that referred this lead to the company. Null if no referrer was captured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// The datetime the lead was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The user account associated with this lead.
    #[serde(default)]
    pub user: LeadListItemUser,
}

impl LeadListItem {
    pub fn builder() -> LeadListItemBuilder {
        <LeadListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadListItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    member: Option<LeadListItemMember>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    product: Option<LeadListItemProduct>,
    referrer: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<LeadListItemUser>,
}

impl LeadListItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn member(mut self, value: LeadListItemMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn product(mut self, value: LeadListItemProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn referrer(mut self, value: impl Into<String>) -> Self {
        self.referrer = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn user(mut self, value: LeadListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeadListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](LeadListItemBuilder::created_at)
    /// - [`id`](LeadListItemBuilder::id)
    /// - [`updated_at`](LeadListItemBuilder::updated_at)
    /// - [`user`](LeadListItemBuilder::user)
    pub fn build(self) -> Result<LeadListItem, BuildError> {
        Ok(LeadListItem {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            member: self.member,
            metadata: self.metadata,
            product: self.product,
            referrer: self.referrer,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
