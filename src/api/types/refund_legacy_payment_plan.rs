pub use crate::prelude::*;

/// The plan attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RefundLegacyPaymentPlan {
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
    /// Custom key-value pairs stored on the plan. Included in webhook payloads for payment and membership events. Max 50 keys, 100 chars per key, 500 chars per string value. The reserved keys `custom_cta` and `custom_cta_url`, when set, override the product's checkout call to action for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl RefundLegacyPaymentPlan {
    pub fn builder() -> RefundLegacyPaymentPlanBuilder {
        <RefundLegacyPaymentPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLegacyPaymentPlanBuilder {
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl RefundLegacyPaymentPlanBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundLegacyPaymentPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundLegacyPaymentPlanBuilder::id)
    pub fn build(self) -> Result<RefundLegacyPaymentPlan, BuildError> {
        Ok(RefundLegacyPaymentPlan {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
        })
    }
}
