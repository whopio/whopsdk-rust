pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdMusic {
    /// The music attachment's file id.
    #[serde(default)]
    pub id: String,
    /// The uploaded file's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// CDN url of the MP3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AdMusic {
    pub fn builder() -> AdMusicBuilder {
        <AdMusicBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdMusicBuilder {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
}

impl AdMusicBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdMusic`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdMusicBuilder::id)
    pub fn build(self) -> Result<AdMusic, BuildError> {
        Ok(AdMusic {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            url: self.url,
        })
    }
}
