pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferredUsersPartnersResponseDataItem {
    #[serde(default)]
    pub total_earnings_usd: String,
    #[serde(default)]
    pub total_volume_usd: String,
    #[serde(default)]
    pub user: ReferredUsersPartnersResponseDataItemUser,
}

impl ReferredUsersPartnersResponseDataItem {
    pub fn builder() -> ReferredUsersPartnersResponseDataItemBuilder {
        <ReferredUsersPartnersResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferredUsersPartnersResponseDataItemBuilder {
    total_earnings_usd: Option<String>,
    total_volume_usd: Option<String>,
    user: Option<ReferredUsersPartnersResponseDataItemUser>,
}

impl ReferredUsersPartnersResponseDataItemBuilder {
    pub fn total_earnings_usd(mut self, value: impl Into<String>) -> Self {
        self.total_earnings_usd = Some(value.into());
        self
    }

    pub fn total_volume_usd(mut self, value: impl Into<String>) -> Self {
        self.total_volume_usd = Some(value.into());
        self
    }

    pub fn user(mut self, value: ReferredUsersPartnersResponseDataItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferredUsersPartnersResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_earnings_usd`](ReferredUsersPartnersResponseDataItemBuilder::total_earnings_usd)
    /// - [`total_volume_usd`](ReferredUsersPartnersResponseDataItemBuilder::total_volume_usd)
    /// - [`user`](ReferredUsersPartnersResponseDataItemBuilder::user)
    pub fn build(self) -> Result<ReferredUsersPartnersResponseDataItem, BuildError> {
        Ok(ReferredUsersPartnersResponseDataItem {
            total_earnings_usd: self
                .total_earnings_usd
                .ok_or_else(|| BuildError::missing_field("total_earnings_usd"))?,
            total_volume_usd: self
                .total_volume_usd
                .ok_or_else(|| BuildError::missing_field("total_volume_usd"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
