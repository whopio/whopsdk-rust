pub use crate::prelude::*;

/// The product the lead expressed interest in. Null if the lead is not associated with a specific product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeadListItemProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl LeadListItemProduct {
    pub fn builder() -> LeadListItemProductBuilder {
        <LeadListItemProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadListItemProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl LeadListItemProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeadListItemProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LeadListItemProductBuilder::id)
    /// - [`title`](LeadListItemProductBuilder::title)
    pub fn build(self) -> Result<LeadListItemProduct, BuildError> {
        Ok(LeadListItemProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
