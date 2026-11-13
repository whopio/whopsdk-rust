pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Gif {
    /// Height of `original_url`, in pixels.
    #[serde(default)]
    pub height: i64,
    /// Full-size GIF. HTTPS on a Klipy media host.
    #[serde(default)]
    pub original_url: String,
    /// Smaller version for previews and placeholders. HTTPS on a Klipy media host.
    #[serde(default)]
    pub preview_url: String,
    /// Identifier for the GIF.
    #[serde(default)]
    pub slug: String,
    /// GIF title.
    #[serde(default)]
    pub title: String,
    /// URL the message renders the GIF from. HTTPS on a Klipy media host.
    #[serde(default)]
    pub url: String,
    /// Width of `original_url`, in pixels.
    #[serde(default)]
    pub width: i64,
}

impl Gif {
    pub fn builder() -> GifBuilder {
        <GifBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GifBuilder {
    height: Option<i64>,
    original_url: Option<String>,
    preview_url: Option<String>,
    slug: Option<String>,
    title: Option<String>,
    url: Option<String>,
    width: Option<i64>,
}

impl GifBuilder {
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn original_url(mut self, value: impl Into<String>) -> Self {
        self.original_url = Some(value.into());
        self
    }

    pub fn preview_url(mut self, value: impl Into<String>) -> Self {
        self.preview_url = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Gif`].
    /// This method will fail if any of the following fields are not set:
    /// - [`height`](GifBuilder::height)
    /// - [`original_url`](GifBuilder::original_url)
    /// - [`preview_url`](GifBuilder::preview_url)
    /// - [`slug`](GifBuilder::slug)
    /// - [`title`](GifBuilder::title)
    /// - [`url`](GifBuilder::url)
    /// - [`width`](GifBuilder::width)
    pub fn build(self) -> Result<Gif, BuildError> {
        Ok(Gif {
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
            original_url: self
                .original_url
                .ok_or_else(|| BuildError::missing_field("original_url"))?,
            preview_url: self
                .preview_url
                .ok_or_else(|| BuildError::missing_field("preview_url"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
        })
    }
}
