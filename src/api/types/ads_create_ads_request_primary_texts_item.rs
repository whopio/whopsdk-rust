pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestPrimaryTextsItem {
    /// The text shown to viewers.
    #[serde(default)]
    pub text: String,
}

impl CreateAdsRequestPrimaryTextsItem {
    pub fn builder() -> CreateAdsRequestPrimaryTextsItemBuilder {
        <CreateAdsRequestPrimaryTextsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestPrimaryTextsItemBuilder {
    text: Option<String>,
}

impl CreateAdsRequestPrimaryTextsItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestPrimaryTextsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CreateAdsRequestPrimaryTextsItemBuilder::text)
    pub fn build(self) -> Result<CreateAdsRequestPrimaryTextsItem, BuildError> {
        Ok(CreateAdsRequestPrimaryTextsItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
