pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateInvoicesRequest {
    /// The date and time when the invoice will be automatically finalized. For charge_automatically, triggers an automatic charge. For send_invoice, sends the invoice email at the specified time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_finalizes_at: Option<DateTime<FixedOffset>>,
    /// Inline billing address to create or update a mailing address for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<UpdateInvoicesRequestBillingAddress>,
    /// Whether to charge the customer a buyer fee on this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_buyer_fee: Option<bool>,
    /// How the invoice should be collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<InvoiceCollectionMethods>,
    /// The name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// The date by which the invoice must be paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateTime<FixedOffset>>,
    /// The email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Line items that break down the invoice total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<UpdateInvoicesRequestLineItemsItem>>,
    /// The unique identifier of an existing mailing address to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address_id: Option<String>,
    /// The unique identifier of a member to assign as the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The unique identifier of the payment method to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// Updated plan attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<UpdateInvoicesRequestPlan>,
    /// The unique identifier of an existing product to attach to this invoice. Only allowed while the invoice is still a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The date that defines when the subscription billing cycle should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_billing_anchor_at: Option<DateTime<FixedOffset>>,
}

impl UpdateInvoicesRequest {
    pub fn builder() -> UpdateInvoicesRequestBuilder {
        <UpdateInvoicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInvoicesRequestBuilder {
    automatically_finalizes_at: Option<DateTime<FixedOffset>>,
    billing_address: Option<UpdateInvoicesRequestBillingAddress>,
    charge_buyer_fee: Option<bool>,
    collection_method: Option<InvoiceCollectionMethods>,
    customer_name: Option<String>,
    due_date: Option<DateTime<FixedOffset>>,
    email_address: Option<String>,
    line_items: Option<Vec<UpdateInvoicesRequestLineItemsItem>>,
    mailing_address_id: Option<String>,
    member_id: Option<String>,
    payment_method_id: Option<String>,
    plan: Option<UpdateInvoicesRequestPlan>,
    product_id: Option<String>,
    subscription_billing_anchor_at: Option<DateTime<FixedOffset>>,
}

impl UpdateInvoicesRequestBuilder {
    pub fn automatically_finalizes_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.automatically_finalizes_at = Some(value);
        self
    }

    pub fn billing_address(mut self, value: UpdateInvoicesRequestBillingAddress) -> Self {
        self.billing_address = Some(value);
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

    pub fn line_items(mut self, value: Vec<UpdateInvoicesRequestLineItemsItem>) -> Self {
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

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn plan(mut self, value: UpdateInvoicesRequestPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn subscription_billing_anchor_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.subscription_billing_anchor_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateInvoicesRequest`].
    pub fn build(self) -> Result<UpdateInvoicesRequest, BuildError> {
        Ok(UpdateInvoicesRequest {
            automatically_finalizes_at: self.automatically_finalizes_at,
            billing_address: self.billing_address,
            charge_buyer_fee: self.charge_buyer_fee,
            collection_method: self.collection_method,
            customer_name: self.customer_name,
            due_date: self.due_date,
            email_address: self.email_address,
            line_items: self.line_items,
            mailing_address_id: self.mailing_address_id,
            member_id: self.member_id,
            payment_method_id: self.payment_method_id,
            plan: self.plan,
            product_id: self.product_id,
            subscription_billing_anchor_at: self.subscription_billing_anchor_at,
        })
    }
}
