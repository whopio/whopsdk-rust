pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestDescriptionsItem {
    /// The text shown to viewers.
    #[serde(default)]
    pub text: String,
}

impl UpdateAdsRequestDescriptionsItem {
    pub fn builder() -> UpdateAdsRequestDescriptionsItemBuilder {
        <UpdateAdsRequestDescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestDescriptionsItemBuilder {
    text: Option<String>,
}

impl UpdateAdsRequestDescriptionsItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestDescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](UpdateAdsRequestDescriptionsItemBuilder::text)
    pub fn build(self) -> Result<UpdateAdsRequestDescriptionsItem, BuildError> {
        Ok(UpdateAdsRequestDescriptionsItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
