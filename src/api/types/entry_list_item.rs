pub use crate::prelude::*;

/// An entry represents a user's signup for a waitlisted plan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntryListItem {
    /// The datetime the entry was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The unique identifier for the entry.
    #[serde(default)]
    pub id: String,
    /// The waitlisted plan that this entry is a signup for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<EntryListItemPlan>,
    /// The product associated with this entry's waitlisted plan. Null if the plan is not tied to a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<EntryListItemProduct>,
    /// The current status of the waitlist entry (e.g., drafted, pending, approved, denied).
    pub status: EntryStatus,
    /// The user who submitted this waitlist entry.
    #[serde(default)]
    pub user: EntryListItemUser,
}

impl EntryListItem {
    pub fn builder() -> EntryListItemBuilder {
        <EntryListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntryListItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    plan: Option<EntryListItemPlan>,
    product: Option<EntryListItemProduct>,
    status: Option<EntryStatus>,
    user: Option<EntryListItemUser>,
}

impl EntryListItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn plan(mut self, value: EntryListItemPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: EntryListItemProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn status(mut self, value: EntryStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user(mut self, value: EntryListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntryListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EntryListItemBuilder::id)
    /// - [`status`](EntryListItemBuilder::status)
    /// - [`user`](EntryListItemBuilder::user)
    pub fn build(self) -> Result<EntryListItem, BuildError> {
        Ok(EntryListItem {
            created_at: self.created_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            plan: self.plan,
            product: self.product,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
