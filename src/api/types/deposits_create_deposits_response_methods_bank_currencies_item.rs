pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDepositsResponseMethodsBankCurrenciesItem {
    /// Bank account number for deposits in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Currency supported by these bank instructions.
    #[serde(default)]
    pub currency: String,
    /// Receiving bank address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_bank_address: Option<String>,
    /// Receiving bank name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_bank_name: Option<String>,
    /// Beneficiary name to use for transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_beneficiary_name: Option<String>,
    /// Reference to include with bank transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_reference: Option<String>,
    /// Active deposit rails for this currency.
    #[serde(default)]
    pub rails: Vec<CreateDepositsResponseMethodsBankCurrenciesItemRailsItem>,
    /// Bank routing number for deposits in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    /// SWIFT/BIC code for international wires, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_bic: Option<String>,
}

impl CreateDepositsResponseMethodsBankCurrenciesItem {
    pub fn builder() -> CreateDepositsResponseMethodsBankCurrenciesItemBuilder {
        <CreateDepositsResponseMethodsBankCurrenciesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsResponseMethodsBankCurrenciesItemBuilder {
    account_number: Option<String>,
    currency: Option<String>,
    deposit_bank_address: Option<String>,
    deposit_bank_name: Option<String>,
    deposit_beneficiary_name: Option<String>,
    deposit_reference: Option<String>,
    rails: Option<Vec<CreateDepositsResponseMethodsBankCurrenciesItemRailsItem>>,
    routing_number: Option<String>,
    swift_bic: Option<String>,
}

impl CreateDepositsResponseMethodsBankCurrenciesItemBuilder {
    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn deposit_bank_address(mut self, value: impl Into<String>) -> Self {
        self.deposit_bank_address = Some(value.into());
        self
    }

    pub fn deposit_bank_name(mut self, value: impl Into<String>) -> Self {
        self.deposit_bank_name = Some(value.into());
        self
    }

    pub fn deposit_beneficiary_name(mut self, value: impl Into<String>) -> Self {
        self.deposit_beneficiary_name = Some(value.into());
        self
    }

    pub fn deposit_reference(mut self, value: impl Into<String>) -> Self {
        self.deposit_reference = Some(value.into());
        self
    }

    pub fn rails(
        mut self,
        value: Vec<CreateDepositsResponseMethodsBankCurrenciesItemRailsItem>,
    ) -> Self {
        self.rails = Some(value);
        self
    }

    pub fn routing_number(mut self, value: impl Into<String>) -> Self {
        self.routing_number = Some(value.into());
        self
    }

    pub fn swift_bic(mut self, value: impl Into<String>) -> Self {
        self.swift_bic = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsResponseMethodsBankCurrenciesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CreateDepositsResponseMethodsBankCurrenciesItemBuilder::currency)
    /// - [`rails`](CreateDepositsResponseMethodsBankCurrenciesItemBuilder::rails)
    pub fn build(self) -> Result<CreateDepositsResponseMethodsBankCurrenciesItem, BuildError> {
        Ok(CreateDepositsResponseMethodsBankCurrenciesItem {
            account_number: self.account_number,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            deposit_bank_address: self.deposit_bank_address,
            deposit_bank_name: self.deposit_bank_name,
            deposit_beneficiary_name: self.deposit_beneficiary_name,
            deposit_reference: self.deposit_reference,
            rails: self
                .rails
                .ok_or_else(|| BuildError::missing_field("rails"))?,
            routing_number: self.routing_number,
            swift_bic: self.swift_bic,
        })
    }
}
