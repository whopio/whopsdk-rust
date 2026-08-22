pub use crate::prelude::*;

/// Coarse location, shaped like the event `user` block. It belongs to the owner of the wallet the money moved into or out of — the party the event is about, never their counterparty. Omitted entirely when nothing is known.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PulseEventsResponseDataItemUser {
    /// City name. Omitted when unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// ISO 3166-1 alpha-2 country code. Omitted when unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl PulseEventsResponseDataItemUser {
    pub fn builder() -> PulseEventsResponseDataItemUserBuilder {
        <PulseEventsResponseDataItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PulseEventsResponseDataItemUserBuilder {
    city: Option<String>,
    country: Option<String>,
}

impl PulseEventsResponseDataItemUserBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PulseEventsResponseDataItemUser`].
    pub fn build(self) -> Result<PulseEventsResponseDataItemUser, BuildError> {
        Ok(PulseEventsResponseDataItemUser {
            city: self.city,
            country: self.country,
        })
    }
}
