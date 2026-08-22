pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MembershipMember {
    /// What the member can reach on the account: `customer` for paying members, `admin` for team members, `no_access` once every grant has lapsed.
    pub access_level: MembershipMemberAccessLevel,
    /// When the member last opened the account's content, as an ISO 8601 timestamp. `null` if they never have.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
    /// The member's sort position in the buyer's own account list. `null` until they arrange it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub position: Option<f64>,
}

impl MembershipMember {
    pub fn builder() -> MembershipMemberBuilder {
        <MembershipMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipMemberBuilder {
    access_level: Option<MembershipMemberAccessLevel>,
    last_accessed_at: Option<String>,
    position: Option<f64>,
}

impl MembershipMemberBuilder {
    pub fn access_level(mut self, value: MembershipMemberAccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    pub fn last_accessed_at(mut self, value: impl Into<String>) -> Self {
        self.last_accessed_at = Some(value.into());
        self
    }

    pub fn position(mut self, value: f64) -> Self {
        self.position = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembershipMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_level`](MembershipMemberBuilder::access_level)
    pub fn build(self) -> Result<MembershipMember, BuildError> {
        Ok(MembershipMember {
            access_level: self
                .access_level
                .ok_or_else(|| BuildError::missing_field("access_level"))?,
            last_accessed_at: self.last_accessed_at,
            position: self.position,
        })
    }
}
