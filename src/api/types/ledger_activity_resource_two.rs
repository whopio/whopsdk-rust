pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceTwo {
    /// Bounty ID.
    #[serde(default)]
    pub id: String,
    pub object: LedgerActivityResourceTwoObject,
    /// Bounty lifecycle status.
    #[serde(default)]
    pub status: String,
    /// Bounty title.
    #[serde(default)]
    pub title: String,
}

impl LedgerActivityResourceTwo {
    pub fn builder() -> LedgerActivityResourceTwoBuilder {
        <LedgerActivityResourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceTwoBuilder {
    id: Option<String>,
    object: Option<LedgerActivityResourceTwoObject>,
    status: Option<String>,
    title: Option<String>,
}

impl LedgerActivityResourceTwoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceTwoObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityResourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceTwoBuilder::id)
    /// - [`object`](LedgerActivityResourceTwoBuilder::object)
    /// - [`status`](LedgerActivityResourceTwoBuilder::status)
    /// - [`title`](LedgerActivityResourceTwoBuilder::title)
    pub fn build(self) -> Result<LedgerActivityResourceTwo, BuildError> {
        Ok(LedgerActivityResourceTwo {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
