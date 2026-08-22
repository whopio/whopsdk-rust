pub use crate::prelude::*;

/// The checkout session configuration associated with this setup intent. Null if no checkout session was used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSetupIntentsResponseCheckoutConfiguration {
    /// The unique identifier for the checkout session.
    #[serde(default)]
    pub id: String,
}

impl CreateSetupIntentsResponseCheckoutConfiguration {
    pub fn builder() -> CreateSetupIntentsResponseCheckoutConfigurationBuilder {
        <CreateSetupIntentsResponseCheckoutConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSetupIntentsResponseCheckoutConfigurationBuilder {
    id: Option<String>,
}

impl CreateSetupIntentsResponseCheckoutConfigurationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSetupIntentsResponseCheckoutConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSetupIntentsResponseCheckoutConfigurationBuilder::id)
    pub fn build(self) -> Result<CreateSetupIntentsResponseCheckoutConfiguration, BuildError> {
        Ok(CreateSetupIntentsResponseCheckoutConfiguration {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
