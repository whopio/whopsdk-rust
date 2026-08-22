pub use crate::prelude::*;

/// The product this payment was made for
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RefundPaymentProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// Custom key-value pairs stored on the product and included in payment and membership webhook payloads. Max 50 keys, 100 characters per key, 500 characters per string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl RefundPaymentProduct {
    pub fn builder() -> RefundPaymentProductBuilder {
        <RefundPaymentProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundPaymentProductBuilder {
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl RefundPaymentProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundPaymentProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundPaymentProductBuilder::id)
    pub fn build(self) -> Result<RefundPaymentProduct, BuildError> {
        Ok(RefundPaymentProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
        })
    }
}
