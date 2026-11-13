pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub amount_due: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<PaymentAddress>,
    #[serde(default)]
    pub buyer_fee_amount: Money,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub buyer_fee_percentage: f64,
    /// A signed link that opens this invoice in a checkout session so the buyer can pay it. Pass it as `link` to `POST /checkout_sessions`. Present while the invoice is open. `null` in any other status, for an invoice billed to a business buyer, and for an invoice whose membership was canceled. Also `null` unless you have `payment:charge`, or `invoice:create` or `invoice:update` on an invoice with no `payment_method_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_link: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// True when `DELETE /invoices/{id}` can delete this invoice. A draft qualifies unless it is scheduled or bills a subscription. A void invoice qualifies unless it collected money or still awaits a bank wire.
    #[serde(default)]
    pub deletable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,
    #[serde(default)]
    pub line_items: Vec<InvoiceLineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The membership this invoice bills, prefixed `mem_`. For a renewal invoice, it is the subscription that renews. For an invoice with a recurring item, it is the membership created when the invoice is issued. Null for one-off invoices, for a draft not yet issued, and without the member:basic:read permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    /// A note to the customer, shown on the invoice email, PDF, and checkout page with its line breaks kept. `null` when none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub number: String,
    pub object: InvoiceObject,
    #[serde(default)]
    pub overdue: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    /// The payment methods the customer can choose from when paying through the invoice's checkout link. It takes precedence over the invoice's variants and the account. `null` when the invoice uses its variants' payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<CheckoutSessionPaymentMethodConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(default)]
    pub recipient_emails: Vec<String>,
    /// When the invoice's saved payment method will be charged, as an ISO 8601 timestamp. It is the `charge_at` the invoice was sent or issued with. Without one, it is `due_at` when an invoice with a `payment_method_id` is sent or issued before it is due. On a draft, it is the `charge_at` of its scheduled send. `null` otherwise, once the charge starts, and once the invoice is paid, void, or uncollectible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_charge_at: Option<String>,
    /// The `collection_method` a draft's scheduled send applies when it fires. `null` when the draft has no scheduled send or it was scheduled without one, and once the invoice is issued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_collection_method: Option<InvoiceScheduledCollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_send_at: Option<String>,
    /// Whether this draft is issued automatically when its `due_at` arrives. Issuing it charges its `payment_method_id`, or emails its payment link to its `recipient_emails`. `false` for an invoice that is not a draft and for a draft with a scheduled send. `false` while sending the draft would fail for missing recipients or an unavailable payment link. `false` for a draft that had no future `due_at` when it was last saved. `false` for a draft created before 2026-11-12T22:00:00Z.
    #[serde(default)]
    pub sends_on_due_date: bool,
    pub status: InvoiceStatus,
    #[serde(default)]
    pub subtotal: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Money>,
    #[serde(default)]
    pub total: Money,
    pub totals_status: InvoiceTotalsStatus,
    #[serde(default)]
    pub updated_at: String,
}

