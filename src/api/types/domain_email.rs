pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainEmail {
    /// `pending` while Whop sets the domain up with its email provider and publishes its DNS records, `ready` once the provider verified them, and `action_required` when it couldn't; its issue says what happens next. `releasing` while Whop removes email from the domain.
    pub state: DomainEmailState,
}

impl DomainEmail {
    pub fn builder() -> DomainEmailBuilder {
        <DomainEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainEmailBuilder {
    state: Option<DomainEmailState>,
}

impl DomainEmailBuilder {
    pub fn state(mut self, value: DomainEmailState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainEmail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](DomainEmailBuilder::state)
    pub fn build(self) -> Result<DomainEmail, BuildError> {
        Ok(DomainEmail {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
