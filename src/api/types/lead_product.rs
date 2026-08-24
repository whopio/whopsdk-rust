pub use crate::prelude::*;

/// The product the lead expressed interest in. Null if the lead is not associated with a specific product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeadProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl LeadProduct {
    pub fn builder() -> LeadProductBuilder {
        <LeadProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl LeadProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeadProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LeadProductBuilder::id)
    /// - [`title`](LeadProductBuilder::title)
    pub fn build(self) -> Result<LeadProduct, BuildError> {
        Ok(LeadProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
