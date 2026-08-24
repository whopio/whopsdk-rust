pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePeopleResponseDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}

impl RetrievePeopleResponseDevice {
    pub fn builder() -> RetrievePeopleResponseDeviceBuilder {
        <RetrievePeopleResponseDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseDeviceBuilder {
    browser: Option<String>,
    device: Option<String>,
    os: Option<String>,
}

impl RetrievePeopleResponseDeviceBuilder {
    pub fn browser(mut self, value: impl Into<String>) -> Self {
        self.browser = Some(value.into());
        self
    }

    pub fn device(mut self, value: impl Into<String>) -> Self {
        self.device = Some(value.into());
        self
    }

    pub fn os(mut self, value: impl Into<String>) -> Self {
        self.os = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponseDevice`].
    pub fn build(self) -> Result<RetrievePeopleResponseDevice, BuildError> {
        Ok(RetrievePeopleResponseDevice {
            browser: self.browser,
            device: self.device,
            os: self.os,
        })
    }
}
