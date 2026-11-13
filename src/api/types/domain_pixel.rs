pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainPixel {
    /// `ready` while the Whop pixel recorded a page view on the domain in the last 7 days. `pending` until a pixel you asked for records its first one, and `action_required` once it goes 7 days without one; its issue says what to do.
    pub state: DomainPixelState,
}

impl DomainPixel {
    pub fn builder() -> DomainPixelBuilder {
        <DomainPixelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainPixelBuilder {
    state: Option<DomainPixelState>,
}

impl DomainPixelBuilder {
    pub fn state(mut self, value: DomainPixelState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainPixel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](DomainPixelBuilder::state)
    pub fn build(self) -> Result<DomainPixel, BuildError> {
        Ok(DomainPixel {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
