pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateExperiencesRequest {
    /// The access level of the experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<ExperienceAccessLevels>,
    /// Whether the experience is publicly accessible without a membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// A logo image displayed alongside the experience name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<UpdateExperiencesRequestLogo>,
    /// The display name of the experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether Whop app notifications are enabled for this experience. Webhooks still fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    /// The position of the experience within its section for display ordering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// The unique identifier of the section to move the experience into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<String>,
}

impl UpdateExperiencesRequest {
    pub fn builder() -> UpdateExperiencesRequestBuilder {
        <UpdateExperiencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExperiencesRequestBuilder {
    access_level: Option<ExperienceAccessLevels>,
    is_public: Option<bool>,
    logo: Option<UpdateExperiencesRequestLogo>,
    name: Option<String>,
    notifications_enabled: Option<bool>,
    order: Option<String>,
    section_id: Option<String>,
}

impl UpdateExperiencesRequestBuilder {
    pub fn access_level(mut self, value: ExperienceAccessLevels) -> Self {
        self.access_level = Some(value);
        self
    }

    pub fn is_public(mut self, value: bool) -> Self {
        self.is_public = Some(value);
        self
    }

    pub fn logo(mut self, value: UpdateExperiencesRequestLogo) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notifications_enabled(mut self, value: bool) -> Self {
        self.notifications_enabled = Some(value);
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
        self
    }

    pub fn section_id(mut self, value: impl Into<String>) -> Self {
        self.section_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateExperiencesRequest`].
    pub fn build(self) -> Result<UpdateExperiencesRequest, BuildError> {
        Ok(UpdateExperiencesRequest {
            access_level: self.access_level,
            is_public: self.is_public,
            logo: self.logo,
            name: self.name,
            notifications_enabled: self.notifications_enabled,
            order: self.order,
            section_id: self.section_id,
        })
    }
}
