pub use crate::prelude::*;

/// The checkout session configuration associated with this setup intent. Null if no checkout session was used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentLegacyCheckoutConfiguration {
    /// The unique identifier for the checkout session.
    #[serde(default)]
    pub id: String,
}

impl SetupIntentLegacyCheckoutConfiguration {
    pub fn builder() -> SetupIntentLegacyCheckoutConfigurationBuilder {
        <SetupIntentLegacyCheckoutConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentLegacyCheckoutConfigurationBuilder {
    id: Option<String>,
}

impl SetupIntentLegacyCheckoutConfigurationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentLegacyCheckoutConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetupIntentLegacyCheckoutConfigurationBuilder::id)
    pub fn build(self) -> Result<SetupIntentLegacyCheckoutConfiguration, BuildError> {
        Ok(SetupIntentLegacyCheckoutConfiguration {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
