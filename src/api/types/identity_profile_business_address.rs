pub use crate::prelude::*;

/// Registered business address reported by the identity provider. Present on `business` profiles.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IdentityProfileBusinessAddress {
    /// The city of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The country of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The line 1 of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// The line 2 of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// The postal code of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl IdentityProfileBusinessAddress {
    pub fn builder() -> IdentityProfileBusinessAddressBuilder {
        <IdentityProfileBusinessAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentityProfileBusinessAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl IdentityProfileBusinessAddressBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = Some(value.into());
        self
    }

    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IdentityProfileBusinessAddress`].
    pub fn build(self) -> Result<IdentityProfileBusinessAddress, BuildError> {
        Ok(IdentityProfileBusinessAddress {
            city: self.city,
            country: self.country,
            line1: self.line1,
            line2: self.line2,
            postal_code: self.postal_code,
            state: self.state,
        })
    }
}
