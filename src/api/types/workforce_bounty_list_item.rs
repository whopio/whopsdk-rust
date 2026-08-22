pub use crate::prelude::*;

/// A privately accessible bounty.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkforceBountyListItem {
    /// The underlying bounty implementation type.
    pub bounty_type: BountyTypes,
    /// The datetime the bounty was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The currency used for the bounty funds.
    pub currency: Currencies,
    /// The description of the bounty.
    #[serde(default)]
    pub description: String,
    /// The unique identifier for the bounty.
    #[serde(default)]
    pub id: String,
    /// The current lifecycle status of the bounty.
    pub status: Statuses,
    /// The title of the bounty.
    #[serde(default)]
    pub title: String,
    /// The total amount currently funded in the bounty pool for payout.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_available: f64,
    /// The total amount paid out for this bounty.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_paid: f64,
    /// The datetime the bounty was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The number of watcher votes required before the submission can resolve.
    #[serde(default)]
    pub vote_threshold: i64,
}

impl WorkforceBountyListItem {
    pub fn builder() -> WorkforceBountyListItemBuilder {
        <WorkforceBountyListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkforceBountyListItemBuilder {
    bounty_type: Option<BountyTypes>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    description: Option<String>,
    id: Option<String>,
    status: Option<Statuses>,
    title: Option<String>,
    total_available: Option<f64>,
    total_paid: Option<f64>,
    updated_at: Option<DateTime<FixedOffset>>,
    vote_threshold: Option<i64>,
}

impl WorkforceBountyListItemBuilder {
    pub fn bounty_type(mut self, value: BountyTypes) -> Self {
        self.bounty_type = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
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

    pub fn status(mut self, value: Statuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn total_available(mut self, value: f64) -> Self {
        self.total_available = Some(value);
        self
    }

    pub fn total_paid(mut self, value: f64) -> Self {
        self.total_paid = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn vote_threshold(mut self, value: i64) -> Self {
        self.vote_threshold = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkforceBountyListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bounty_type`](WorkforceBountyListItemBuilder::bounty_type)
    /// - [`created_at`](WorkforceBountyListItemBuilder::created_at)
    /// - [`currency`](WorkforceBountyListItemBuilder::currency)
    /// - [`description`](WorkforceBountyListItemBuilder::description)
    /// - [`id`](WorkforceBountyListItemBuilder::id)
    /// - [`status`](WorkforceBountyListItemBuilder::status)
    /// - [`title`](WorkforceBountyListItemBuilder::title)
    /// - [`total_available`](WorkforceBountyListItemBuilder::total_available)
    /// - [`total_paid`](WorkforceBountyListItemBuilder::total_paid)
    /// - [`updated_at`](WorkforceBountyListItemBuilder::updated_at)
    /// - [`vote_threshold`](WorkforceBountyListItemBuilder::vote_threshold)
    pub fn build(self) -> Result<WorkforceBountyListItem, BuildError> {
        Ok(WorkforceBountyListItem {
            bounty_type: self
                .bounty_type
                .ok_or_else(|| BuildError::missing_field("bounty_type"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            total_available: self
                .total_available
                .ok_or_else(|| BuildError::missing_field("total_available"))?,
            total_paid: self
                .total_paid
                .ok_or_else(|| BuildError::missing_field("total_paid"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            vote_threshold: self
                .vote_threshold
                .ok_or_else(|| BuildError::missing_field("vote_threshold"))?,
        })
    }
}
