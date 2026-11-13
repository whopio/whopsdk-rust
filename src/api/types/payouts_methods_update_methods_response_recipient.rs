pub use crate::prelude::*;

/// The recipient of a third-party payout method. Present only for recipient payout methods.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMethodsResponseRecipient {
    /// ISO 3166-1 alpha-3 country code.
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
}

impl UpdateMethodsResponseRecipient {
    pub fn builder() -> UpdateMethodsResponseRecipientBuilder {
        <UpdateMethodsResponseRecipientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMethodsResponseRecipientBuilder {
    country: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl UpdateMethodsResponseRecipientBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMethodsResponseRecipient`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](UpdateMethodsResponseRecipientBuilder::country)
    /// - [`email`](UpdateMethodsResponseRecipientBuilder::email)
    /// - [`first_name`](UpdateMethodsResponseRecipientBuilder::first_name)
    /// - [`last_name`](UpdateMethodsResponseRecipientBuilder::last_name)
    pub fn build(self) -> Result<UpdateMethodsResponseRecipient, BuildError> {
        Ok(UpdateMethodsResponseRecipient {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
        })
    }
}
