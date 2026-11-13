pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMembersRequest {
    /// Your own key-value data on this member. Up to 50 keys of at most 100 characters. Values are strings of at most 500 characters, numbers, `true`, `false`, or `null`. At most 200 KB. Replaces existing metadata, and `null` clears it. Requires `member:manage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<UpdateMembersRequestMetadataValue>>>,
    /// The name invoices show for this member, in place of the user's own. `null` clears it. Requires `member:manage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// `unsubscribed` to opt out of the account's marketing, `null` to opt back in. Only the member can set it, with `user:notifications:update`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_status: Option<UpdateMembersRequestUnsubscribeStatus>,
}

impl UpdateMembersRequest {
    pub fn builder() -> UpdateMembersRequestBuilder {
        <UpdateMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMembersRequestBuilder {
    metadata: Option<HashMap<String, Option<UpdateMembersRequestMetadataValue>>>,
    name: Option<String>,
    unsubscribe_status: Option<UpdateMembersRequestUnsubscribeStatus>,
}

impl UpdateMembersRequestBuilder {
    pub fn metadata(
        mut self,
        value: HashMap<String, Option<UpdateMembersRequestMetadataValue>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn unsubscribe_status(mut self, value: UpdateMembersRequestUnsubscribeStatus) -> Self {
        self.unsubscribe_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMembersRequest`].
    pub fn build(self) -> Result<UpdateMembersRequest, BuildError> {
        Ok(UpdateMembersRequest {
            metadata: self.metadata,
            name: self.name,
            unsubscribe_status: self.unsubscribe_status,
        })
    }
}
