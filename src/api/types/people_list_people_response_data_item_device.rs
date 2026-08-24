pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPeopleResponseDataItemDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}

impl ListPeopleResponseDataItemDevice {
    pub fn builder() -> ListPeopleResponseDataItemDeviceBuilder {
        <ListPeopleResponseDataItemDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseDataItemDeviceBuilder {
    browser: Option<String>,
    device: Option<String>,
    os: Option<String>,
}

impl ListPeopleResponseDataItemDeviceBuilder {
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

    /// Consumes the builder and constructs a [`ListPeopleResponseDataItemDevice`].
    pub fn build(self) -> Result<ListPeopleResponseDataItemDevice, BuildError> {
        Ok(ListPeopleResponseDataItemDevice {
            browser: self.browser,
            device: self.device,
            os: self.os,
        })
    }
}
