pub use crate::prelude::*;

/// A dispute is a chargeback or payment challenge filed against a company, including evidence and response status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisputeListItem {
    /// The disputed amount in the specified currency, formatted as a decimal.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The company that the dispute was filed against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<DisputeListItemCompany>,
    /// The datetime the dispute was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The three-letter ISO currency code for the disputed amount.
    pub currency: Currencies,
    /// Whether the dispute evidence can still be edited and submitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// The unique identifier for the dispute.
    #[serde(default)]
    pub id: String,
    /// The deadline by which dispute evidence must be submitted. Null if no response deadline is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub needs_response_by: Option<DateTime<FixedOffset>>,
    /// The original payment that was disputed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<DisputeListItemPayment>,
    /// The plan associated with the disputed payment. Null if the dispute is not linked to a specific plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<DisputeListItemPlan>,
    /// The product associated with the disputed payment. Null if the dispute is not linked to a specific product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<DisputeListItemProduct>,
    /// A human-readable reason for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The card network reason code for the dispute. Null when the payment processor did not provide one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// The current status of the dispute lifecycle, such as needs_response, under_review, won, or lost.
    pub status: DisputeStatuses,
    /// Whether the dispute was automatically resolved through Visa Rapid Dispute Resolution (RDR).
    #[serde(default)]
    pub visa_rdr: bool,
}

impl DisputeListItem {
    pub fn builder() -> DisputeListItemBuilder {
        <DisputeListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemBuilder {
    amount: Option<f64>,
    company: Option<DisputeListItemCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    editable: Option<bool>,
    id: Option<String>,
    needs_response_by: Option<DateTime<FixedOffset>>,
    payment: Option<DisputeListItemPayment>,
    plan: Option<DisputeListItemPlan>,
    product: Option<DisputeListItemProduct>,
    reason: Option<String>,
    reason_code: Option<String>,
    status: Option<DisputeStatuses>,
    visa_rdr: Option<bool>,
}

impl DisputeListItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn company(mut self, value: DisputeListItemCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn editable(mut self, value: bool) -> Self {
        self.editable = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn needs_response_by(mut self, value: DateTime<FixedOffset>) -> Self {
        self.needs_response_by = Some(value);
        self
    }

    pub fn payment(mut self, value: DisputeListItemPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn plan(mut self, value: DisputeListItemPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: DisputeListItemProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn reason_code(mut self, value: impl Into<String>) -> Self {
        self.reason_code = Some(value.into());
        self
    }

    pub fn status(mut self, value: DisputeStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn visa_rdr(mut self, value: bool) -> Self {
        self.visa_rdr = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](DisputeListItemBuilder::amount)
    /// - [`currency`](DisputeListItemBuilder::currency)
    /// - [`id`](DisputeListItemBuilder::id)
    /// - [`status`](DisputeListItemBuilder::status)
    /// - [`visa_rdr`](DisputeListItemBuilder::visa_rdr)
    pub fn build(self) -> Result<DisputeListItem, BuildError> {
        Ok(DisputeListItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            company: self.company,
            created_at: self.created_at,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            editable: self.editable,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            needs_response_by: self.needs_response_by,
            payment: self.payment,
            plan: self.plan,
            product: self.product,
            reason: self.reason,
            reason_code: self.reason_code,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            visa_rdr: self
                .visa_rdr
                .ok_or_else(|| BuildError::missing_field("visa_rdr"))?,
        })
    }
}
