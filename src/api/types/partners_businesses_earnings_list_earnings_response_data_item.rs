pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListEarningsResponseDataItem {
    /// Referred account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<ListEarningsResponseDataItemAccount>,
    /// Why the earning was canceled or reversed, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelation_reason: Option<String>,
    /// What the referrer earns, in USD. Null until the earning settles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_amount_usd: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Income and cost lines behind this earning's commission. Null for earnings settled before this data was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_activity: Option<Vec<ListEarningsResponseDataItemFinancialActivityItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Which income source the commission is on: product-sales gross profit, Whop Ads spend billed to the business, platform balance transfer fees, or Whop Card interchange.
    pub income_source: ListEarningsResponseDataItemIncomeSource,
    pub object: ListEarningsResponseDataItemObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub payout_at: Option<DateTime<FixedOffset>>,
    /// The referrer's share of Whop's gross profit, as a fraction (0.3 = 30%). Null until the earning settles.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub payout_percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ListEarningsResponseDataItemProduct>,
    /// The resource that generated the earning: the customer payment receipt for sales and ad spend earnings, the balance transfer for transfer earnings, or the card transaction for card interchange earnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<ListEarningsResponseDataItemResource>,
    /// Whether this earning is a second-tier (grandparent) commission.
    #[serde(default)]
    pub second_tier: bool,
    /// Current status of the earning.
    pub status: ListEarningsResponseDataItemStatus,
    /// The underlying transaction amount the commission's income comes from, in USD.
    #[serde(default)]
    pub transaction_amount_usd: String,
}

impl ListEarningsResponseDataItem {
    pub fn builder() -> ListEarningsResponseDataItemBuilder {
        <ListEarningsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemBuilder {
    account: Option<ListEarningsResponseDataItemAccount>,
    cancelation_reason: Option<String>,
    commission_amount_usd: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    financial_activity: Option<Vec<ListEarningsResponseDataItemFinancialActivityItem>>,
    id: Option<String>,
    income_source: Option<ListEarningsResponseDataItemIncomeSource>,
    object: Option<ListEarningsResponseDataItemObject>,
    payout_at: Option<DateTime<FixedOffset>>,
    payout_percentage: Option<f64>,
    product: Option<ListEarningsResponseDataItemProduct>,
    resource: Option<ListEarningsResponseDataItemResource>,
    second_tier: Option<bool>,
    status: Option<ListEarningsResponseDataItemStatus>,
    transaction_amount_usd: Option<String>,
}

impl ListEarningsResponseDataItemBuilder {
    pub fn account(mut self, value: ListEarningsResponseDataItemAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn cancelation_reason(mut self, value: impl Into<String>) -> Self {
        self.cancelation_reason = Some(value.into());
        self
    }

    pub fn commission_amount_usd(mut self, value: impl Into<String>) -> Self {
        self.commission_amount_usd = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn financial_activity(
        mut self,
        value: Vec<ListEarningsResponseDataItemFinancialActivityItem>,
    ) -> Self {
        self.financial_activity = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn income_source(mut self, value: ListEarningsResponseDataItemIncomeSource) -> Self {
        self.income_source = Some(value);
        self
    }

    pub fn object(mut self, value: ListEarningsResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn payout_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.payout_at = Some(value);
        self
    }

    pub fn payout_percentage(mut self, value: f64) -> Self {
        self.payout_percentage = Some(value);
        self
    }

    pub fn product(mut self, value: ListEarningsResponseDataItemProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn resource(mut self, value: ListEarningsResponseDataItemResource) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn second_tier(mut self, value: bool) -> Self {
        self.second_tier = Some(value);
        self
    }

    pub fn status(mut self, value: ListEarningsResponseDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn transaction_amount_usd(mut self, value: impl Into<String>) -> Self {
        self.transaction_amount_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListEarningsResponseDataItemBuilder::created_at)
    /// - [`income_source`](ListEarningsResponseDataItemBuilder::income_source)
    /// - [`object`](ListEarningsResponseDataItemBuilder::object)
    /// - [`second_tier`](ListEarningsResponseDataItemBuilder::second_tier)
    /// - [`status`](ListEarningsResponseDataItemBuilder::status)
    /// - [`transaction_amount_usd`](ListEarningsResponseDataItemBuilder::transaction_amount_usd)
    pub fn build(self) -> Result<ListEarningsResponseDataItem, BuildError> {
        Ok(ListEarningsResponseDataItem {
            account: self.account,
            cancelation_reason: self.cancelation_reason,
            commission_amount_usd: self.commission_amount_usd,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            financial_activity: self.financial_activity,
            id: self.id,
            income_source: self
                .income_source
                .ok_or_else(|| BuildError::missing_field("income_source"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payout_at: self.payout_at,
            payout_percentage: self.payout_percentage,
            product: self.product,
            resource: self.resource,
            second_tier: self
                .second_tier
                .ok_or_else(|| BuildError::missing_field("second_tier"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            transaction_amount_usd: self
                .transaction_amount_usd
                .ok_or_else(|| BuildError::missing_field("transaction_amount_usd"))?,
        })
    }
}
