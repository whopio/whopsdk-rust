pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAdCampaignsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted ad campaign.
    #[serde(default)]
    pub id: String,
}

impl DeleteAdCampaignsResponse {
    pub fn builder() -> DeleteAdCampaignsResponseBuilder {
        <DeleteAdCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAdCampaignsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteAdCampaignsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAdCampaignsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteAdCampaignsResponseBuilder::deleted)
    /// - [`id`](DeleteAdCampaignsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteAdCampaignsResponse, BuildError> {
        Ok(DeleteAdCampaignsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
