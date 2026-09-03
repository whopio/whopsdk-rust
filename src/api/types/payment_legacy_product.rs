pub use crate::prelude::*;

/// The product this payment was made for
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentLegacyProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// Custom key-value pairs stored on the product and included in payment and membership webhook payloads. Max 50 keys, 100 characters per key, 500 characters per string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// URL slug in the product's public link, e.g. `pickaxe-analytics` in whop.com/company/pickaxe-analytics.
    #[serde(default)]
    pub route: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl PaymentLegacyProduct {
    pub fn builder() -> PaymentLegacyProductBuilder {
        <PaymentLegacyProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyProductBuilder {
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    route: Option<String>,
    title: Option<String>,
}

impl PaymentLegacyProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentLegacyProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentLegacyProductBuilder::id)
    /// - [`route`](PaymentLegacyProductBuilder::route)
    /// - [`title`](PaymentLegacyProductBuilder::title)
    pub fn build(self) -> Result<PaymentLegacyProduct, BuildError> {
        Ok(PaymentLegacyProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
