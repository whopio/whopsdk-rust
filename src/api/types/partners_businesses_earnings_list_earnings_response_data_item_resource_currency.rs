pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceCurrency {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    pub object: ListEarningsResponseDataItemResourceCurrencyObject,
}

impl ListEarningsResponseDataItemResourceCurrency {
    pub fn builder() -> ListEarningsResponseDataItemResourceCurrencyBuilder {
        <ListEarningsResponseDataItemResourceCurrencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceCurrencyBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    id: Option<String>,
    merchant_name: Option<String>,
    object: Option<ListEarningsResponseDataItemResourceCurrencyObject>,
}

impl ListEarningsResponseDataItemResourceCurrencyBuilder {
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

    pub fn merchant_name(mut self, value: impl Into<String>) -> Self {
        self.merchant_name = Some(value.into());
        self
    }

    pub fn object(mut self, value: ListEarningsResponseDataItemResourceCurrencyObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceCurrency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListEarningsResponseDataItemResourceCurrencyBuilder::created_at)
    /// - [`id`](ListEarningsResponseDataItemResourceCurrencyBuilder::id)
    /// - [`object`](ListEarningsResponseDataItemResourceCurrencyBuilder::object)
    pub fn build(self) -> Result<ListEarningsResponseDataItemResourceCurrency, BuildError> {
        Ok(ListEarningsResponseDataItemResourceCurrency {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self.currency,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            merchant_name: self.merchant_name,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
