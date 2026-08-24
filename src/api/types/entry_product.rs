pub use crate::prelude::*;

/// The product associated with this entry's waitlisted plan. Null if the plan is not tied to a product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntryProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl EntryProduct {
    pub fn builder() -> EntryProductBuilder {
        <EntryProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntryProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl EntryProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntryProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EntryProductBuilder::id)
    /// - [`title`](EntryProductBuilder::title)
    pub fn build(self) -> Result<EntryProduct, BuildError> {
        Ok(EntryProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
