pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdContentReviewNetwork {
    /// Whether the ad passes Whop's content policy and this network's.
    #[serde(default)]
    pub recommended: bool,
    /// The network. `whop` is Whop's own content policy, which every other network also requires.
    pub title: AdContentReviewNetworkTitle,
}

impl AdContentReviewNetwork {
    pub fn builder() -> AdContentReviewNetworkBuilder {
        <AdContentReviewNetworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdContentReviewNetworkBuilder {
    recommended: Option<bool>,
    title: Option<AdContentReviewNetworkTitle>,
}

impl AdContentReviewNetworkBuilder {
    pub fn recommended(mut self, value: bool) -> Self {
        self.recommended = Some(value);
        self
    }

    pub fn title(mut self, value: AdContentReviewNetworkTitle) -> Self {
        self.title = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdContentReviewNetwork`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recommended`](AdContentReviewNetworkBuilder::recommended)
    /// - [`title`](AdContentReviewNetworkBuilder::title)
    pub fn build(self) -> Result<AdContentReviewNetwork, BuildError> {
        Ok(AdContentReviewNetwork {
            recommended: self
                .recommended
                .ok_or_else(|| BuildError::missing_field("recommended"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
