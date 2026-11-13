pub use crate::prelude::*;

/// Details for a company card's assigned member when they are not registered as a cardholder yet. Send with billing to register them and start provisioning in one call instead of sending them an onboarding invitation. The legal name comes from the member's approved identity verification when they have one, and from these fields when they do not.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCardsRequestCardholder {
    /// The member's email address. Defaults to the email on their user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The member's legal first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The member's legal last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The member's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl CreateCardsRequestCardholder {
    pub fn builder() -> CreateCardsRequestCardholderBuilder {
        <CreateCardsRequestCardholderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCardsRequestCardholderBuilder {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}

impl CreateCardsRequestCardholderBuilder {
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

    /// Consumes the builder and constructs a [`CreateCardsRequestCardholder`].
    pub fn build(self) -> Result<CreateCardsRequestCardholder, BuildError> {
        Ok(CreateCardsRequestCardholder {
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
        })
    }
}
