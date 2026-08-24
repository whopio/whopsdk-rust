pub use crate::prelude::*;

/// The billing address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCardUpdatedPayloadDataBilling {
    /// Billing city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Billing country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Street address line 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Street address line 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Billing postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Billing region or state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl PostCardUpdatedPayloadDataBilling {
    pub fn builder() -> PostCardUpdatedPayloadDataBillingBuilder {
        <PostCardUpdatedPayloadDataBillingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCardUpdatedPayloadDataBillingBuilder {
    city: Option<String>,
    country_code: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    region: Option<String>,
}

impl PostCardUpdatedPayloadDataBillingBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
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

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCardUpdatedPayloadDataBilling`].
    pub fn build(self) -> Result<PostCardUpdatedPayloadDataBilling, BuildError> {
        Ok(PostCardUpdatedPayloadDataBilling {
            city: self.city,
            country_code: self.country_code,
            line1: self.line1,
            line2: self.line2,
            postal_code: self.postal_code,
            region: self.region,
        })
    }
}
