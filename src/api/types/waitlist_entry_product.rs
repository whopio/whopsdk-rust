pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaitlistEntryProduct {
    /// Product ID, prefixed `prod_`.
    #[serde(default)]
    pub id: String,
    /// Product display name.
    #[serde(default)]
    pub title: String,
}

impl WaitlistEntryProduct {
    pub fn builder() -> WaitlistEntryProductBuilder {
        <WaitlistEntryProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitlistEntryProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl WaitlistEntryProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WaitlistEntryProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WaitlistEntryProductBuilder::id)
    /// - [`title`](WaitlistEntryProductBuilder::title)
    pub fn build(self) -> Result<WaitlistEntryProduct, BuildError> {
        Ok(WaitlistEntryProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
