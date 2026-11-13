pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MessageCustomAuthor {
    /// Webhook display name, or null to use the user's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Webhook profile image, or null to use the user's image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<File>,
}

impl MessageCustomAuthor {
    pub fn builder() -> MessageCustomAuthorBuilder {
        <MessageCustomAuthorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageCustomAuthorBuilder {
    display_name: Option<String>,
    profile_picture: Option<File>,
}

impl MessageCustomAuthorBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn profile_picture(mut self, value: File) -> Self {
        self.profile_picture = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageCustomAuthor`].
    pub fn build(self) -> Result<MessageCustomAuthor, BuildError> {
        Ok(MessageCustomAuthor {
            display_name: self.display_name,
            profile_picture: self.profile_picture,
        })
    }
}
