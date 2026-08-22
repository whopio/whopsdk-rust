pub use crate::prelude::*;

/// The payment record that is the subject of this resolution case.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionCenterCaseListItemPayment {
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
}

impl ResolutionCenterCaseListItemPayment {
    pub fn builder() -> ResolutionCenterCaseListItemPaymentBuilder {
        <ResolutionCenterCaseListItemPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCaseListItemPaymentBuilder {
    id: Option<String>,
}

impl ResolutionCenterCaseListItemPaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolutionCenterCaseListItemPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResolutionCenterCaseListItemPaymentBuilder::id)
    pub fn build(self) -> Result<ResolutionCenterCaseListItemPayment, BuildError> {
        Ok(ResolutionCenterCaseListItemPayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
