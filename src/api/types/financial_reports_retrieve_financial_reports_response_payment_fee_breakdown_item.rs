pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrieveFinancialReportsResponsePaymentFeeBreakdownItem {
    /// Payment costs attributed to this payment method.
    #[serde(default)]
    pub amount: Money,
    /// The customer-facing payment method family or standalone service.
    pub category: RetrieveFinancialReportsResponsePaymentFeeBreakdownItemCategory,
    /// The card brand, payment provider, payment rail, or standalone service.
    #[serde(default)]
    pub payment_method: String,
    /// Start of the time bucket containing these payment costs.
    #[serde(default)]
    pub period: String,
}

impl RetrieveFinancialReportsResponsePaymentFeeBreakdownItem {
    pub fn builder() -> RetrieveFinancialReportsResponsePaymentFeeBreakdownItemBuilder {
        <RetrieveFinancialReportsResponsePaymentFeeBreakdownItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveFinancialReportsResponsePaymentFeeBreakdownItemBuilder {
    amount: Option<Money>,
    category: Option<RetrieveFinancialReportsResponsePaymentFeeBreakdownItemCategory>,
    payment_method: Option<String>,
    period: Option<String>,
}

impl RetrieveFinancialReportsResponsePaymentFeeBreakdownItemBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn category(
        mut self,
        value: RetrieveFinancialReportsResponsePaymentFeeBreakdownItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveFinancialReportsResponsePaymentFeeBreakdownItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](RetrieveFinancialReportsResponsePaymentFeeBreakdownItemBuilder::amount)
    /// - [`category`](RetrieveFinancialReportsResponsePaymentFeeBreakdownItemBuilder::category)
    /// - [`payment_method`](RetrieveFinancialReportsResponsePaymentFeeBreakdownItemBuilder::payment_method)
    /// - [`period`](RetrieveFinancialReportsResponsePaymentFeeBreakdownItemBuilder::period)
    pub fn build(
        self,
    ) -> Result<RetrieveFinancialReportsResponsePaymentFeeBreakdownItem, BuildError> {
        Ok(RetrieveFinancialReportsResponsePaymentFeeBreakdownItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
        })
    }
}
