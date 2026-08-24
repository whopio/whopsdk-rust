pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceName {
    /// User ID.
    #[serde(default)]
    pub id: String,
    /// User display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub object: LedgerActivityResourceNameObject,
    /// User profile image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture_url: Option<String>,
    /// User's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl LedgerActivityResourceName {
    pub fn builder() -> LedgerActivityResourceNameBuilder {
        <LedgerActivityResourceNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceNameBuilder {
    id: Option<String>,
    name: Option<String>,
    object: Option<LedgerActivityResourceNameObject>,
    profile_picture_url: Option<String>,
    username: Option<String>,
}

impl LedgerActivityResourceNameBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceNameObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn profile_picture_url(mut self, value: impl Into<String>) -> Self {
        self.profile_picture_url = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityResourceName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceNameBuilder::id)
    /// - [`object`](LedgerActivityResourceNameBuilder::object)
    pub fn build(self) -> Result<LedgerActivityResourceName, BuildError> {
        Ok(LedgerActivityResourceName {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            profile_picture_url: self.profile_picture_url,
            username: self.username,
        })
    }
}
