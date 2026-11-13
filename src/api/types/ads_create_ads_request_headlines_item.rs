pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestHeadlinesItem {
    /// The text shown to viewers.
    #[serde(default)]
    pub text: String,
}

impl CreateAdsRequestHeadlinesItem {
    pub fn builder() -> CreateAdsRequestHeadlinesItemBuilder {
        <CreateAdsRequestHeadlinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestHeadlinesItemBuilder {
    text: Option<String>,
}

impl CreateAdsRequestHeadlinesItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestHeadlinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CreateAdsRequestHeadlinesItemBuilder::text)
    pub fn build(self) -> Result<CreateAdsRequestHeadlinesItem, BuildError> {
        Ok(CreateAdsRequestHeadlinesItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
