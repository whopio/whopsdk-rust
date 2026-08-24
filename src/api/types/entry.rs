pub use crate::prelude::*;

/// An entry represents a user's signup for a waitlisted plan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Entry {
    /// The datetime the entry was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The list of responses collected from the user when submitting their waitlist entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_responses: Option<Vec<EntryCustomFieldResponsesItem>>,
    /// The unique identifier for the entry.
    #[serde(default)]
    pub id: String,
    /// The waitlisted plan that this entry is a signup for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<EntryPlan>,
    /// The product associated with this entry's waitlisted plan. Null if the plan is not tied to a product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<EntryProduct>,
    /// The current status of the waitlist entry (e.g., drafted, pending, approved, denied).
    pub status: EntryStatus,
    /// The user who submitted this waitlist entry.
    #[serde(default)]
    pub user: EntryUser,
}

impl Entry {
    pub fn builder() -> EntryBuilder {
        <EntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntryBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    custom_field_responses: Option<Vec<EntryCustomFieldResponsesItem>>,
    id: Option<String>,
    plan: Option<EntryPlan>,
    product: Option<EntryProduct>,
    status: Option<EntryStatus>,
    user: Option<EntryUser>,
}

impl EntryBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn custom_field_responses(mut self, value: Vec<EntryCustomFieldResponsesItem>) -> Self {
        self.custom_field_responses = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn plan(mut self, value: EntryPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: EntryProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn status(mut self, value: EntryStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user(mut self, value: EntryUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Entry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EntryBuilder::id)
    /// - [`status`](EntryBuilder::status)
    /// - [`user`](EntryBuilder::user)
    pub fn build(self) -> Result<Entry, BuildError> {
        Ok(Entry {
            created_at: self.created_at,
            custom_field_responses: self.custom_field_responses,
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
