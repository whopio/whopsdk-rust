pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdContentReview {
    #[serde(default)]
    pub networks: Vec<AdContentReviewNetwork>,
    /// `reviewed` once Whop content review finishes, `in_review` while it runs, `not_reviewed` when it never ran.
    pub status: AdContentReviewStatus,
    #[serde(default)]
    pub tags: Vec<AdContentReviewTag>,
}

impl AdContentReview {
    pub fn builder() -> AdContentReviewBuilder {
        <AdContentReviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdContentReviewBuilder {
    networks: Option<Vec<AdContentReviewNetwork>>,
    status: Option<AdContentReviewStatus>,
    tags: Option<Vec<AdContentReviewTag>>,
}

impl AdContentReviewBuilder {
    pub fn networks(mut self, value: Vec<AdContentReviewNetwork>) -> Self {
        self.networks = Some(value);
        self
    }

    pub fn status(mut self, value: AdContentReviewStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<AdContentReviewTag>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdContentReview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`networks`](AdContentReviewBuilder::networks)
    /// - [`status`](AdContentReviewBuilder::status)
    /// - [`tags`](AdContentReviewBuilder::tags)
    pub fn build(self) -> Result<AdContentReview, BuildError> {
        Ok(AdContentReview {
            networks: self
                .networks
                .ok_or_else(|| BuildError::missing_field("networks"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
        })
    }
}
