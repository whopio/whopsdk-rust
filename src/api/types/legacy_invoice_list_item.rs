pub use crate::prelude::*;

/// An invoice represents an itemized bill sent by a company to a customer for a specific product and plan, tracking the amount owed, due date, and payment status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LegacyInvoiceListItem {
    /// The datetime the invoice was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The plan that this invoice charges for.
    pub current_plan: LegacyInvoiceListItemCurrentPlan,
    /// The deadline by which payment is expected. Null if the invoice is collected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub due_date: Option<DateTime<FixedOffset>>,
    /// The email address of the customer this invoice is addressed to. Null if no email is on file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// A signed token that allows fetching invoice data publicly without authentication.
    #[serde(default)]
    pub fetch_invoice_token: String,
    /// The unique identifier for the invoice.
    #[serde(default)]
    pub id: String,
    /// Optional line items that break down the invoice total into individual charges.
    #[serde(default)]
    pub line_items: Vec<LegacyInvoiceListItemLineItemsItem>,
    /// The sequential invoice number for display purposes.
    #[serde(default)]
    pub number: String,
    /// Whether a payment on this invoice is still clearing. True while a delayed payment method such as ACH or SEPA settles, during which the invoice stays open and is not marked past due.
    #[serde(default)]
    pub payment_processing: bool,
    /// The current payment status of the invoice, such as draft, open, paid, or void.
    pub status: InvoiceStatuses,
    /// The user this invoice is addressed to. Null if the user account has been removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<LegacyInvoiceListItemUser>,
}

impl LegacyInvoiceListItem {
    pub fn builder() -> LegacyInvoiceListItemBuilder {
        <LegacyInvoiceListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LegacyInvoiceListItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    current_plan: Option<LegacyInvoiceListItemCurrentPlan>,
    due_date: Option<DateTime<FixedOffset>>,
    email_address: Option<String>,
    fetch_invoice_token: Option<String>,
    id: Option<String>,
    line_items: Option<Vec<LegacyInvoiceListItemLineItemsItem>>,
    number: Option<String>,
    payment_processing: Option<bool>,
    status: Option<InvoiceStatuses>,
    user: Option<LegacyInvoiceListItemUser>,
}

impl LegacyInvoiceListItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn current_plan(mut self, value: LegacyInvoiceListItemCurrentPlan) -> Self {
        self.current_plan = Some(value);
        self
    }

    pub fn due_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn fetch_invoice_token(mut self, value: impl Into<String>) -> Self {
        self.fetch_invoice_token = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<LegacyInvoiceListItemLineItemsItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn payment_processing(mut self, value: bool) -> Self {
        self.payment_processing = Some(value);
        self
    }

    pub fn status(mut self, value: InvoiceStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user(mut self, value: LegacyInvoiceListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LegacyInvoiceListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](LegacyInvoiceListItemBuilder::created_at)
    /// - [`current_plan`](LegacyInvoiceListItemBuilder::current_plan)
    /// - [`fetch_invoice_token`](LegacyInvoiceListItemBuilder::fetch_invoice_token)
    /// - [`id`](LegacyInvoiceListItemBuilder::id)
    /// - [`line_items`](LegacyInvoiceListItemBuilder::line_items)
    /// - [`number`](LegacyInvoiceListItemBuilder::number)
    /// - [`payment_processing`](LegacyInvoiceListItemBuilder::payment_processing)
    /// - [`status`](LegacyInvoiceListItemBuilder::status)
    pub fn build(self) -> Result<LegacyInvoiceListItem, BuildError> {
        Ok(LegacyInvoiceListItem {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            current_plan: self
                .current_plan
                .ok_or_else(|| BuildError::missing_field("current_plan"))?,
            due_date: self.due_date,
            email_address: self.email_address,
            fetch_invoice_token: self
                .fetch_invoice_token
                .ok_or_else(|| BuildError::missing_field("fetch_invoice_token"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            payment_processing: self
                .payment_processing
                .ok_or_else(|| BuildError::missing_field("payment_processing"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            user: self.user,
        })
    }
}
