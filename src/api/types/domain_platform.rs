pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainPlatform {
    /// Whether Whop holds this address for the account. A platform address is ready as soon as its route is claimed.
    pub state: DomainPlatformState,
}

impl DomainPlatform {
    pub fn builder() -> DomainPlatformBuilder {
        <DomainPlatformBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainPlatformBuilder {
    state: Option<DomainPlatformState>,
}

impl DomainPlatformBuilder {
    pub fn state(mut self, value: DomainPlatformState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainPlatform`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](DomainPlatformBuilder::state)
    pub fn build(self) -> Result<DomainPlatform, BuildError> {
        Ok(DomainPlatform {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
