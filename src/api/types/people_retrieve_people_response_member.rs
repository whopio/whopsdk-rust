pub use crate::prelude::*;

/// The user's member record at this account, when they are a member of it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrievePeopleResponseMember {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_total_spend: Option<f64>,
}

impl RetrievePeopleResponseMember {
    pub fn builder() -> RetrievePeopleResponseMemberBuilder {
        <RetrievePeopleResponseMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseMemberBuilder {
    id: Option<String>,
    joined_at: Option<DateTime<FixedOffset>>,
    status: Option<String>,
    usd_total_spend: Option<f64>,
}

impl RetrievePeopleResponseMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn joined_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.joined_at = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn usd_total_spend(mut self, value: f64) -> Self {
        self.usd_total_spend = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RetrievePeopleResponseMemberBuilder::id)
    pub fn build(self) -> Result<RetrievePeopleResponseMember, BuildError> {
        Ok(RetrievePeopleResponseMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            joined_at: self.joined_at,
            status: self.status,
            usd_total_spend: self.usd_total_spend,
        })
    }
}
