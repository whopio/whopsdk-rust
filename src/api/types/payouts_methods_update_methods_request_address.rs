pub use crate::prelude::*;

/// The payout address to keep on file for this payout method. Only the fields you send are changed; blank fields are ignored.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMethodsRequestAddress {
    /// The city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The first line of the street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// The second line of the street address, such as an apartment or suite number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// The postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl UpdateMethodsRequestAddress {
    pub fn builder() -> UpdateMethodsRequestAddressBuilder {
        <UpdateMethodsRequestAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMethodsRequestAddressBuilder {
    city: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl UpdateMethodsRequestAddressBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateMethodsRequestAddress`].
    pub fn build(self) -> Result<UpdateMethodsRequestAddress, BuildError> {
        Ok(UpdateMethodsRequestAddress {
            city: self.city,
            line1: self.line1,
            line2: self.line2,
            postal_code: self.postal_code,
            state: self.state,
        })
    }
}
