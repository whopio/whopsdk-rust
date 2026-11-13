pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateBusinessesResponse {
    pub commission_advance: PartnerAdvance,
    /// The referred business relationship ID.
    #[serde(default)]
    pub id: String,
}

impl UpdateBusinessesResponse {
    pub fn builder() -> UpdateBusinessesResponseBuilder {
        <UpdateBusinessesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBusinessesResponseBuilder {
    commission_advance: Option<PartnerAdvance>,
    id: Option<String>,
}

impl UpdateBusinessesResponseBuilder {
    pub fn commission_advance(mut self, value: PartnerAdvance) -> Self {
        self.commission_advance = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBusinessesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`commission_advance`](UpdateBusinessesResponseBuilder::commission_advance)
    /// - [`id`](UpdateBusinessesResponseBuilder::id)
    pub fn build(self) -> Result<UpdateBusinessesResponse, BuildError> {
        Ok(UpdateBusinessesResponse {
            commission_advance: self
                .commission_advance
                .ok_or_else(|| BuildError::missing_field("commission_advance"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
