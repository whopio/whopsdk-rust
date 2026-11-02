pub use crate::prelude::*;

/// Details for the invited cardholder, accepted only while completing onboarding on an invited card. The legal name comes from an approved identity verification when the invited user has one, and from these fields when they do not.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCardsRequestCardholder {
    /// Email address for the invited cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Legal first name of the invited cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Legal last name of the invited cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Phone number for the invited cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl UpdateCardsRequestCardholder {
    pub fn builder() -> UpdateCardsRequestCardholderBuilder {
        <UpdateCardsRequestCardholderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCardsRequestCardholderBuilder {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}

impl UpdateCardsRequestCardholderBuilder {
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

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCardsRequestCardholder`].
    pub fn build(self) -> Result<UpdateCardsRequestCardholder, BuildError> {
        Ok(UpdateCardsRequestCardholder {
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
        })
    }
}
