pub use crate::prelude::*;

/// The live per-speed payout caps for the account in the requested currency — the numbers a payout request is validated against at submit time, so clients can cap an amount input at a value the request will accept. Only present when include_limits is true.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListMethodsResponseLimits {
    /// The currency the caps are denominated in.
    #[serde(default)]
    pub currency: String,
    /// Caps for instant-speed payouts, which additionally draw on pending funds.
    #[serde(default)]
    pub instant: ListMethodsResponseLimitsInstant,
    pub object: ListMethodsResponseLimitsObject,
    /// Caps for standard-speed payouts, which draw on settled funds only.
    #[serde(default)]
    pub standard: ListMethodsResponseLimitsStandard,
}

impl ListMethodsResponseLimits {
    pub fn builder() -> ListMethodsResponseLimitsBuilder {
        <ListMethodsResponseLimitsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMethodsResponseLimitsBuilder {
    currency: Option<String>,
    instant: Option<ListMethodsResponseLimitsInstant>,
    object: Option<ListMethodsResponseLimitsObject>,
    standard: Option<ListMethodsResponseLimitsStandard>,
}

impl ListMethodsResponseLimitsBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn instant(mut self, value: ListMethodsResponseLimitsInstant) -> Self {
        self.instant = Some(value);
        self
    }

    pub fn object(mut self, value: ListMethodsResponseLimitsObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn standard(mut self, value: ListMethodsResponseLimitsStandard) -> Self {
        self.standard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMethodsResponseLimits`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](ListMethodsResponseLimitsBuilder::currency)
    /// - [`instant`](ListMethodsResponseLimitsBuilder::instant)
    /// - [`object`](ListMethodsResponseLimitsBuilder::object)
    /// - [`standard`](ListMethodsResponseLimitsBuilder::standard)
    pub fn build(self) -> Result<ListMethodsResponseLimits, BuildError> {
        Ok(ListMethodsResponseLimits {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            instant: self
                .instant
                .ok_or_else(|| BuildError::missing_field("instant"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            standard: self
                .standard
                .ok_or_else(|| BuildError::missing_field("standard"))?,
        })
    }
}
