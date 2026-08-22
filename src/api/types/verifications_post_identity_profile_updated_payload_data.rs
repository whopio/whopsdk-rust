pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostIdentityProfileUpdatedPayloadData {
    /// The identity profile id (`idpf_`). Re-fetch `GET /verifications` for its current state.
    #[serde(default)]
    pub id: String,
}

impl PostIdentityProfileUpdatedPayloadData {
    pub fn builder() -> PostIdentityProfileUpdatedPayloadDataBuilder {
        <PostIdentityProfileUpdatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostIdentityProfileUpdatedPayloadDataBuilder {
    id: Option<String>,
}

impl PostIdentityProfileUpdatedPayloadDataBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostIdentityProfileUpdatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostIdentityProfileUpdatedPayloadDataBuilder::id)
    pub fn build(self) -> Result<PostIdentityProfileUpdatedPayloadData, BuildError> {
        Ok(PostIdentityProfileUpdatedPayloadData {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
