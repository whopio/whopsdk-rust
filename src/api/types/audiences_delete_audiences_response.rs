pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAudiencesResponse {
    #[serde(default)]
    pub success: bool,
}

impl DeleteAudiencesResponse {
    pub fn builder() -> DeleteAudiencesResponseBuilder {
        <DeleteAudiencesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAudiencesResponseBuilder {
    success: Option<bool>,
}

impl DeleteAudiencesResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteAudiencesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](DeleteAudiencesResponseBuilder::success)
    pub fn build(self) -> Result<DeleteAudiencesResponse, BuildError> {
        Ok(DeleteAudiencesResponse {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
