pub use crate::prelude::*;

/// The banner image displayed at the top of the forum page. Pass null to remove the existing banner.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateForumsRequestBannerImage {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateForumsRequestBannerImage {
    pub fn builder() -> UpdateForumsRequestBannerImageBuilder {
        <UpdateForumsRequestBannerImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateForumsRequestBannerImageBuilder {
    id: Option<String>,
}

impl UpdateForumsRequestBannerImageBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateForumsRequestBannerImage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateForumsRequestBannerImageBuilder::id)
    pub fn build(self) -> Result<UpdateForumsRequestBannerImage, BuildError> {
        Ok(UpdateForumsRequestBannerImage {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
