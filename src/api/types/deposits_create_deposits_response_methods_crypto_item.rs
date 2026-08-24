pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateDepositsResponseMethodsCryptoItem {
    /// Address to send funds to on this network. Null when the provider has not issued one yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_address: Option<String>,
    /// Network icon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Network display name.
    pub name: CreateDepositsResponseMethodsCryptoItemName,
    /// Tokens accepted for deposit on this network.
    #[serde(default)]
    pub supported_currencies: Vec<CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItem>,
}

impl CreateDepositsResponseMethodsCryptoItem {
    pub fn builder() -> CreateDepositsResponseMethodsCryptoItemBuilder {
        <CreateDepositsResponseMethodsCryptoItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsResponseMethodsCryptoItemBuilder {
    deposit_address: Option<String>,
    icon_url: Option<String>,
    name: Option<CreateDepositsResponseMethodsCryptoItemName>,
    supported_currencies:
        Option<Vec<CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItem>>,
}

impl CreateDepositsResponseMethodsCryptoItemBuilder {
    pub fn deposit_address(mut self, value: impl Into<String>) -> Self {
        self.deposit_address = Some(value.into());
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: CreateDepositsResponseMethodsCryptoItemName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn supported_currencies(
        mut self,
        value: Vec<CreateDepositsResponseMethodsCryptoItemSupportedCurrenciesItem>,
    ) -> Self {
        self.supported_currencies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsResponseMethodsCryptoItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateDepositsResponseMethodsCryptoItemBuilder::name)
    /// - [`supported_currencies`](CreateDepositsResponseMethodsCryptoItemBuilder::supported_currencies)
    pub fn build(self) -> Result<CreateDepositsResponseMethodsCryptoItem, BuildError> {
        Ok(CreateDepositsResponseMethodsCryptoItem {
            deposit_address: self.deposit_address,
            icon_url: self.icon_url,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            supported_currencies: self
                .supported_currencies
                .ok_or_else(|| BuildError::missing_field("supported_currencies"))?,
        })
    }
}
