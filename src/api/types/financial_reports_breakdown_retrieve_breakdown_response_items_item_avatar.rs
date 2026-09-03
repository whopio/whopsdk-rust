pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrieveBreakdownResponseItemsItemAvatar {
    pub shape: RetrieveBreakdownResponseItemsItemAvatarShape,
    /// The image to show, or `null` to fall back to the row's initials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl RetrieveBreakdownResponseItemsItemAvatar {
    pub fn builder() -> RetrieveBreakdownResponseItemsItemAvatarBuilder {
        <RetrieveBreakdownResponseItemsItemAvatarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBreakdownResponseItemsItemAvatarBuilder {
    shape: Option<RetrieveBreakdownResponseItemsItemAvatarShape>,
    url: Option<String>,
}

impl RetrieveBreakdownResponseItemsItemAvatarBuilder {
    pub fn shape(mut self, value: RetrieveBreakdownResponseItemsItemAvatarShape) -> Self {
        self.shape = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBreakdownResponseItemsItemAvatar`].
    /// This method will fail if any of the following fields are not set:
    /// - [`shape`](RetrieveBreakdownResponseItemsItemAvatarBuilder::shape)
    pub fn build(self) -> Result<RetrieveBreakdownResponseItemsItemAvatar, BuildError> {
        Ok(RetrieveBreakdownResponseItemsItemAvatar {
            shape: self
                .shape
                .ok_or_else(|| BuildError::missing_field("shape"))?,
            url: self.url,
        })
    }
}
