pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestAdsCertificationsValue {
    /// Must be `pending_information`.
    pub status: UpdatePreferencesRequestAdsCertificationsValueStatus,
}

impl UpdatePreferencesRequestAdsCertificationsValue {
    pub fn builder() -> UpdatePreferencesRequestAdsCertificationsValueBuilder {
        <UpdatePreferencesRequestAdsCertificationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestAdsCertificationsValueBuilder {
    status: Option<UpdatePreferencesRequestAdsCertificationsValueStatus>,
}

impl UpdatePreferencesRequestAdsCertificationsValueBuilder {
    pub fn status(mut self, value: UpdatePreferencesRequestAdsCertificationsValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesRequestAdsCertificationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](UpdatePreferencesRequestAdsCertificationsValueBuilder::status)
    pub fn build(self) -> Result<UpdatePreferencesRequestAdsCertificationsValue, BuildError> {
        Ok(UpdatePreferencesRequestAdsCertificationsValue {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
