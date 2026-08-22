pub use crate::prelude::*;

/// The checkout session configuration associated with this setup intent. Null if no checkout session was used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentCheckoutConfiguration {
    /// The unique identifier for the checkout session.
    #[serde(default)]
    pub id: String,
}

impl SetupIntentCheckoutConfiguration {
    pub fn builder() -> SetupIntentCheckoutConfigurationBuilder {
        <SetupIntentCheckoutConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentCheckoutConfigurationBuilder {
    id: Option<String>,
}

impl SetupIntentCheckoutConfigurationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentCheckoutConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetupIntentCheckoutConfigurationBuilder::id)
    pub fn build(self) -> Result<SetupIntentCheckoutConfiguration, BuildError> {
        Ok(SetupIntentCheckoutConfiguration {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
