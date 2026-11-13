pub use crate::prelude::*;

/// How a USD balance pays a bank in another currency: it converts into the payout method's currency, and the payout leaves from the balance in that currency. Null when the payout is paid from the requested balance directly.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateQuotePayoutsResponseConversion {
    /// What the requested amount converts into, in the payout method's currency. This is an estimate when the account's USD has to settle before payout. The payout is then for what the conversion produces. `POST /payouts` answers with the payout request until that payout exists.
    #[serde(default)]
    pub converted_amount: Money,
    /// How much more the recipient receives than if the same amount were paid out in USD and converted at the payout provider's rate. Null when the provider has no price for that route or converting first saves nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings: Option<Money>,
    /// The payout fee in the converted currency, taken from the converted amount.
    #[serde(default)]
    pub payout_fee: Money,
}

impl CreateQuotePayoutsResponseConversion {
    pub fn builder() -> CreateQuotePayoutsResponseConversionBuilder {
        <CreateQuotePayoutsResponseConversionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateQuotePayoutsResponseConversionBuilder {
    converted_amount: Option<Money>,
    estimated_savings: Option<Money>,
    payout_fee: Option<Money>,
}

impl CreateQuotePayoutsResponseConversionBuilder {
    pub fn converted_amount(mut self, value: Money) -> Self {
        self.converted_amount = Some(value);
        self
    }

    pub fn estimated_savings(mut self, value: Money) -> Self {
        self.estimated_savings = Some(value);
        self
    }

    pub fn payout_fee(mut self, value: Money) -> Self {
        self.payout_fee = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateQuotePayoutsResponseConversion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`converted_amount`](CreateQuotePayoutsResponseConversionBuilder::converted_amount)
    /// - [`payout_fee`](CreateQuotePayoutsResponseConversionBuilder::payout_fee)
    pub fn build(self) -> Result<CreateQuotePayoutsResponseConversion, BuildError> {
        Ok(CreateQuotePayoutsResponseConversion {
            converted_amount: self
                .converted_amount
                .ok_or_else(|| BuildError::missing_field("converted_amount"))?,
            estimated_savings: self.estimated_savings,
            payout_fee: self
                .payout_fee
                .ok_or_else(|| BuildError::missing_field("payout_fee"))?,
        })
    }
}
