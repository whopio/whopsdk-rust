pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceOwner {
    /// Ledger account ID.
    #[serde(default)]
    pub id: String,
    pub object: LedgerActivityResourceOwnerObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<LedgerActivityResourceOwnerOwner>,
}

impl LedgerActivityResourceOwner {
    pub fn builder() -> LedgerActivityResourceOwnerBuilder {
        <LedgerActivityResourceOwnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceOwnerBuilder {
    id: Option<String>,
    object: Option<LedgerActivityResourceOwnerObject>,
    owner: Option<LedgerActivityResourceOwnerOwner>,
}

impl LedgerActivityResourceOwnerBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceOwnerObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn owner(mut self, value: LedgerActivityResourceOwnerOwner) -> Self {
        self.owner = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityResourceOwner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceOwnerBuilder::id)
    /// - [`object`](LedgerActivityResourceOwnerBuilder::object)
    pub fn build(self) -> Result<LedgerActivityResourceOwner, BuildError> {
        Ok(LedgerActivityResourceOwner {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            owner: self.owner,
        })
    }
}
