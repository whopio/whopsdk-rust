pub use crate::prelude::*;

/// Sensitive card details. Present only on `GET /cards/:id` for active cards; `null` when the card is inactive or details cannot be retrieved.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCardsResponseSecrets {
    /// Full card number.
    #[serde(default)]
    pub card_number: String,
    /// Card verification code.
    #[serde(default)]
    pub cvc: String,
    /// Cardholder name printed on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_on_card: Option<String>,
    /// The card PIN. Only returned when the request is authenticated as the user the card is assigned to; `null` for all other callers, including account API keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
}

impl CreateCardsResponseSecrets {
    pub fn builder() -> CreateCardsResponseSecretsBuilder {
        <CreateCardsResponseSecretsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCardsResponseSecretsBuilder {
    card_number: Option<String>,
    cvc: Option<String>,
    name_on_card: Option<String>,
    pin: Option<String>,
}

impl CreateCardsResponseSecretsBuilder {
    pub fn card_number(mut self, value: impl Into<String>) -> Self {
        self.card_number = Some(value.into());
        self
    }

    pub fn cvc(mut self, value: impl Into<String>) -> Self {
        self.cvc = Some(value.into());
        self
    }

    pub fn name_on_card(mut self, value: impl Into<String>) -> Self {
        self.name_on_card = Some(value.into());
        self
    }

    pub fn pin(mut self, value: impl Into<String>) -> Self {
        self.pin = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCardsResponseSecrets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card_number`](CreateCardsResponseSecretsBuilder::card_number)
    /// - [`cvc`](CreateCardsResponseSecretsBuilder::cvc)
    pub fn build(self) -> Result<CreateCardsResponseSecrets, BuildError> {
        Ok(CreateCardsResponseSecrets {
            card_number: self
                .card_number
                .ok_or_else(|| BuildError::missing_field("card_number"))?,
            cvc: self.cvc.ok_or_else(|| BuildError::missing_field("cvc"))?,
            name_on_card: self.name_on_card,
            pin: self.pin,
        })
    }
}
