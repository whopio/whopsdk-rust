pub use crate::prelude::*;

/// The checkout session configuration associated with this setup intent. Null if no checkout session was used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentListItemCheckoutConfiguration {
    /// The unique identifier for the checkout session.
    #[serde(default)]
    pub id: String,
}

impl SetupIntentListItemCheckoutConfiguration {
    pub fn builder() -> SetupIntentListItemCheckoutConfigurationBuilder {
        <SetupIntentListItemCheckoutConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentListItemCheckoutConfigurationBuilder {
    id: Option<String>,
}

impl SetupIntentListItemCheckoutConfigurationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentListItemCheckoutConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetupIntentListItemCheckoutConfigurationBuilder::id)
    pub fn build(self) -> Result<SetupIntentListItemCheckoutConfiguration, BuildError> {
        Ok(SetupIntentListItemCheckoutConfiguration {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
