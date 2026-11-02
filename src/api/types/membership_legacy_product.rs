pub use crate::prelude::*;

/// The product this membership grants access to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MembershipLegacyProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// Custom key-value pairs stored on the product and included in payment and membership webhook payloads. Max 50 keys, 100 characters per key, 500 characters per string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl MembershipLegacyProduct {
    pub fn builder() -> MembershipLegacyProductBuilder {
        <MembershipLegacyProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipLegacyProductBuilder {
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    title: Option<String>,
}

impl MembershipLegacyProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipLegacyProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipLegacyProductBuilder::id)
    /// - [`title`](MembershipLegacyProductBuilder::title)
    pub fn build(self) -> Result<MembershipLegacyProduct, BuildError> {
        Ok(MembershipLegacyProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
