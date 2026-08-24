pub use crate::prelude::*;

/// Optional capture metadata describing where and how the footage was recorded. Persisted on the submission. On a `data_capture` bounty every field except `fov` is required whenever metadata is provided.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBountySubmissionsRequestMetadata {
    /// City the footage was recorded in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country the footage was recorded in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Device the footage was recorded on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Horizontal field of view in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fov: Option<i64>,
    /// Identifier of the person who recorded the footage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// Site or venue the footage was recorded at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    /// Station or position within the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station: Option<String>,
}

impl CreateBountySubmissionsRequestMetadata {
    pub fn builder() -> CreateBountySubmissionsRequestMetadataBuilder {
        <CreateBountySubmissionsRequestMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBountySubmissionsRequestMetadataBuilder {
    city: Option<String>,
    country: Option<String>,
    device: Option<String>,
    fov: Option<i64>,
    operator: Option<String>,
    site: Option<String>,
    station: Option<String>,
}

impl CreateBountySubmissionsRequestMetadataBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn device(mut self, value: impl Into<String>) -> Self {
        self.device = Some(value.into());
        self
    }

    pub fn fov(mut self, value: i64) -> Self {
        self.fov = Some(value);
        self
    }

    pub fn operator(mut self, value: impl Into<String>) -> Self {
        self.operator = Some(value.into());
        self
    }

    pub fn site(mut self, value: impl Into<String>) -> Self {
        self.site = Some(value.into());
        self
    }

    pub fn station(mut self, value: impl Into<String>) -> Self {
        self.station = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBountySubmissionsRequestMetadata`].
    pub fn build(self) -> Result<CreateBountySubmissionsRequestMetadata, BuildError> {
        Ok(CreateBountySubmissionsRequestMetadata {
            city: self.city,
            country: self.country,
            device: self.device,
            fov: self.fov,
            operator: self.operator,
            site: self.site,
            station: self.station,
        })
    }
}
