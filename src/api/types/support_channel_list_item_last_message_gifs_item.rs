pub use crate::prelude::*;

/// Represents a GIF attachment in a feed post
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelListItemLastMessageGifsItem {
    /// The unique identifier of the GIF
    #[serde(default)]
    pub slug: String,
}

impl SupportChannelListItemLastMessageGifsItem {
    pub fn builder() -> SupportChannelListItemLastMessageGifsItemBuilder {
        <SupportChannelListItemLastMessageGifsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelListItemLastMessageGifsItemBuilder {
    slug: Option<String>,
}

impl SupportChannelListItemLastMessageGifsItemBuilder {
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelListItemLastMessageGifsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slug`](SupportChannelListItemLastMessageGifsItemBuilder::slug)
    pub fn build(self) -> Result<SupportChannelListItemLastMessageGifsItem, BuildError> {
        Ok(SupportChannelListItemLastMessageGifsItem {
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
