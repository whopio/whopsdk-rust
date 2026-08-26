pub use crate::prelude::*;

/// The saved payout method used. Requires payout:destination:read; null without it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostPayoutCreatedPayloadDataPayoutMethod {
    /// Saved payout method nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Supported payout method display details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method:
        Option<PostPayoutCreatedPayloadDataPayoutMethodSupportedPayoutMethod>,
}

impl PostPayoutCreatedPayloadDataPayoutMethod {
    pub fn builder() -> PostPayoutCreatedPayloadDataPayoutMethodBuilder {
        <PostPayoutCreatedPayloadDataPayoutMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutCreatedPayloadDataPayoutMethodBuilder {
    nickname: Option<String>,
    supported_payout_method: Option<PostPayoutCreatedPayloadDataPayoutMethodSupportedPayoutMethod>,
}

impl PostPayoutCreatedPayloadDataPayoutMethodBuilder {
    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn supported_payout_method(
        mut self,
        value: PostPayoutCreatedPayloadDataPayoutMethodSupportedPayoutMethod,
    ) -> Self {
        self.supported_payout_method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutCreatedPayloadDataPayoutMethod`].
    pub fn build(self) -> Result<PostPayoutCreatedPayloadDataPayoutMethod, BuildError> {
        Ok(PostPayoutCreatedPayloadDataPayoutMethod {
            nickname: self.nickname,
            supported_payout_method: self.supported_payout_method,
        })
    }
}
