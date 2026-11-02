pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrieveFinancialReportsResponseRowsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ik_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The family the row's `line_category` rolls up into. Balance summary rows are always `balance`.
    pub grouping: RetrieveFinancialReportsResponseRowsItemGrouping,
    /// The ledger line category the row aggregates. Balance summary rows carry the balance bucket instead.
    pub line_category: RetrieveFinancialReportsResponseRowsItemLineCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count: Option<i64>,
    /// Start of the time bucket this row covers, truncated to `group_by`.
    #[serde(default)]
    pub period: String,
    /// Which side of the income statement the category falls on, or `null` when it is not a P&L category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_and_loss_section:
        Option<RetrieveFinancialReportsResponseRowsItemProfitAndLossSection>,
}

impl RetrieveFinancialReportsResponseRowsItem {
    pub fn builder() -> RetrieveFinancialReportsResponseRowsItemBuilder {
        <RetrieveFinancialReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveFinancialReportsResponseRowsItemBuilder {
    account_ik_path: Option<String>,
    account_name: Option<String>,
    account_type: Option<String>,
    amount: Option<f64>,
    grouping: Option<RetrieveFinancialReportsResponseRowsItemGrouping>,
    line_category: Option<RetrieveFinancialReportsResponseRowsItemLineCategory>,
    line_count: Option<i64>,
    period: Option<String>,
    profit_and_loss_section: Option<RetrieveFinancialReportsResponseRowsItemProfitAndLossSection>,
}

impl RetrieveFinancialReportsResponseRowsItemBuilder {
    pub fn account_ik_path(mut self, value: impl Into<String>) -> Self {
        self.account_ik_path = Some(value.into());
        self
    }

    pub fn account_name(mut self, value: impl Into<String>) -> Self {
        self.account_name = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn grouping(mut self, value: RetrieveFinancialReportsResponseRowsItemGrouping) -> Self {
        self.grouping = Some(value);
        self
    }

    pub fn line_category(
        mut self,
        value: RetrieveFinancialReportsResponseRowsItemLineCategory,
    ) -> Self {
        self.line_category = Some(value);
        self
    }

    pub fn line_count(mut self, value: i64) -> Self {
        self.line_count = Some(value);
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    pub fn profit_and_loss_section(
        mut self,
        value: RetrieveFinancialReportsResponseRowsItemProfitAndLossSection,
    ) -> Self {
        self.profit_and_loss_section = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveFinancialReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](RetrieveFinancialReportsResponseRowsItemBuilder::amount)
    /// - [`grouping`](RetrieveFinancialReportsResponseRowsItemBuilder::grouping)
    /// - [`line_category`](RetrieveFinancialReportsResponseRowsItemBuilder::line_category)
    /// - [`period`](RetrieveFinancialReportsResponseRowsItemBuilder::period)
    pub fn build(self) -> Result<RetrieveFinancialReportsResponseRowsItem, BuildError> {
        Ok(RetrieveFinancialReportsResponseRowsItem {
            account_ik_path: self.account_ik_path,
            account_name: self.account_name,
            account_type: self.account_type,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            grouping: self
                .grouping
                .ok_or_else(|| BuildError::missing_field("grouping"))?,
            line_category: self
                .line_category
                .ok_or_else(|| BuildError::missing_field("line_category"))?,
            line_count: self.line_count,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
            profit_and_loss_section: self.profit_and_loss_section,
        })
    }
}
