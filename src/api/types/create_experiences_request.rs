pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateExperiencesRequest {
    /// The unique identifier of the company to create this experience for.
    #[serde(default)]
    pub account_id: String,
    /// The unique identifier of the app that powers this experience.
    #[serde(default)]
    pub app_id: String,
    /// Whether the experience is publicly accessible without a membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// A logo image displayed alongside the experience name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<CreateExperiencesRequestLogo>,
    /// The display name of the experience. Defaults to the app's name if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether Whop app notifications are enabled for this experience. Webhooks still fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    /// The unique identifier of the section to place the experience in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<String>,
}

impl CreateExperiencesRequest {
    pub fn builder() -> CreateExperiencesRequestBuilder {
        <CreateExperiencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExperiencesRequestBuilder {
    account_id: Option<String>,
    app_id: Option<String>,
    is_public: Option<bool>,
    logo: Option<CreateExperiencesRequestLogo>,
    name: Option<String>,
    notifications_enabled: Option<bool>,
    section_id: Option<String>,
}

impl CreateExperiencesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn is_public(mut self, value: bool) -> Self {
        self.is_public = Some(value);
        self
    }

    pub fn logo(mut self, value: CreateExperiencesRequestLogo) -> Self {
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

    pub fn section_id(mut self, value: impl Into<String>) -> Self {
        self.section_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateExperiencesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateExperiencesRequestBuilder::account_id)
    /// - [`app_id`](CreateExperiencesRequestBuilder::app_id)
    pub fn build(self) -> Result<CreateExperiencesRequest, BuildError> {
        Ok(CreateExperiencesRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            app_id: self
                .app_id
                .ok_or_else(|| BuildError::missing_field("app_id"))?,
            is_public: self.is_public,
            logo: self.logo,
            name: self.name,
            notifications_enabled: self.notifications_enabled,
            section_id: self.section_id,
        })
    }
}
