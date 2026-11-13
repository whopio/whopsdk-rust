pub use crate::prelude::*;

/// Represents a GIF attachment in a feed post
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelLastMessageGifsItem {
    /// The unique identifier of the GIF
    #[serde(default)]
    pub slug: String,
}

impl SupportChannelLastMessageGifsItem {
    pub fn builder() -> SupportChannelLastMessageGifsItemBuilder {
        <SupportChannelLastMessageGifsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelLastMessageGifsItemBuilder {
    slug: Option<String>,
}

impl SupportChannelLastMessageGifsItemBuilder {
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelLastMessageGifsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slug`](SupportChannelLastMessageGifsItemBuilder::slug)
    pub fn build(self) -> Result<SupportChannelLastMessageGifsItem, BuildError> {
        Ok(SupportChannelLastMessageGifsItem {
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
