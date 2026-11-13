pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IssueInvoicesRequest {
    /// When to charge the draft's `payment_method_id`, as an ISO 8601 timestamp in the future. Only for a draft with a `payment_method_id` and a total above zero. Omit to charge on `due_at` if it is still ahead, or right away otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_at: Option<DateTime<FixedOffset>>,
    /// How the subscription this invoice starts collects each renewal. `charge_automatically` charges the payment method that paid the invoice each period: the draft's `payment_method_id`, or the one the customer pays the invoice link with. `send_invoice` issues a numbered invoice each period, due on the renewal date and emailed with its payment link to the latest invoice's `recipient_emails`, so it needs at least one recipient and isn't available for invoices billed to a business. Defaults to `charge_automatically`. Only for a draft with a recurring item. A scheduled send applies it when it fires (see `scheduled_collection_method`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<IssueInvoicesRequestCollectionMethod>,
}

impl IssueInvoicesRequest {
    pub fn builder() -> IssueInvoicesRequestBuilder {
        <IssueInvoicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IssueInvoicesRequestBuilder {
    charge_at: Option<DateTime<FixedOffset>>,
    collection_method: Option<IssueInvoicesRequestCollectionMethod>,
}

impl IssueInvoicesRequestBuilder {
    pub fn charge_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.charge_at = Some(value);
        self
    }

    pub fn collection_method(mut self, value: IssueInvoicesRequestCollectionMethod) -> Self {
        self.collection_method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IssueInvoicesRequest`].
    pub fn build(self) -> Result<IssueInvoicesRequest, BuildError> {
        Ok(IssueInvoicesRequest {
            charge_at: self.charge_at,
            collection_method: self.collection_method,
        })
    }
}
