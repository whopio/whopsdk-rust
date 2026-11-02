pub use crate::prelude::*;

/// The saved payout method used. Requires payout:destination:read; null without it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostPayoutReversedPayloadDataPayoutMethod {
    /// Saved payout method nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Supported payout method display details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method:
        Option<PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod>,
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

    /// Consumes the builder and constructs a [`PostPayoutReversedPayloadDataPayoutMethod`].
    pub fn build(self) -> Result<PostPayoutReversedPayloadDataPayoutMethod, BuildError> {
        Ok(PostPayoutReversedPayloadDataPayoutMethod {
            nickname: self.nickname,
            supported_payout_method: self.supported_payout_method,
        })
    }
}
