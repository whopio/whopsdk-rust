pub use crate::prelude::*;

/// The membership record associated with the disputed payment. Null if the membership no longer exists.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionCenterCaseLegacyMember {
    /// The unique identifier for the extra public member.
    #[serde(default)]
    pub id: String,
}

impl ResolutionCenterCaseLegacyMember {
    pub fn builder() -> ResolutionCenterCaseLegacyMemberBuilder {
        <ResolutionCenterCaseLegacyMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCaseLegacyMemberBuilder {
    id: Option<String>,
}

impl ResolutionCenterCaseLegacyMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolutionCenterCaseLegacyMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResolutionCenterCaseLegacyMemberBuilder::id)
    pub fn build(self) -> Result<ResolutionCenterCaseLegacyMember, BuildError> {
        Ok(ResolutionCenterCaseLegacyMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
