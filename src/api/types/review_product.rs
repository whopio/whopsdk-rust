pub use crate::prelude::*;

/// The product that this review was written for.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReviewProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl ReviewProduct {
    pub fn builder() -> ReviewProductBuilder {
        <ReviewProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl ReviewProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReviewProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReviewProductBuilder::id)
    /// - [`title`](ReviewProductBuilder::title)
    pub fn build(self) -> Result<ReviewProduct, BuildError> {
        Ok(ReviewProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
