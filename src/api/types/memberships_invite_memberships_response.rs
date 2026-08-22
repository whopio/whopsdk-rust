pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteMembershipsResponse {
    #[serde(default)]
    pub invitation_sent: bool,
}

impl InviteMembershipsResponse {
    pub fn builder() -> InviteMembershipsResponseBuilder {
        <InviteMembershipsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteMembershipsResponseBuilder {
    invitation_sent: Option<bool>,
}

impl InviteMembershipsResponseBuilder {
    pub fn invitation_sent(mut self, value: bool) -> Self {
        self.invitation_sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InviteMembershipsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invitation_sent`](InviteMembershipsResponseBuilder::invitation_sent)
    pub fn build(self) -> Result<InviteMembershipsResponse, BuildError> {
        Ok(InviteMembershipsResponse {
            invitation_sent: self
                .invitation_sent
                .ok_or_else(|| BuildError::missing_field("invitation_sent"))?,
        })
    }
}
