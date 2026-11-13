pub use crate::prelude::*;

/// The saved payout method used. Requires payout:destination:read; null without it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostPayoutUpdatedPayloadDataPayoutMethod {
    /// Saved payout method nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Supported payout method display details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method:
        Option<PostPayoutUpdatedPayloadDataPayoutMethodSupportedPayoutMethod>,
    /// The kind of payout method the payout used. `external` sent funds out of Whop. `whop_card` moved funds onto the account's own Whop Card.
    pub r#type: PostPayoutUpdatedPayloadDataPayoutMethodType,
}

impl PostPayoutUpdatedPayloadDataPayoutMethod {
    pub fn builder() -> PostPayoutUpdatedPayloadDataPayoutMethodBuilder {
        <PostPayoutUpdatedPayloadDataPayoutMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutUpdatedPayloadDataPayoutMethodBuilder {
    nickname: Option<String>,
    supported_payout_method: Option<PostPayoutUpdatedPayloadDataPayoutMethodSupportedPayoutMethod>,
    r#type: Option<PostPayoutUpdatedPayloadDataPayoutMethodType>,
}

impl PostPayoutUpdatedPayloadDataPayoutMethodBuilder {
    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn supported_payout_method(
        mut self,
        value: PostPayoutUpdatedPayloadDataPayoutMethodSupportedPayoutMethod,
    ) -> Self {
        self.supported_payout_method = Some(value);
        self
    }

    pub fn r#type(mut self, value: PostPayoutUpdatedPayloadDataPayoutMethodType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutUpdatedPayloadDataPayoutMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostPayoutUpdatedPayloadDataPayoutMethodBuilder::r#type)
    pub fn build(self) -> Result<PostPayoutUpdatedPayloadDataPayoutMethod, BuildError> {
        Ok(PostPayoutUpdatedPayloadDataPayoutMethod {
            nickname: self.nickname,
            supported_payout_method: self.supported_payout_method,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
