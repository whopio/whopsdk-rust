pub use crate::prelude::*;

/// Category `redirect` only. Empty unless the method declares redirect-specific fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethodRedirect {}

impl CreateConfirmationTokensRequestPaymentMethodRedirect {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodRedirectBuilder {
        <CreateConfirmationTokensRequestPaymentMethodRedirectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodRedirectBuilder {}

impl CreateConfirmationTokensRequestPaymentMethodRedirectBuilder {
    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethodRedirect`].
    pub fn build(self) -> Result<CreateConfirmationTokensRequestPaymentMethodRedirect, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethodRedirect {})
    }
}
