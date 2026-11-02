pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesResponseAdsCertificationsItem {
    /// Countries every approved application of this type covers, as ISO 3166-1 alpha-2 codes. Ads targeting only these countries are exempt from the category's restrictions.
    #[serde(default)]
    pub approved_countries: Vec<String>,
    /// The business name on the latest application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// The kind of business on the latest application. `null` until the account applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<UpdatePreferencesResponseAdsCertificationsItemBusinessType>,
    /// The certification this entry describes.
    pub certification_type: UpdatePreferencesResponseAdsCertificationsItemCertificationType,
    /// Countries the latest application covers, as ISO 3166-1 alpha-2 codes.
    #[serde(default)]
    pub countries: Vec<String>,
    /// Why the latest application was denied. `null` unless `status` is `denied`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<String>,
    /// The latest application's request ID, prefixed `inrq_`. `null` until the account applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// `not_started` until the account applies; `pending_information` while an application waits for answers; `in_review` once submitted; then `approved` or `denied`.
    pub status: UpdatePreferencesResponseAdsCertificationsItemStatus,
    /// The website on the latest application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpdatePreferencesResponseAdsCertificationsItem {
    pub fn builder() -> UpdatePreferencesResponseAdsCertificationsItemBuilder {
        <UpdatePreferencesResponseAdsCertificationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesResponseAdsCertificationsItemBuilder {
    approved_countries: Option<Vec<String>>,
    business_name: Option<String>,
    business_type: Option<UpdatePreferencesResponseAdsCertificationsItemBusinessType>,
    certification_type: Option<UpdatePreferencesResponseAdsCertificationsItemCertificationType>,
    countries: Option<Vec<String>>,
    denial_reason: Option<String>,
    request_id: Option<String>,
    status: Option<UpdatePreferencesResponseAdsCertificationsItemStatus>,
    url: Option<String>,
}

impl UpdatePreferencesResponseAdsCertificationsItemBuilder {
    pub fn approved_countries(mut self, value: Vec<String>) -> Self {
        self.approved_countries = Some(value);
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_type(
        mut self,
        value: UpdatePreferencesResponseAdsCertificationsItemBusinessType,
    ) -> Self {
        self.business_type = Some(value);
        self
    }

    pub fn certification_type(
        mut self,
        value: UpdatePreferencesResponseAdsCertificationsItemCertificationType,
    ) -> Self {
        self.certification_type = Some(value);
        self
    }

    pub fn countries(mut self, value: Vec<String>) -> Self {
        self.countries = Some(value);
        self
    }

    pub fn denial_reason(mut self, value: impl Into<String>) -> Self {
        self.denial_reason = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: UpdatePreferencesResponseAdsCertificationsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesResponseAdsCertificationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approved_countries`](UpdatePreferencesResponseAdsCertificationsItemBuilder::approved_countries)
    /// - [`certification_type`](UpdatePreferencesResponseAdsCertificationsItemBuilder::certification_type)
    /// - [`countries`](UpdatePreferencesResponseAdsCertificationsItemBuilder::countries)
    /// - [`status`](UpdatePreferencesResponseAdsCertificationsItemBuilder::status)
    pub fn build(self) -> Result<UpdatePreferencesResponseAdsCertificationsItem, BuildError> {
        Ok(UpdatePreferencesResponseAdsCertificationsItem {
            approved_countries: self
                .approved_countries
                .ok_or_else(|| BuildError::missing_field("approved_countries"))?,
            business_name: self.business_name,
            business_type: self.business_type,
            certification_type: self
                .certification_type
                .ok_or_else(|| BuildError::missing_field("certification_type"))?,
            countries: self
                .countries
                .ok_or_else(|| BuildError::missing_field("countries"))?,
            denial_reason: self.denial_reason,
            request_id: self.request_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            url: self.url,
        })
    }
}
