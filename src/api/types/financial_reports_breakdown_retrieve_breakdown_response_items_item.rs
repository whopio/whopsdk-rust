pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrieveBreakdownResponseItemsItem {
    #[serde(default)]
    pub amount: Money,
    /// How to draw the row's icon. `null` when the row has nothing to show (balances, adjustments, ad campaigns), so clients render no icon rather than a placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<RetrieveBreakdownResponseItemsItemAvatar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// An opaque identifier for this grouping within the breakdown.
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub name: String,
    pub object: RetrieveBreakdownResponseItemsItemObject,
    /// The prefixed identifier of the represented Whop resource, when one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl RetrieveBreakdownResponseItemsItem {
    pub fn builder() -> RetrieveBreakdownResponseItemsItemBuilder {
        <RetrieveBreakdownResponseItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBreakdownResponseItemsItemBuilder {
    amount: Option<Money>,
    avatar: Option<RetrieveBreakdownResponseItemsItemAvatar>,
    image_url: Option<String>,
    key: Option<String>,
    name: Option<String>,
    object: Option<RetrieveBreakdownResponseItemsItemObject>,
    resource_id: Option<String>,
}

impl RetrieveBreakdownResponseItemsItemBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn avatar(mut self, value: RetrieveBreakdownResponseItemsItemAvatar) -> Self {
        self.avatar = Some(value);
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: RetrieveBreakdownResponseItemsItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBreakdownResponseItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](RetrieveBreakdownResponseItemsItemBuilder::amount)
    /// - [`key`](RetrieveBreakdownResponseItemsItemBuilder::key)
    /// - [`name`](RetrieveBreakdownResponseItemsItemBuilder::name)
    /// - [`object`](RetrieveBreakdownResponseItemsItemBuilder::object)
    pub fn build(self) -> Result<RetrieveBreakdownResponseItemsItem, BuildError> {
        Ok(RetrieveBreakdownResponseItemsItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            avatar: self.avatar,
            image_url: self.image_url,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            resource_id: self.resource_id,
        })
    }
}
