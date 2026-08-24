pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListSwapsResponseDataItem {
    /// Account ID that owns the wallet used for the swap.
    #[serde(default)]
    pub account_id: String,
    /// Latest error returned for a failed swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Swap ID.
    #[serde(default)]
    pub id: String,
    pub object: ListSwapsResponseDataItemObject,
    /// Current swap status. `complete` and `failed` are terminal.
    pub status: ListSwapsResponseDataItemStatus,
    /// On-chain transaction hashes produced by the swap.
    #[serde(default)]
    pub tx_hashes: Vec<String>,
}

impl ListSwapsResponseDataItem {
    pub fn builder() -> ListSwapsResponseDataItemBuilder {
        <ListSwapsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSwapsResponseDataItemBuilder {
    account_id: Option<String>,
    error: Option<String>,
    id: Option<String>,
    object: Option<ListSwapsResponseDataItemObject>,
    status: Option<ListSwapsResponseDataItemStatus>,
    tx_hashes: Option<Vec<String>>,
}

impl ListSwapsResponseDataItemBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ListSwapsResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn status(mut self, value: ListSwapsResponseDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tx_hashes(mut self, value: Vec<String>) -> Self {
        self.tx_hashes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSwapsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ListSwapsResponseDataItemBuilder::account_id)
    /// - [`id`](ListSwapsResponseDataItemBuilder::id)
    /// - [`object`](ListSwapsResponseDataItemBuilder::object)
    /// - [`status`](ListSwapsResponseDataItemBuilder::status)
    /// - [`tx_hashes`](ListSwapsResponseDataItemBuilder::tx_hashes)
    pub fn build(self) -> Result<ListSwapsResponseDataItem, BuildError> {
        Ok(ListSwapsResponseDataItem {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            error: self.error,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tx_hashes: self
                .tx_hashes
                .ok_or_else(|| BuildError::missing_field("tx_hashes"))?,
        })
    }
}
