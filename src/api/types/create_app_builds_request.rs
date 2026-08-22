pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAppBuildsRequest {
    /// The AI prompt that generated this build, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_prompt_id: Option<String>,
    /// The app to create the build for, prefixed `app_`. Defaults to the app behind the presented credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The uploaded build file: `{ id }` for an existing file or `{ direct_upload_id }` for a completed direct upload.
    #[serde(default)]
    pub attachment: CreateAppBuildsRequestAttachment,
    /// A client-generated checksum of the build file, used to verify file integrity when unpacked.
    #[serde(default)]
    pub checksum: String,
    /// The target platform for the build.
    pub platform: CreateAppBuildsRequestPlatform,
    /// An optional compressed archive (.zip or .gz) of the source code that produced this build, stored alongside the build so it can be downloaded later. Referenced like `attachment`, and must be a different file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_attachment: Option<CreateAppBuildsRequestSourceAttachment>,
    /// The view types this build supports. Only list the ones its code implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_app_view_types: Option<Vec<CreateAppBuildsRequestSupportedAppViewTypesItem>>,
}

impl CreateAppBuildsRequest {
    pub fn builder() -> CreateAppBuildsRequestBuilder {
        <CreateAppBuildsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAppBuildsRequestBuilder {
    ai_prompt_id: Option<String>,
    app_id: Option<String>,
    attachment: Option<CreateAppBuildsRequestAttachment>,
    checksum: Option<String>,
    platform: Option<CreateAppBuildsRequestPlatform>,
    source_attachment: Option<CreateAppBuildsRequestSourceAttachment>,
    supported_app_view_types: Option<Vec<CreateAppBuildsRequestSupportedAppViewTypesItem>>,
}

impl CreateAppBuildsRequestBuilder {
    pub fn ai_prompt_id(mut self, value: impl Into<String>) -> Self {
        self.ai_prompt_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn attachment(mut self, value: CreateAppBuildsRequestAttachment) -> Self {
        self.attachment = Some(value);
        self
    }

    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = Some(value.into());
        self
    }

    pub fn platform(mut self, value: CreateAppBuildsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn source_attachment(mut self, value: CreateAppBuildsRequestSourceAttachment) -> Self {
        self.source_attachment = Some(value);
        self
    }

    pub fn supported_app_view_types(
        mut self,
        value: Vec<CreateAppBuildsRequestSupportedAppViewTypesItem>,
    ) -> Self {
        self.supported_app_view_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAppBuildsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachment`](CreateAppBuildsRequestBuilder::attachment)
    /// - [`checksum`](CreateAppBuildsRequestBuilder::checksum)
    /// - [`platform`](CreateAppBuildsRequestBuilder::platform)
    pub fn build(self) -> Result<CreateAppBuildsRequest, BuildError> {
        Ok(CreateAppBuildsRequest {
            ai_prompt_id: self.ai_prompt_id,
            app_id: self.app_id,
            attachment: self
                .attachment
                .ok_or_else(|| BuildError::missing_field("attachment"))?,
            checksum: self
                .checksum
                .ok_or_else(|| BuildError::missing_field("checksum"))?,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            source_attachment: self.source_attachment,
            supported_app_view_types: self.supported_app_view_types,
        })
    }
}
