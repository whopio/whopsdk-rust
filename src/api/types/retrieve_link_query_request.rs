pub use crate::prelude::*;

/// Query parameters for retrieveLink
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveLinkQueryRequest {
    /// Username from the partner link's `a` query parameter.
    #[serde(default)]
    pub partner_username: String,
    /// Reward slug from the partner link's `reward` query parameter.
    #[serde(default)]
    pub reward_slug: String,
}

impl RetrieveLinkQueryRequest {
    pub fn builder() -> RetrieveLinkQueryRequestBuilder {
        <RetrieveLinkQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveLinkQueryRequestBuilder {
    partner_username: Option<String>,
    reward_slug: Option<String>,
}

impl RetrieveLinkQueryRequestBuilder {
    pub fn partner_username(mut self, value: impl Into<String>) -> Self {
        self.partner_username = Some(value.into());
        self
    }

    pub fn reward_slug(mut self, value: impl Into<String>) -> Self {
        self.reward_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveLinkQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_username`](RetrieveLinkQueryRequestBuilder::partner_username)
    /// - [`reward_slug`](RetrieveLinkQueryRequestBuilder::reward_slug)
    pub fn build(self) -> Result<RetrieveLinkQueryRequest, BuildError> {
        Ok(RetrieveLinkQueryRequest {
            partner_username: self
                .partner_username
                .ok_or_else(|| BuildError::missing_field("partner_username"))?,
            reward_slug: self
                .reward_slug
                .ok_or_else(|| BuildError::missing_field("reward_slug"))?,
        })
    }
}
