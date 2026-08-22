pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddFreeDaysMembershipRequest {
    /// The number of free days to add (1-1095). Extends the billing period, expiration date, or Stripe trial depending on plan type.
    #[serde(default)]
    pub free_days: i64,
}

impl AddFreeDaysMembershipRequest {
    pub fn builder() -> AddFreeDaysMembershipRequestBuilder {
        <AddFreeDaysMembershipRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddFreeDaysMembershipRequestBuilder {
    free_days: Option<i64>,
}

impl AddFreeDaysMembershipRequestBuilder {
    pub fn free_days(mut self, value: i64) -> Self {
        self.free_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddFreeDaysMembershipRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`free_days`](AddFreeDaysMembershipRequestBuilder::free_days)
    pub fn build(self) -> Result<AddFreeDaysMembershipRequest, BuildError> {
        Ok(AddFreeDaysMembershipRequest {
            free_days: self
                .free_days
                .ok_or_else(|| BuildError::missing_field("free_days"))?,
        })
    }
}
