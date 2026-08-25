pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BountyActiveLivestreamFeed {
    /// User hosting the proof livestream — the worker streaming their attempt. `null` if the host account no longer exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<UserSummary>,
    /// Livestream feed ID.
    #[serde(default)]
    pub id: String,
    /// Display title for the proof livestream.
    #[serde(default)]
    pub title: String,
}

impl BountyActiveLivestreamFeed {
    pub fn builder() -> BountyActiveLivestreamFeedBuilder {
        <BountyActiveLivestreamFeedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountyActiveLivestreamFeedBuilder {
    host: Option<UserSummary>,
    id: Option<String>,
    title: Option<String>,
}

impl BountyActiveLivestreamFeedBuilder {
    pub fn host(mut self, value: UserSummary) -> Self {
        self.host = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BountyActiveLivestreamFeed`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BountyActiveLivestreamFeedBuilder::id)
    /// - [`title`](BountyActiveLivestreamFeedBuilder::title)
    pub fn build(self) -> Result<BountyActiveLivestreamFeed, BuildError> {
        Ok(BountyActiveLivestreamFeed {
            host: self.host,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
