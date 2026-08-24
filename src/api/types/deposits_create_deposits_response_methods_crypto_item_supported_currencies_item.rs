pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItem {
    /// Token icon URL. Null when no icon is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Token symbol.
    pub name: CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemName,
}

impl CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItem {
    pub fn builder() -> CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemBuilder {
        <CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemBuilder {
    icon_url: Option<String>,
    name: Option<CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemName>,
}

impl CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemBuilder {
    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn name(
        mut self,
        value: CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemName,
    ) -> Self {
        self.name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItemBuilder::name)
    pub fn build(
        self,
    ) -> Result<CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItem, BuildError> {
        Ok(
            CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItem {
                icon_url: self.icon_url,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            },
        )
    }
}
