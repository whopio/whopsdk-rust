pub use crate::prelude::*;

/// The saved payout method used. Requires payout:destination:read; null without it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostPayoutReversedPayloadDataPayoutMethod {
    /// Saved payout method nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Supported payout method display details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method:
        Option<PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod>,
    /// The kind of payout method the payout used. `external` sent funds out of Whop. `whop_card` moved funds onto the account's own Whop Card.
    pub r#type: PostPayoutReversedPayloadDataPayoutMethodType,
}

impl PostPayoutReversedPayloadDataPayoutMethod {
    pub fn builder() -> PostPayoutReversedPayloadDataPayoutMethodBuilder {
        <PostPayoutReversedPayloadDataPayoutMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutReversedPayloadDataPayoutMethodBuilder {
    nickname: Option<String>,
    supported_payout_method: Option<PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod>,
    r#type: Option<PostPayoutReversedPayloadDataPayoutMethodType>,
}

impl PostPayoutReversedPayloadDataPayoutMethodBuilder {
    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn supported_payout_method(
        mut self,
        value: PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod,
    ) -> Self {
        self.supported_payout_method = Some(value);
        self
    }

    pub fn r#type(mut self, value: PostPayoutReversedPayloadDataPayoutMethodType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutReversedPayloadDataPayoutMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostPayoutReversedPayloadDataPayoutMethodBuilder::r#type)
    pub fn build(self) -> Result<PostPayoutReversedPayloadDataPayoutMethod, BuildError> {
        Ok(PostPayoutReversedPayloadDataPayoutMethod {
            nickname: self.nickname,
            supported_payout_method: self.supported_payout_method,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
