pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceTwo {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub id: String,
    pub object: ListEarningsResponseDataItemResourceTwoObject,
}

impl ListEarningsResponseDataItemResourceTwo {
    pub fn builder() -> ListEarningsResponseDataItemResourceTwoBuilder {
        <ListEarningsResponseDataItemResourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceTwoBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    id: Option<String>,
    object: Option<ListEarningsResponseDataItemResourceTwoObject>,
}

impl ListEarningsResponseDataItemResourceTwoBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ListEarningsResponseDataItemResourceTwoObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListEarningsResponseDataItemResourceTwoBuilder::created_at)
    /// - [`currency`](ListEarningsResponseDataItemResourceTwoBuilder::currency)
    /// - [`id`](ListEarningsResponseDataItemResourceTwoBuilder::id)
    /// - [`object`](ListEarningsResponseDataItemResourceTwoBuilder::object)
    pub fn build(self) -> Result<ListEarningsResponseDataItemResourceTwo, BuildError> {
        Ok(ListEarningsResponseDataItemResourceTwo {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
