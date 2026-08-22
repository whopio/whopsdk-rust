pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemFinancialActivityItem {
    /// Line amount in its native currency.
    #[serde(default)]
    pub amount: String,
    /// Line amount in USD.
    #[serde(default)]
    pub amount_usd: String,
    /// Fee or cost category of the line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Currency of the native amount.
    #[serde(default)]
    pub currency: String,
    /// Whether the line is income Whop collected or a cost Whop paid.
    pub r#type: ListEarningsResponseDataItemFinancialActivityItemType,
}

impl ListEarningsResponseDataItemFinancialActivityItem {
    pub fn builder() -> ListEarningsResponseDataItemFinancialActivityItemBuilder {
        <ListEarningsResponseDataItemFinancialActivityItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemFinancialActivityItemBuilder {
    amount: Option<String>,
    amount_usd: Option<String>,
    category: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    r#type: Option<ListEarningsResponseDataItemFinancialActivityItemType>,
}

impl ListEarningsResponseDataItemFinancialActivityItemBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn amount_usd(mut self, value: impl Into<String>) -> Self {
        self.amount_usd = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ListEarningsResponseDataItemFinancialActivityItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemFinancialActivityItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ListEarningsResponseDataItemFinancialActivityItemBuilder::amount)
    /// - [`amount_usd`](ListEarningsResponseDataItemFinancialActivityItemBuilder::amount_usd)
    /// - [`currency`](ListEarningsResponseDataItemFinancialActivityItemBuilder::currency)
    /// - [`r#type`](ListEarningsResponseDataItemFinancialActivityItemBuilder::r#type)
    pub fn build(self) -> Result<ListEarningsResponseDataItemFinancialActivityItem, BuildError> {
        Ok(ListEarningsResponseDataItemFinancialActivityItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            amount_usd: self
                .amount_usd
                .ok_or_else(|| BuildError::missing_field("amount_usd"))?,
            category: self.category,
            created_at: self.created_at,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