impl Invoice {
    pub fn builder() -> InvoiceBuilder {
        <InvoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceBuilder {
    account_id: Option<String>,
    amount_due: Option<Money>,
    billing_address: Option<PaymentAddress>,
    buyer_fee_amount: Option<Money>,
    buyer_fee_percentage: Option<f64>,
    checkout_link: Option<String>,
    created_at: Option<String>,
    customer_name: Option<String>,
    deletable: Option<bool>,
    due_at: Option<String>,
    email_address: Option<String>,
    id: Option<String>,
    invoice_prefix: Option<String>,
    line_items: Option<Vec<InvoiceLineItem>>,
    mailing_address_id: Option<String>,
    member_id: Option<String>,
    membership_id: Option<String>,
    memo: Option<String>,
    metadata: Option<HashMap<String, String>>,
    number: Option<String>,
    object: Option<InvoiceObject>,
    overdue: Option<bool>,
    paid_at: Option<String>,
    payment_method_configuration: Option<CheckoutSessionPaymentMethodConfiguration>,
    payment_method_id: Option<String>,
    recipient_emails: Option<Vec<String>>,
    scheduled_charge_at: Option<String>,
    scheduled_collection_method: Option<InvoiceScheduledCollectionMethod>,
    scheduled_send_at: Option<String>,
    sends_on_due_date: Option<bool>,
    status: Option<InvoiceStatus>,
    subtotal: Option<Money>,
    tax_amount: Option<Money>,
    total: Option<Money>,
    totals_status: Option<InvoiceTotalsStatus>,
    updated_at: Option<String>,
}

impl InvoiceBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount_due(mut self, value: Money) -> Self {
        self.amount_due = Some(value);
        self
    }

    pub fn billing_address(mut self, value: PaymentAddress) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn buyer_fee_amount(mut self, value: Money) -> Self {
        self.buyer_fee_amount = Some(value);
        self
    }

    pub fn buyer_fee_percentage(mut self, value: f64) -> Self {
        self.buyer_fee_percentage = Some(value);
        self
    }

    pub fn checkout_link(mut self, value: impl Into<String>) -> Self {
        self.checkout_link = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn customer_name(mut self, value: impl Into<String>) -> Self {
        self.customer_name = Some(value.into());
        self
    }

    pub fn deletable(mut self, value: bool) -> Self {
        self.deletable = Some(value);
        self
    }

    pub fn due_at(mut self, value: impl Into<String>) -> Self {
        self.due_at = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_prefix(mut self, value: impl Into<String>) -> Self {
        self.invoice_prefix = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<InvoiceLineItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn mailing_address_id(mut self, value: impl Into<String>) -> Self {
        self.mailing_address_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn membership_id(mut self, value: impl Into<String>) -> Self {
        self.membership_id = Some(value.into());
        self
    }

    pub fn memo(mut self, value: impl Into<String>) -> Self {
        self.memo = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn object(mut self, value: InvoiceObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn overdue(mut self, value: bool) -> Self {
        self.overdue = Some(value);
        self
    }

    pub fn paid_at(mut self, value: impl Into<String>) -> Self {
        self.paid_at = Some(value.into());
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: CheckoutSessionPaymentMethodConfiguration,
    ) -> Self {
        self.payment_method_configuration = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn recipient_emails(mut self, value: Vec<String>) -> Self {
        self.recipient_emails = Some(value);
        self
    }

    pub fn scheduled_charge_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_charge_at = Some(value.into());
        self
    }

    pub fn scheduled_collection_method(mut self, value: InvoiceScheduledCollectionMethod) -> Self {
        self.scheduled_collection_method = Some(value);
        self
    }

    pub fn scheduled_send_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_send_at = Some(value.into());
        self
    }

    pub fn sends_on_due_date(mut self, value: bool) -> Self {
        self.sends_on_due_date = Some(value);
        self
    }

    pub fn status(mut self, value: InvoiceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subtotal(mut self, value: Money) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn tax_amount(mut self, value: Money) -> Self {
        self.tax_amount = Some(value);
        self
    }

    pub fn total(mut self, value: Money) -> Self {
        self.total = Some(value);
        self
    }

    pub fn totals_status(mut self, value: InvoiceTotalsStatus) -> Self {
        self.totals_status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Invoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](InvoiceBuilder::account_id)
    /// - [`amount_due`](InvoiceBuilder::amount_due)
    /// - [`buyer_fee_amount`](InvoiceBuilder::buyer_fee_amount)
    /// - [`buyer_fee_percentage`](InvoiceBuilder::buyer_fee_percentage)
    /// - [`created_at`](InvoiceBuilder::created_at)
    /// - [`deletable`](InvoiceBuilder::deletable)
    /// - [`id`](InvoiceBuilder::id)
    /// - [`line_items`](InvoiceBuilder::line_items)
    /// - [`metadata`](InvoiceBuilder::metadata)
    /// - [`number`](InvoiceBuilder::number)
    /// - [`object`](InvoiceBuilder::object)
    /// - [`overdue`](InvoiceBuilder::overdue)
    /// - [`recipient_emails`](InvoiceBuilder::recipient_emails)
    /// - [`sends_on_due_date`](InvoiceBuilder::sends_on_due_date)
    /// - [`status`](InvoiceBuilder::status)
    /// - [`subtotal`](InvoiceBuilder::subtotal)
    /// - [`total`](InvoiceBuilder::total)
    /// - [`totals_status`](InvoiceBuilder::totals_status)
    /// - [`updated_at`](InvoiceBuilder::updated_at)
    pub fn build(self) -> Result<Invoice, BuildError> {
        Ok(Invoice {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount_due: self
                .amount_due
                .ok_or_else(|| BuildError::missing_field("amount_due"))?,
            billing_address: self.billing_address,
            buyer_fee_amount: self
                .buyer_fee_amount
                .ok_or_else(|| BuildError::missing_field("buyer_fee_amount"))?,
            buyer_fee_percentage: self
                .buyer_fee_percentage
                .ok_or_else(|| BuildError::missing_field("buyer_fee_percentage"))?,
            checkout_link: self.checkout_link,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            customer_name: self.customer_name,
            deletable: self
                .deletable
                .ok_or_else(|| BuildError::missing_field("deletable"))?,
            due_at: self.due_at,
            email_address: self.email_address,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_prefix: self.invoice_prefix,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            mailing_address_id: self.mailing_address_id,
            member_id: self.member_id,
            membership_id: self.membership_id,
            memo: self.memo,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            overdue: self
                .overdue
                .ok_or_else(|| BuildError::missing_field("overdue"))?,
            paid_at: self.paid_at,
            payment_method_configuration: self.payment_method_configuration,
            payment_method_id: self.payment_method_id,
            recipient_emails: self
                .recipient_emails
                .ok_or_else(|| BuildError::missing_field("recipient_emails"))?,
            scheduled_charge_at: self.scheduled_charge_at,
            scheduled_collection_method: self.scheduled_collection_method,
            scheduled_send_at: self.scheduled_send_at,
            sends_on_due_date: self
                .sends_on_due_date
                .ok_or_else(|| BuildError::missing_field("sends_on_due_date"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subtotal: self
                .subtotal
                .ok_or_else(|| BuildError::missing_field("subtotal"))?,
            tax_amount: self.tax_amount,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            totals_status: self
                .totals_status
                .ok_or_else(|| BuildError::missing_field("totals_status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
