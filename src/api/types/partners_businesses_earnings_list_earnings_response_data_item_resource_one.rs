pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceOne {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub id: String,
    pub object: ListEarningsResponseDataItemResourceOneObject,
}

impl ListEarningsResponseDataItemResourceOne {
    pub fn builder() -> ListEarningsResponseDataItemResourceOneBuilder {
        <ListEarningsResponseDataItemResourceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceOneBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    id: Option<String>,
    object: Option<ListEarningsResponseDataItemResourceOneObject>,
}

impl ListEarningsResponseDataItemResourceOneBuilder {
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

    pub fn object(mut self, value: ListEarningsResponseDataItemResourceOneObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListEarningsResponseDataItemResourceOneBuilder::created_at)
    /// - [`currency`](ListEarningsResponseDataItemResourceOneBuilder::currency)
    /// - [`id`](ListEarningsResponseDataItemResourceOneBuilder::id)
    /// - [`object`](ListEarningsResponseDataItemResourceOneBuilder::object)
    pub fn build(self) -> Result<ListEarningsResponseDataItemResourceOne, BuildError> {
        Ok(ListEarningsResponseDataItemResourceOne {
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
