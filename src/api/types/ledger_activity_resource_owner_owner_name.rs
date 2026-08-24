pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceOwnerOwnerName {
    /// User ID.
    #[serde(default)]
    pub id: String,
    /// User display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub object: LedgerActivityResourceOwnerOwnerNameObject,
    /// User profile image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture_url: Option<String>,
    /// User's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl LedgerActivityResourceOwnerOwnerName {
    pub fn builder() -> LedgerActivityResourceOwnerOwnerNameBuilder {
        <LedgerActivityResourceOwnerOwnerNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceOwnerOwnerNameBuilder {
    id: Option<String>,
    name: Option<String>,
    object: Option<LedgerActivityResourceOwnerOwnerNameObject>,
    profile_picture_url: Option<String>,
    username: Option<String>,
}

impl LedgerActivityResourceOwnerOwnerNameBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceOwnerOwnerNameObject) -> Self {
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

    /// Consumes the builder and constructs a [`LedgerActivityResourceOwnerOwnerName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceOwnerOwnerNameBuilder::id)
    /// - [`object`](LedgerActivityResourceOwnerOwnerNameBuilder::object)
    pub fn build(self) -> Result<LedgerActivityResourceOwnerOwnerName, BuildError> {
        Ok(LedgerActivityResourceOwnerOwnerName {
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
