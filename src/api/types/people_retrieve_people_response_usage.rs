pub use crate::prelude::*;

/// Exact usage breakdowns for the person's browser traffic (distinct events per value).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePeopleResponseUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<Vec<RetrievePeopleResponseUsageBrowserItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Vec<RetrievePeopleResponseUsageCityItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<RetrievePeopleResponseUsageCountryItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Vec<RetrievePeopleResponseUsageDeviceItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Vec<RetrievePeopleResponseUsageIpItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Vec<RetrievePeopleResponseUsageOsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<Vec<RetrievePeopleResponseUsageReferrerItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Vec<RetrievePeopleResponseUsageTimezoneItem>>,
}

impl RetrievePeopleResponseUsage {
    pub fn builder() -> RetrievePeopleResponseUsageBuilder {
        <RetrievePeopleResponseUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseUsageBuilder {
    browser: Option<Vec<RetrievePeopleResponseUsageBrowserItem>>,
    city: Option<Vec<RetrievePeopleResponseUsageCityItem>>,
    country: Option<Vec<RetrievePeopleResponseUsageCountryItem>>,
    device: Option<Vec<RetrievePeopleResponseUsageDeviceItem>>,
    ip: Option<Vec<RetrievePeopleResponseUsageIpItem>>,
    os: Option<Vec<RetrievePeopleResponseUsageOsItem>>,
    referrer: Option<Vec<RetrievePeopleResponseUsageReferrerItem>>,
    timezone: Option<Vec<RetrievePeopleResponseUsageTimezoneItem>>,
}

impl RetrievePeopleResponseUsageBuilder {
    pub fn browser(mut self, value: Vec<RetrievePeopleResponseUsageBrowserItem>) -> Self {
        self.browser = Some(value);
        self
    }

    pub fn city(mut self, value: Vec<RetrievePeopleResponseUsageCityItem>) -> Self {
        self.city = Some(value);
        self
    }

    pub fn country(mut self, value: Vec<RetrievePeopleResponseUsageCountryItem>) -> Self {
        self.country = Some(value);
        self
    }

    pub fn device(mut self, value: Vec<RetrievePeopleResponseUsageDeviceItem>) -> Self {
        self.device = Some(value);
        self
    }

    pub fn ip(mut self, value: Vec<RetrievePeopleResponseUsageIpItem>) -> Self {
        self.ip = Some(value);
        self
    }

    pub fn os(mut self, value: Vec<RetrievePeopleResponseUsageOsItem>) -> Self {
        self.os = Some(value);
        self
    }

    pub fn referrer(mut self, value: Vec<RetrievePeopleResponseUsageReferrerItem>) -> Self {
        self.referrer = Some(value);
        self
    }

    pub fn timezone(mut self, value: Vec<RetrievePeopleResponseUsageTimezoneItem>) -> Self {
        self.timezone = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseUsage`].
    pub fn build(self) -> Result<RetrievePeopleResponseUsage, BuildError> {
        Ok(RetrievePeopleResponseUsage {
            browser: self.browser,
            city: self.city,
            country: self.country,
            device: self.device,
            ip: self.ip,
            os: self.os,
            referrer: self.referrer,
            timezone: self.timezone,
        })
    }
}
