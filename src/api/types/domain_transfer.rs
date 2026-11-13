pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainTransfer {
    /// The code that takes the domain. For an `internal` move it starts `whop_dt_`, and the receiving Whop account enters it as `registration.auth_code` when it creates or updates the domain. Returned when the response is a single domain and you can manage it; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Where the domain is moving. `internal` is another Whop account.
    pub destination: DomainTransferDestination,
    /// `ready` while the code waits to be used. `pending` once it has: Whop removes the domain's email, Apple Pay and pixel from this account, then moves the registration and removes the domain from this account.
    pub state: DomainTransferState,
}

impl DomainTransfer {
    pub fn builder() -> DomainTransferBuilder {
        <DomainTransferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainTransferBuilder {
    code: Option<String>,
    destination: Option<DomainTransferDestination>,
    state: Option<DomainTransferState>,
}

impl DomainTransferBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn destination(mut self, value: DomainTransferDestination) -> Self {
        self.destination = Some(value);
        self
    }

    pub fn state(mut self, value: DomainTransferState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainTransfer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](DomainTransferBuilder::destination)
    /// - [`state`](DomainTransferBuilder::state)
    pub fn build(self) -> Result<DomainTransfer, BuildError> {
        Ok(DomainTransfer {
            code: self.code,
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
