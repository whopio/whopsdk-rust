pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AcceptCommissionAdvanceBusinessesResponse {
    pub commission_advance: PartnerAdvance,
    /// The referred business relationship ID.
    #[serde(default)]
    pub id: String,
}

impl AcceptCommissionAdvanceBusinessesResponse {
    pub fn builder() -> AcceptCommissionAdvanceBusinessesResponseBuilder {
        <AcceptCommissionAdvanceBusinessesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcceptCommissionAdvanceBusinessesResponseBuilder {
    commission_advance: Option<PartnerAdvance>,
    id: Option<String>,
}

impl AcceptCommissionAdvanceBusinessesResponseBuilder {
    pub fn commission_advance(mut self, value: PartnerAdvance) -> Self {
        self.commission_advance = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AcceptCommissionAdvanceBusinessesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`commission_advance`](AcceptCommissionAdvanceBusinessesResponseBuilder::commission_advance)
    /// - [`id`](AcceptCommissionAdvanceBusinessesResponseBuilder::id)
    pub fn build(self) -> Result<AcceptCommissionAdvanceBusinessesResponse, BuildError> {
        Ok(AcceptCommissionAdvanceBusinessesResponse {
            commission_advance: self
                .commission_advance
                .ok_or_else(|| BuildError::missing_field("commission_advance"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
