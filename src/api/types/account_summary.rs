pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountSummary {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub id: String,
    /// Account display name.
    #[serde(default)]
    pub title: String,
}

impl AccountSummary {
    pub fn builder() -> AccountSummaryBuilder {
        <AccountSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSummaryBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl AccountSummaryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountSummaryBuilder::id)
    /// - [`title`](AccountSummaryBuilder::title)
    pub fn build(self) -> Result<AccountSummary, BuildError> {
        Ok(AccountSummary {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
