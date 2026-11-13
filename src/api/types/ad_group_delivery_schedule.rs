pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupDeliverySchedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<Vec<AdGroupDeliveryWindow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<Vec<AdGroupDeliveryWindow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<Vec<AdGroupDeliveryWindow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<Vec<AdGroupDeliveryWindow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<Vec<AdGroupDeliveryWindow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<Vec<AdGroupDeliveryWindow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<Vec<AdGroupDeliveryWindow>>,
}

impl AdGroupDeliverySchedule {
    pub fn builder() -> AdGroupDeliveryScheduleBuilder {
        <AdGroupDeliveryScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDeliveryScheduleBuilder {
    friday: Option<Vec<AdGroupDeliveryWindow>>,
    monday: Option<Vec<AdGroupDeliveryWindow>>,
    saturday: Option<Vec<AdGroupDeliveryWindow>>,
    sunday: Option<Vec<AdGroupDeliveryWindow>>,
    thursday: Option<Vec<AdGroupDeliveryWindow>>,
    tuesday: Option<Vec<AdGroupDeliveryWindow>>,
    wednesday: Option<Vec<AdGroupDeliveryWindow>>,
}

impl AdGroupDeliveryScheduleBuilder {
    pub fn friday(mut self, value: Vec<AdGroupDeliveryWindow>) -> Self {
        self.friday = Some(value);
        self
    }

    pub fn monday(mut self, value: Vec<AdGroupDeliveryWindow>) -> Self {
        self.monday = Some(value);
        self
    }

    pub fn saturday(mut self, value: Vec<AdGroupDeliveryWindow>) -> Self {
        self.saturday = Some(value);
        self
    }

    pub fn sunday(mut self, value: Vec<AdGroupDeliveryWindow>) -> Self {
        self.sunday = Some(value);
        self
    }

    pub fn thursday(mut self, value: Vec<AdGroupDeliveryWindow>) -> Self {
        self.thursday = Some(value);
        self
    }

    pub fn tuesday(mut self, value: Vec<AdGroupDeliveryWindow>) -> Self {
        self.tuesday = Some(value);
        self
    }

    pub fn wednesday(mut self, value: Vec<AdGroupDeliveryWindow>) -> Self {
        self.wednesday = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDeliverySchedule`].
    pub fn build(self) -> Result<AdGroupDeliverySchedule, BuildError> {
        Ok(AdGroupDeliverySchedule {
            friday: self.friday,
            monday: self.monday,
            saturday: self.saturday,
            sunday: self.sunday,
            thursday: self.thursday,
            tuesday: self.tuesday,
            wednesday: self.wednesday,
        })
    }
}
