pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentNextActionRedirectData {
    /// The widest the provider's page lays out usefully, in CSS pixels — cap a frame or dialog presenting it at this width. `null` when the page fills whatever width it is given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_max_width: Option<i64>,
    /// The provider's page for this payment, as an absolute URL — take the buyer there.
    #[serde(default)]
    pub url: String,
}

impl PaymentNextActionRedirectData {
    pub fn builder() -> PaymentNextActionRedirectDataBuilder {
        <PaymentNextActionRedirectDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentNextActionRedirectDataBuilder {
    frame_max_width: Option<i64>,
    url: Option<String>,
}

impl PaymentNextActionRedirectDataBuilder {
    pub fn frame_max_width(mut self, value: i64) -> Self {
        self.frame_max_width = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentNextActionRedirectData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](PaymentNextActionRedirectDataBuilder::url)
    pub fn build(self) -> Result<PaymentNextActionRedirectData, BuildError> {
        Ok(PaymentNextActionRedirectData {
            frame_max_width: self.frame_max_width,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
