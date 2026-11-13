pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateInvoicesRequest {
    /// The issuing account.
    #[serde(default)]
    pub account_id: String,
    /// Create a billing address linked to this member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CreateInvoicesRequestBillingAddress>,
    /// The buyer's adjustable fee in percentage points, with at most two decimal places: `0` for none, or between 5 and 100. The account keeps it, and Whop waives its take rate on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub buyer_fee_percentage: Option<f64>,
    /// The contact name when creating an email-only member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// When payment is due, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<DateTime<FixedOffset>>,
    /// Resolve or create the member for this email, creating a Whop user when none exists. Mutually exclusive with member_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The variants being invoiced, all in the same currency. Each variant may appear once. Use quantity for multiple units. A recurring variant must belong to a product. Its first invoice charges `initial_price` plus `renewal_price`. Later invoices charge only `renewal_price`. Paying the first invoice starts a paid billing period immediately, without the variant's trial. The subscription uses the `collection_method` the invoice was sent or issued with. A finite payment count includes the first paid invoice. With several items, every item that has a product must be a regular product, and all items need compatible payment, tax, and access settings.
    #[serde(default)]
    pub items: Vec<CreateInvoicesRequestItemsItem>,
    /// An address associated with the member. Mutually exclusive with billing_address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address_id: Option<String>,
    /// The billed customer, a member of the account, prefixed `mber_`. It can't change once the draft is created. Changing email recipients does not change this buyer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// A note to the customer, shown on the invoice email, PDF, and checkout page with its line breaks kept. Trimmed; `null` or blank clears it. Can be updated on draft, open, or uncollectible invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// Up to 50 caller-owned string key/value pairs. The `invoice_memo` key is reserved and ignored; set `memo` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// The payment methods the customer can choose from when paying through the invoice's checkout link, in the shape a variant or checkout configuration takes. It takes precedence over the invoice's variants and the account, and changes neither. Send both lists. `null` uses its variants' payment methods. Can be updated on drafts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<CreateInvoicesRequestPaymentMethodConfiguration>,
    /// A saved payment method belonging to the member, prefixed `payt_`. Sending or issuing the invoice charges this method at the `charge_at` passed to that call, otherwise on `due_at`, or right away when there is no `due_at` or it has passed. Sending also emails the invoice now when the charge is later; issuing does not. Setting it needs `payment:charge`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Email addresses saved for invoice delivery, independent of the billed customer. Saving them sends no email. Defaults to the customer email when omitted on create. Can be updated on draft, open, or uncollectible invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_emails: Option<Vec<String>>,
}

impl CreateInvoicesRequest {
    pub fn builder() -> CreateInvoicesRequestBuilder {
        <CreateInvoicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoicesRequestBuilder {
    account_id: Option<String>,
    billing_address: Option<CreateInvoicesRequestBillingAddress>,
    buyer_fee_percentage: Option<f64>,
    customer_name: Option<String>,
    due_at: Option<DateTime<FixedOffset>>,
    email_address: Option<String>,
    items: Option<Vec<CreateInvoicesRequestItemsItem>>,
    mailing_address_id: Option<String>,
    member_id: Option<String>,
    memo: Option<String>,
    metadata: Option<HashMap<String, String>>,
    payment_method_configuration: Option<CreateInvoicesRequestPaymentMethodConfiguration>,
    payment_method_id: Option<String>,
    recipient_emails: Option<Vec<String>>,
}

impl CreateInvoicesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn billing_address(mut self, value: CreateInvoicesRequestBillingAddress) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn buyer_fee_percentage(mut self, value: f64) -> Self {
        self.buyer_fee_percentage = Some(value);
        self
    }

    pub fn customer_name(mut self, value: impl Into<String>) -> Self {
        self.customer_name = Some(value.into());
        self
    }

    pub fn due_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.due_at = Some(value);
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<CreateInvoicesRequestItemsItem>) -> Self {
        self.items = Some(value);
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

    pub fn memo(mut self, value: impl Into<String>) -> Self {
        self.memo = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: CreateInvoicesRequestPaymentMethodConfiguration,
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

    /// Consumes the builder and constructs a [`CreateInvoicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateInvoicesRequestBuilder::account_id)
    /// - [`items`](CreateInvoicesRequestBuilder::items)
    pub fn build(self) -> Result<CreateInvoicesRequest, BuildError> {
        Ok(CreateInvoicesRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            billing_address: self.billing_address,
            buyer_fee_percentage: self.buyer_fee_percentage,
            customer_name: self.customer_name,
            due_at: self.due_at,
            email_address: self.email_address,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            mailing_address_id: self.mailing_address_id,
            member_id: self.member_id,
            memo: self.memo,
            metadata: self.metadata,
            payment_method_configuration: self.payment_method_configuration,
            payment_method_id: self.payment_method_id,
            recipient_emails: self.recipient_emails,
        })
    }
}
