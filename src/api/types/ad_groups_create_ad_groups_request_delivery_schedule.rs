pub use crate::prelude::*;

/// Hours the ad group delivers in each week, keyed by day. Each window runs from `start` to `end` on the hour, with `24:00` for midnight, and a day's windows can't overlap or touch. A day that's empty or left out doesn't deliver. Replaces the whole schedule; `null` delivers at every hour. Some platforms need a lifetime `budget_type` for a schedule, on the ad group or on its campaign when the campaign holds the budget.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdGroupsRequestDeliverySchedule {
    /// Windows the ad group delivers in on Friday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleFridayItem>>,
    /// Windows the ad group delivers in on Monday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleMondayItem>>,
    /// Windows the ad group delivers in on Saturday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleSaturdayItem>>,
    /// Windows the ad group delivers in on Sunday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleSundayItem>>,
    /// Windows the ad group delivers in on Thursday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleThursdayItem>>,
    /// Windows the ad group delivers in on Tuesday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleTuesdayItem>>,
    /// Windows the ad group delivers in on Wednesday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleWednesdayItem>>,
}

impl CreateAdGroupsRequestDeliverySchedule {
    pub fn builder() -> CreateAdGroupsRequestDeliveryScheduleBuilder {
        <CreateAdGroupsRequestDeliveryScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdGroupsRequestDeliveryScheduleBuilder {
    friday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleFridayItem>>,
    monday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleMondayItem>>,
    saturday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleSaturdayItem>>,
    sunday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleSundayItem>>,
    thursday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleThursdayItem>>,
    tuesday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleTuesdayItem>>,
    wednesday: Option<Vec<CreateAdGroupsRequestDeliveryScheduleWednesdayItem>>,
}

impl CreateAdGroupsRequestDeliveryScheduleBuilder {
    pub fn friday(mut self, value: Vec<CreateAdGroupsRequestDeliveryScheduleFridayItem>) -> Self {
        self.friday = Some(value);
        self
    }

    pub fn monday(mut self, value: Vec<CreateAdGroupsRequestDeliveryScheduleMondayItem>) -> Self {
        self.monday = Some(value);
        self
    }

    pub fn saturday(
        mut self,
        value: Vec<CreateAdGroupsRequestDeliveryScheduleSaturdayItem>,
    ) -> Self {
        self.saturday = Some(value);
        self
    }

    pub fn sunday(mut self, value: Vec<CreateAdGroupsRequestDeliveryScheduleSundayItem>) -> Self {
        self.sunday = Some(value);
        self
    }

    pub fn thursday(
        mut self,
        value: Vec<CreateAdGroupsRequestDeliveryScheduleThursdayItem>,
    ) -> Self {
        self.thursday = Some(value);
        self
    }

    pub fn tuesday(mut self, value: Vec<CreateAdGroupsRequestDeliveryScheduleTuesdayItem>) -> Self {
        self.tuesday = Some(value);
        self
    }

    pub fn wednesday(
        mut self,
        value: Vec<CreateAdGroupsRequestDeliveryScheduleWednesdayItem>,
    ) -> Self {
        self.wednesday = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdGroupsRequestDeliverySchedule`].
    pub fn build(self) -> Result<CreateAdGroupsRequestDeliverySchedule, BuildError> {
        Ok(CreateAdGroupsRequestDeliverySchedule {
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
