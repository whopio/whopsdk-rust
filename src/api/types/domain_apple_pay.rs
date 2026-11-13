pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainApplePay {
    /// `pending` while Whop registers the domain with Apple, `ready` once Apple verified it, and `action_required` when Apple couldn't; its issue says how to fix it. `releasing` while Whop removes it from Apple.
    pub state: DomainApplePayState,
}

impl DomainApplePay {
    pub fn builder() -> DomainApplePayBuilder {
        <DomainApplePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainApplePayBuilder {
    state: Option<DomainApplePayState>,
}

impl DomainApplePayBuilder {
    pub fn state(mut self, value: DomainApplePayState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainApplePay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](DomainApplePayBuilder::state)
    pub fn build(self) -> Result<DomainApplePay, BuildError> {
        Ok(DomainApplePay {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
