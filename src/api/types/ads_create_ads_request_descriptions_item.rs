pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestDescriptionsItem {
    /// The text shown to viewers.
    #[serde(default)]
    pub text: String,
}

impl CreateAdsRequestDescriptionsItem {
    pub fn builder() -> CreateAdsRequestDescriptionsItemBuilder {
        <CreateAdsRequestDescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestDescriptionsItemBuilder {
    text: Option<String>,
}

impl CreateAdsRequestDescriptionsItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestDescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CreateAdsRequestDescriptionsItemBuilder::text)
    pub fn build(self) -> Result<CreateAdsRequestDescriptionsItem, BuildError> {
        Ok(CreateAdsRequestDescriptionsItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
