pub use crate::prelude::*;

/// An invoice represents an itemized bill sent by a company to a customer for a specific product and plan, tracking the amount owed, due date, and payment status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    /// The date and time when the invoice will be automatically finalized. For charge_automatically, triggers an automatic charge. For send_invoice, sends the invoice email at the specified time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub automatically_finalizes_at: Option<DateTime<FixedOffset>>,
    /// Whether the invoice includes a buyer processing fee on top of the plan price.
    #[serde(default)]
    pub charge_buyer_fee: bool,
    /// The method used to collect payment for this invoice, such as automatic charging or manual payment.
    pub collection_method: InvoiceCollectionMethods,
    /// The company that issued this invoice.
    #[serde(default)]
    pub company: InvoiceCompany,
    /// The datetime the invoice was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The plan that this invoice charges for.
    pub current_plan: InvoiceCurrentPlan,
    /// The full name of the customer this invoice is addressed to. Null if no name is on file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
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
    pub line_items: Vec<InvoiceLineItemsItem>,
    /// The billing/mailing address associated with this invoice, if one was provided at creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<InvoiceMailingAddress>,
    /// The member that the invoice was created for. Null when the invoice is addressed to an email address with no member record behind it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<InvoiceMember>,
    /// The sequential invoice number for display purposes.
    #[serde(default)]
    pub number: String,
    /// The checkout URL where the customer can pay this invoice online, with their email address pre-filled and locked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_online_url: Option<String>,
    /// The payment that settled this invoice. Null while the invoice is unpaid, when the invoice was marked paid manually, and on a subscription renewal invoice, where the settling payment cannot yet be identified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<InvoicePayment>,
    /// Whether a payment on this invoice is still clearing. True while a delayed payment method such as ACH or SEPA settles, during which the invoice stays open and is not marked past due.
    #[serde(default)]
    pub payment_processing: bool,
    /// The product that this invoice was generated for.
    #[serde(default)]
    pub product: InvoiceProduct,
    /// The current payment status of the invoice, such as draft, open, paid, or void.
    pub status: InvoiceStatuses,
    /// The date that defines when the subscription billing cycle starts. When set on a renewal plan invoice, all future billing periods anchor to this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub subscription_billing_anchor_at: Option<DateTime<FixedOffset>>,
    /// The datetime the invoice was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The user this invoice is addressed to. Null if the user account has been removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<InvoiceUser>,
}

impl Invoice {
    pub fn builder() -> InvoiceBuilder {
        <InvoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceBuilder {
    automatically_finalizes_at: Option<DateTime<FixedOffset>>,
    charge_buyer_fee: Option<bool>,
    collection_method: Option<InvoiceCollectionMethods>,
    company: Option<InvoiceCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    current_plan: Option<InvoiceCurrentPlan>,
    customer_name: Option<String>,
    due_date: Option<DateTime<FixedOffset>>,
    email_address: Option<String>,
    fetch_invoice_token: Option<String>,
    id: Option<String>,
    line_items: Option<Vec<InvoiceLineItemsItem>>,
    mailing_address: Option<InvoiceMailingAddress>,
    member: Option<InvoiceMember>,
    number: Option<String>,
    pay_online_url: Option<String>,
    payment: Option<InvoicePayment>,
    payment_processing: Option<bool>,
    product: Option<InvoiceProduct>,
    status: Option<InvoiceStatuses>,
    subscription_billing_anchor_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<InvoiceUser>,
}

impl InvoiceBuilder {
    pub fn automatically_finalizes_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.automatically_finalizes_at = Some(value);
        self
    }

    pub fn charge_buyer_fee(mut self, value: bool) -> Self {
        self.charge_buyer_fee = Some(value);
        self
    }

    pub fn collection_method(mut self, value: InvoiceCollectionMethods) -> Self {
        self.collection_method = Some(value);
        self
    }

    pub fn company(mut self, value: InvoiceCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn current_plan(mut self, value: InvoiceCurrentPlan) -> Self {
        self.current_plan = Some(value);
        self
    }

    pub fn customer_name(mut self, value: impl Into<String>) -> Self {
        self.customer_name = Some(value.into());
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

    pub fn line_items(mut self, value: Vec<InvoiceLineItemsItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn mailing_address(mut self, value: InvoiceMailingAddress) -> Self {
        self.mailing_address = Some(value);
        self
    }

    pub fn member(mut self, value: InvoiceMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn pay_online_url(mut self, value: impl Into<String>) -> Self {
        self.pay_online_url = Some(value.into());
        self
    }

    pub fn payment(mut self, value: InvoicePayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn payment_processing(mut self, value: bool) -> Self {
        self.payment_processing = Some(value);
        self
    }

    pub fn product(mut self, value: InvoiceProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn status(mut self, value: InvoiceStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscription_billing_anchor_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.subscription_billing_anchor_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn user(mut self, value: InvoiceUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Invoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`charge_buyer_fee`](InvoiceBuilder::charge_buyer_fee)
    /// - [`collection_method`](InvoiceBuilder::collection_method)
    /// - [`company`](InvoiceBuilder::company)
    /// - [`created_at`](InvoiceBuilder::created_at)
    /// - [`current_plan`](InvoiceBuilder::current_plan)
    /// - [`fetch_invoice_token`](InvoiceBuilder::fetch_invoice_token)
    /// - [`id`](InvoiceBuilder::id)
    /// - [`line_items`](InvoiceBuilder::line_items)
    /// - [`number`](InvoiceBuilder::number)
    /// - [`payment_processing`](InvoiceBuilder::payment_processing)
    /// - [`product`](InvoiceBuilder::product)
    /// - [`status`](InvoiceBuilder::status)
    /// - [`updated_at`](InvoiceBuilder::updated_at)
    pub fn build(self) -> Result<Invoice, BuildError> {
        Ok(Invoice {
            automatically_finalizes_at: self.automatically_finalizes_at,
            charge_buyer_fee: self
                .charge_buyer_fee
                .ok_or_else(|| BuildError::missing_field("charge_buyer_fee"))?,
            collection_method: self
                .collection_method
                .ok_or_else(|| BuildError::missing_field("collection_method"))?,
            company: self
                .company
                .ok_or_else(|| BuildError::missing_field("company"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            current_plan: self
                .current_plan
                .ok_or_else(|| BuildError::missing_field("current_plan"))?,
            customer_name: self.customer_name,
            due_date: self.due_date,
            email_address: self.email_address,
            fetch_invoice_token: self
                .fetch_invoice_token
                .ok_or_else(|| BuildError::missing_field("fetch_invoice_token"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            mailing_address: self.mailing_address,
            member: self.member,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            pay_online_url: self.pay_online_url,
            payment: self.payment,
            payment_processing: self
                .payment_processing
                .ok_or_else(|| BuildError::missing_field("payment_processing"))?,
            product: self
                .product
                .ok_or_else(|| BuildError::missing_field("product"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subscription_billing_anchor_at: self.subscription_billing_anchor_at,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user,
        })
    }
}
