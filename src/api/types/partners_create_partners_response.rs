pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePartnersResponse {
    /// The caller's referral link — businesses that sign up through it are attributed to the caller.
    #[serde(default)]
    pub referral_link: String,
    /// When the caller became a Whop partner.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub whop_partner_enabled_at: DateTime<FixedOffset>,
}

impl CreatePartnersResponse {
    pub fn builder() -> CreatePartnersResponseBuilder {
        <CreatePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnersResponseBuilder {
    referral_link: Option<String>,
    whop_partner_enabled_at: Option<DateTime<FixedOffset>>,
}

impl CreatePartnersResponseBuilder {
    pub fn referral_link(mut self, value: impl Into<String>) -> Self {
        self.referral_link = Some(value.into());
        self
    }

    pub fn whop_partner_enabled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.whop_partner_enabled_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`referral_link`](CreatePartnersResponseBuilder::referral_link)
    /// - [`whop_partner_enabled_at`](CreatePartnersResponseBuilder::whop_partner_enabled_at)
    pub fn build(self) -> Result<CreatePartnersResponse, BuildError> {
        Ok(CreatePartnersResponse {
            referral_link: self
                .referral_link
                .ok_or_else(|| BuildError::missing_field("referral_link"))?,
            whop_partner_enabled_at: self
                .whop_partner_enabled_at
                .ok_or_else(|| BuildError::missing_field("whop_partner_enabled_at"))?,
        })
    }
}
