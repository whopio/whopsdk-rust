pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceCreatedAt {
    /// The referred business that qualified.
    #[serde(default)]
    pub business_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The referral link reward the business qualified for, prefixed `prwd_`.
    #[serde(default)]
    pub id: String,
    pub object: ListEarningsResponseDataItemResourceCreatedAtObject,
}

impl ListEarningsResponseDataItemResourceCreatedAt {
    pub fn builder() -> ListEarningsResponseDataItemResourceCreatedAtBuilder {
        <ListEarningsResponseDataItemResourceCreatedAtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceCreatedAtBuilder {
    business_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    object: Option<ListEarningsResponseDataItemResourceCreatedAtObject>,
}

impl ListEarningsResponseDataItemResourceCreatedAtBuilder {
    pub fn business_id(mut self, value: impl Into<String>) -> Self {
        self.business_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ListEarningsResponseDataItemResourceCreatedAtObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceCreatedAt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_id`](ListEarningsResponseDataItemResourceCreatedAtBuilder::business_id)
    /// - [`created_at`](ListEarningsResponseDataItemResourceCreatedAtBuilder::created_at)
    /// - [`id`](ListEarningsResponseDataItemResourceCreatedAtBuilder::id)
    /// - [`object`](ListEarningsResponseDataItemResourceCreatedAtBuilder::object)
    pub fn build(self) -> Result<ListEarningsResponseDataItemResourceCreatedAt, BuildError> {
        Ok(ListEarningsResponseDataItemResourceCreatedAt {
            business_id: self
                .business_id
                .ok_or_else(|| BuildError::missing_field("business_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
