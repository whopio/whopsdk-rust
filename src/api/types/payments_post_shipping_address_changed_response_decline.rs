pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct PostShippingAddressChangedResponseDecline {
    /// Refuses this address; the buyer picks another.
    pub decline: bool,
}

impl PostShippingAddressChangedResponseDecline {
    pub fn builder() -> PostShippingAddressChangedResponseDeclineBuilder {
        <PostShippingAddressChangedResponseDeclineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostShippingAddressChangedResponseDeclineBuilder {
    decline: Option<bool>,
}

impl PostShippingAddressChangedResponseDeclineBuilder {
    pub fn decline(mut self, value: bool) -> Self {
        self.decline = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostShippingAddressChangedResponseDecline`].
    /// This method will fail if any of the following fields are not set:
    /// - [`decline`](PostShippingAddressChangedResponseDeclineBuilder::decline)
    pub fn build(self) -> Result<PostShippingAddressChangedResponseDecline, BuildError> {
        Ok(PostShippingAddressChangedResponseDecline {
            decline: self
                .decline
                .ok_or_else(|| BuildError::missing_field("decline"))?,
        })
    }
}
