pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckAccessUsersResponse {
    pub access_level: CheckAccessUsersResponseAccessLevel,
    #[serde(default)]
    pub has_access: bool,
}

impl CheckAccessUsersResponse {
    pub fn builder() -> CheckAccessUsersResponseBuilder {
        <CheckAccessUsersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckAccessUsersResponseBuilder {
    access_level: Option<CheckAccessUsersResponseAccessLevel>,
    has_access: Option<bool>,
}

impl CheckAccessUsersResponseBuilder {
    pub fn access_level(mut self, value: CheckAccessUsersResponseAccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    pub fn has_access(mut self, value: bool) -> Self {
        self.has_access = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckAccessUsersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_level`](CheckAccessUsersResponseBuilder::access_level)
    /// - [`has_access`](CheckAccessUsersResponseBuilder::has_access)
    pub fn build(self) -> Result<CheckAccessUsersResponse, BuildError> {
        Ok(CheckAccessUsersResponse {
            access_level: self
                .access_level
                .ok_or_else(|| BuildError::missing_field("access_level"))?,
            has_access: self
                .has_access
                .ok_or_else(|| BuildError::missing_field("has_access"))?,
        })
    }
}
