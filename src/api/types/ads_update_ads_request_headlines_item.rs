pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestHeadlinesItem {
    /// The text shown to viewers.
    #[serde(default)]
    pub text: String,
}

impl UpdateAdsRequestHeadlinesItem {
    pub fn builder() -> UpdateAdsRequestHeadlinesItemBuilder {
        <UpdateAdsRequestHeadlinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestHeadlinesItemBuilder {
    text: Option<String>,
}

impl UpdateAdsRequestHeadlinesItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestHeadlinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](UpdateAdsRequestHeadlinesItemBuilder::text)
    pub fn build(self) -> Result<UpdateAdsRequestHeadlinesItem, BuildError> {
        Ok(UpdateAdsRequestHeadlinesItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
