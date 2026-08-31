pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentMethodDisplay {
    /// Present when the category is `bank_debit`. Carries the account's last four when the linking provider surfaced it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_debit: Option<PaymentMethodDisplayPreview>,
    /// Present when the category is `card`. What the collection surface displayed — the token has not been charged, so this is the buyer's claim, not the vault's record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodDisplayPreview>,
    /// The family the type belongs to.
    pub category: PaymentMethodDisplayCategory,
    /// Human-readable label for the method, e.g. `Visa •••• 4242`.
    #[serde(default)]
    pub display_name: String,
    /// The saved payment method this preview came from, or `null` when the buyer supplied a new one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Present when the category is `saved` and the stored method is a card. Unlike the other previews this is the vault's own record, not a claim from the collection surface. Absent for a balance, which has no instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved: Option<PaymentMethodDisplayPreview>,
    /// The payment method type, e.g. `card`, `apple_pay`, `klarna`.
    #[serde(default)]
    pub r#type: String,
    /// Present when the category is `wallet`. Carries the backing card's brand and last four when the wallet surfaced them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<PaymentMethodDisplayPreview>,
}

impl PaymentMethodDisplay {
    pub fn builder() -> PaymentMethodDisplayBuilder {
        <PaymentMethodDisplayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDisplayBuilder {
    bank_debit: Option<PaymentMethodDisplayPreview>,
    card: Option<PaymentMethodDisplayPreview>,
    category: Option<PaymentMethodDisplayCategory>,
    display_name: Option<String>,
    id: Option<String>,
    saved: Option<PaymentMethodDisplayPreview>,
    r#type: Option<String>,
    wallet: Option<PaymentMethodDisplayPreview>,
}

impl PaymentMethodDisplayBuilder {
    pub fn bank_debit(mut self, value: PaymentMethodDisplayPreview) -> Self {
        self.bank_debit = Some(value);
        self
    }

    pub fn card(mut self, value: PaymentMethodDisplayPreview) -> Self {
        self.card = Some(value);
        self
    }

    pub fn category(mut self, value: PaymentMethodDisplayCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn saved(mut self, value: PaymentMethodDisplayPreview) -> Self {
        self.saved = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn wallet(mut self, value: PaymentMethodDisplayPreview) -> Self {
        self.wallet = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodDisplay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](PaymentMethodDisplayBuilder::category)
    /// - [`display_name`](PaymentMethodDisplayBuilder::display_name)
    /// - [`r#type`](PaymentMethodDisplayBuilder::r#type)
    pub fn build(self) -> Result<PaymentMethodDisplay, BuildError> {
        Ok(PaymentMethodDisplay {
            bank_debit: self.bank_debit,
            card: self.card,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id,
            saved: self.saved,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            wallet: self.wallet,
        })
    }
}
