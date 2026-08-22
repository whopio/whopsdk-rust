pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMeUsersRequestProfilePicture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateMeUsersRequestProfilePicture {
    pub fn builder() -> UpdateMeUsersRequestProfilePictureBuilder {
        <UpdateMeUsersRequestProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMeUsersRequestProfilePictureBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl UpdateMeUsersRequestProfilePictureBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMeUsersRequestProfilePicture`].
    pub fn build(self) -> Result<UpdateMeUsersRequestProfilePicture, BuildError> {
        Ok(UpdateMeUsersRequestProfilePicture {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
