pub use crate::prelude::*;

/// The buyer's identity document when the charge currency has a payer_document_requirements entry for this method, such as ARS card, MODO, or Rapipago. This is independent of the method category.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethodPayerDocument {
    /// The Basis Theory token containing the identity-document number.
    #[serde(default)]
    pub token: String,
    /// The selected identity-document type from the method's payer_document_requirements entry.
    pub r#type: CreateConfirmationTokensRequestPaymentMethodPayerDocumentType,
}

impl CreateConfirmationTokensRequestPaymentMethodPayerDocument {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodPayerDocumentBuilder {
        <CreateConfirmationTokensRequestPaymentMethodPayerDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodPayerDocumentBuilder {
    token: Option<String>,
    r#type: Option<CreateConfirmationTokensRequestPaymentMethodPayerDocumentType>,
}

impl CreateConfirmationTokensRequestPaymentMethodPayerDocumentBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn r#type(
        mut self,
        value: CreateConfirmationTokensRequestPaymentMethodPayerDocumentType,
    ) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethodPayerDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](CreateConfirmationTokensRequestPaymentMethodPayerDocumentBuilder::token)
    /// - [`r#type`](CreateConfirmationTokensRequestPaymentMethodPayerDocumentBuilder::r#type)
    pub fn build(
        self,
    ) -> Result<CreateConfirmationTokensRequestPaymentMethodPayerDocument, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethodPayerDocument {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
