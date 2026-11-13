pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdContentReviewTag {
    /// The evidence the review recorded for this rule.
    #[serde(default)]
    pub message: String,
    /// The rule Whop content review found the ad breaks.
    #[serde(default)]
    pub title: String,
}

impl AdContentReviewTag {
    pub fn builder() -> AdContentReviewTagBuilder {
        <AdContentReviewTagBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdContentReviewTagBuilder {
    message: Option<String>,
    title: Option<String>,
}

impl AdContentReviewTagBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdContentReviewTag`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](AdContentReviewTagBuilder::message)
    /// - [`title`](AdContentReviewTagBuilder::title)
    pub fn build(self) -> Result<AdContentReviewTag, BuildError> {
        Ok(AdContentReviewTag {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
