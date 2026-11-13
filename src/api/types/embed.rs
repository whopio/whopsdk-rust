pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub fields: Vec<EmbedField>,
    /// Footer shown at the bottom of the embed. Null when the embed has no footer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedImage>,
    /// Time supplied by the embed author, as an ISO 8601 timestamp in UTC. Null when missing or invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Embed {
    pub fn builder() -> EmbedBuilder {
        <EmbedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmbedBuilder {
    color: Option<i64>,
    description: Option<String>,
    fields: Option<Vec<EmbedField>>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedImage>,
    timestamp_at: Option<String>,
    title: Option<String>,
    url: Option<String>,
}

impl EmbedBuilder {
    pub fn color(mut self, value: i64) -> Self {
        self.color = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<EmbedField>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn footer(mut self, value: EmbedFooter) -> Self {
        self.footer = Some(value);
        self
    }

    pub fn image(mut self, value: EmbedImage) -> Self {
        self.image = Some(value);
        self
    }

    pub fn thumbnail(mut self, value: EmbedImage) -> Self {
        self.thumbnail = Some(value);
        self
    }

    pub fn timestamp_at(mut self, value: impl Into<String>) -> Self {
        self.timestamp_at = Some(value.into());
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

    /// Consumes the builder and constructs a [`Embed`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](EmbedBuilder::fields)
    pub fn build(self) -> Result<Embed, BuildError> {
        Ok(Embed {
            color: self.color,
            description: self.description,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            footer: self.footer,
            image: self.image,
            thumbnail: self.thumbnail,
            timestamp_at: self.timestamp_at,
            title: self.title,
            url: self.url,
        })
    }
}
