pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAppInstallationsRequest {
    /// Your app's own identifier for this installation, such as the ID of the linked account in another system, or `null` when none is set. At most 255 characters and unique across your app's installations, so you can find an installation with `GET /app_installations?external_id=`; setting one another installation already has returns a `409`. Only the app and its developer see it; for the account the app is installed on it is always `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Metadata keys to set, as string values. A key set to `null` is removed and keys you leave out don't change; an empty object clears every key. At most 50 keys; keys are at most 100 characters and values at most 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
}

impl UpdateAppInstallationsRequest {
    pub fn builder() -> UpdateAppInstallationsRequestBuilder {
        <UpdateAppInstallationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAppInstallationsRequestBuilder {
    external_id: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
}

impl UpdateAppInstallationsRequestBuilder {
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAppInstallationsRequest`].
    pub fn build(self) -> Result<UpdateAppInstallationsRequest, BuildError> {
        Ok(UpdateAppInstallationsRequest {
            external_id: self.external_id,
            metadata: self.metadata,
        })
    }
}
