pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceFive {
    /// The referred business that qualified.
    #[serde(default)]
    pub business_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The referral link reward the business qualified for, prefixed `prwd_`.
    #[serde(default)]
    pub id: String,
    pub object: ListEarningsResponseDataItemResourceFiveObject,
}

impl ListEarningsResponseDataItemResourceFive {
    pub fn builder() -> ListEarningsResponseDataItemResourceFiveBuilder {
        <ListEarningsResponseDataItemResourceFiveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceFiveBuilder {
    business_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    object: Option<ListEarningsResponseDataItemResourceFiveObject>,
}

impl ListEarningsResponseDataItemResourceFiveBuilder {
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

    pub fn object(mut self, value: ListEarningsResponseDataItemResourceFiveObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceFive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_id`](ListEarningsResponseDataItemResourceFiveBuilder::business_id)
    /// - [`created_at`](ListEarningsResponseDataItemResourceFiveBuilder::created_at)
    /// - [`id`](ListEarningsResponseDataItemResourceFiveBuilder::id)
    /// - [`object`](ListEarningsResponseDataItemResourceFiveBuilder::object)
    pub fn build(self) -> Result<ListEarningsResponseDataItemResourceFive, BuildError> {
        Ok(ListEarningsResponseDataItemResourceFive {
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
