pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesRequest {
    /// The outstanding invite to resend (an ausri_ identifier from `pending_owner_invites`).
    #[serde(default)]
    pub invite_id: String,
}

impl ResendInviteBusinessesRequest {
    pub fn builder() -> ResendInviteBusinessesRequestBuilder {
        <ResendInviteBusinessesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesRequestBuilder {
    invite_id: Option<String>,
}

impl ResendInviteBusinessesRequestBuilder {
    pub fn invite_id(mut self, value: impl Into<String>) -> Self {
        self.invite_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invite_id`](ResendInviteBusinessesRequestBuilder::invite_id)
    pub fn build(self) -> Result<ResendInviteBusinessesRequest, BuildError> {
        Ok(ResendInviteBusinessesRequest {
            invite_id: self
                .invite_id
                .ok_or_else(|| BuildError::missing_field("invite_id"))?,
        })
    }
}
