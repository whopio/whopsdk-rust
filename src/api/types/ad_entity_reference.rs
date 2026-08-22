pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdEntityReference {
    /// The referenced entity's id.
    #[serde(default)]
    pub id: String,
}

impl AdEntityReference {
    pub fn builder() -> AdEntityReferenceBuilder {
        <AdEntityReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdEntityReferenceBuilder {
    id: Option<String>,
}

impl AdEntityReferenceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdEntityReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AdEntityReferenceBuilder::id)
    pub fn build(self) -> Result<AdEntityReference, BuildError> {
        Ok(AdEntityReference {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
