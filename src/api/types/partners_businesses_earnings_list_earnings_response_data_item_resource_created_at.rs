pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceCreatedAt {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The referred partner business's own commission the share was paid on, prefixed `afpy_`.
    #[serde(default)]
    pub id: String,
    /// Which income source that commission is on.
    pub income_source: ListEarningsResponseDataItemResourceCreatedAtIncomeSource,
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
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    income_source: Option<ListEarningsResponseDataItemResourceCreatedAtIncomeSource>,
    object: Option<ListEarningsResponseDataItemResourceCreatedAtObject>,
}

impl ListEarningsResponseDataItemResourceCreatedAtBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn income_source(
        mut self,
        value: ListEarningsResponseDataItemResourceCreatedAtIncomeSource,
    ) -> Self {
        self.income_source = Some(value);
        self
    }

    pub fn object(mut self, value: ListEarningsResponseDataItemResourceCreatedAtObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceCreatedAt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListEarningsResponseDataItemResourceCreatedAtBuilder::created_at)
    /// - [`id`](ListEarningsResponseDataItemResourceCreatedAtBuilder::id)
    /// - [`income_source`](ListEarningsResponseDataItemResourceCreatedAtBuilder::income_source)
    /// - [`object`](ListEarningsResponseDataItemResourceCreatedAtBuilder::object)
    pub fn build(self) -> Result<ListEarningsResponseDataItemResourceCreatedAt, BuildError> {
        Ok(ListEarningsResponseDataItemResourceCreatedAt {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            income_source: self
                .income_source
                .ok_or_else(|| BuildError::missing_field("income_source"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
