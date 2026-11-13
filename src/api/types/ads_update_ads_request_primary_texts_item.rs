pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestPrimaryTextsItem {
    /// The text shown to viewers.
    #[serde(default)]
    pub text: String,
}

impl UpdateAdsRequestPrimaryTextsItem {
    pub fn builder() -> UpdateAdsRequestPrimaryTextsItemBuilder {
        <UpdateAdsRequestPrimaryTextsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestPrimaryTextsItemBuilder {
    text: Option<String>,
}

impl UpdateAdsRequestPrimaryTextsItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestPrimaryTextsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](UpdateAdsRequestPrimaryTextsItemBuilder::text)
    pub fn build(self) -> Result<UpdateAdsRequestPrimaryTextsItem, BuildError> {
        Ok(UpdateAdsRequestPrimaryTextsItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
