pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEmailAddressesRequest {
    /// ID of the team member who owns the mailbox, prefixed `user_`. Only they can read it or send from it. Leave it out, or pass `null`, to share the mailbox with the account. Giving an owned mailbox to someone else empties it: the owner's mail is never shown to anyone else.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl UpdateEmailAddressesRequest {
    pub fn builder() -> UpdateEmailAddressesRequestBuilder {
        <UpdateEmailAddressesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailAddressesRequestBuilder {
    user_id: Option<String>,
}

impl UpdateEmailAddressesRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEmailAddressesRequest`].
    pub fn build(self) -> Result<UpdateEmailAddressesRequest, BuildError> {
        Ok(UpdateEmailAddressesRequest {
            user_id: self.user_id,
        })
    }
}
