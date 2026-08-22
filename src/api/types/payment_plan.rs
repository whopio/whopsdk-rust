pub use crate::prelude::*;

/// The plan attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentPlan {
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
    /// A personal description or notes section for the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// Custom key-value pairs stored on the plan. Included in webhook payloads for payment and membership events. Max 50 keys, 100 chars per key, 500 chars per string value. The reserved keys `custom_cta` and `custom_cta_url`, when set, override the product's checkout call to action for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl PaymentPlan {
    pub fn builder() -> PaymentPlanBuilder {
        <PaymentPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPlanBuilder {
    id: Option<String>,
    internal_notes: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl PaymentPlanBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn internal_notes(mut self, value: impl Into<String>) -> Self {
        self.internal_notes = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentPlanBuilder::id)
    pub fn build(self) -> Result<PaymentPlan, BuildError> {
        Ok(PaymentPlan {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            internal_notes: self.internal_notes,
            metadata: self.metadata,
        })
    }
}
