pub use crate::prelude::*;

/// Hours the ad group delivers in each week, keyed by day. Each window runs from `start` to `end` on the hour, with `24:00` for midnight, and a day's windows can't overlap or touch. A day that's empty or left out doesn't deliver. Replaces the whole schedule; `null` delivers at every hour. Some platforms need a lifetime `budget_type` for a schedule, on the ad group or on its campaign when the campaign holds the budget.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdGroupsRequestDeliverySchedule {
    /// Windows the ad group delivers in on Friday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleFridayItem>>,
    /// Windows the ad group delivers in on Monday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleMondayItem>>,
    /// Windows the ad group delivers in on Saturday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleSaturdayItem>>,
    /// Windows the ad group delivers in on Sunday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleSundayItem>>,
    /// Windows the ad group delivers in on Thursday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleThursdayItem>>,
    /// Windows the ad group delivers in on Tuesday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleTuesdayItem>>,
    /// Windows the ad group delivers in on Wednesday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleWednesdayItem>>,
}

impl UpdateAdGroupsRequestDeliverySchedule {
    pub fn builder() -> UpdateAdGroupsRequestDeliveryScheduleBuilder {
        <UpdateAdGroupsRequestDeliveryScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdGroupsRequestDeliveryScheduleBuilder {
    friday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleFridayItem>>,
    monday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleMondayItem>>,
    saturday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleSaturdayItem>>,
    sunday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleSundayItem>>,
    thursday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleThursdayItem>>,
    tuesday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleTuesdayItem>>,
    wednesday: Option<Vec<UpdateAdGroupsRequestDeliveryScheduleWednesdayItem>>,
}

impl UpdateAdGroupsRequestDeliveryScheduleBuilder {
    pub fn friday(mut self, value: Vec<UpdateAdGroupsRequestDeliveryScheduleFridayItem>) -> Self {
        self.friday = Some(value);
        self
    }

    pub fn monday(mut self, value: Vec<UpdateAdGroupsRequestDeliveryScheduleMondayItem>) -> Self {
        self.monday = Some(value);
        self
    }

    pub fn saturday(
        mut self,
        value: Vec<UpdateAdGroupsRequestDeliveryScheduleSaturdayItem>,
    ) -> Self {
        self.saturday = Some(value);
        self
    }

    pub fn sunday(mut self, value: Vec<UpdateAdGroupsRequestDeliveryScheduleSundayItem>) -> Self {
        self.sunday = Some(value);
        self
    }

    pub fn thursday(
        mut self,
        value: Vec<UpdateAdGroupsRequestDeliveryScheduleThursdayItem>,
    ) -> Self {
        self.thursday = Some(value);
        self
    }

    pub fn tuesday(mut self, value: Vec<UpdateAdGroupsRequestDeliveryScheduleTuesdayItem>) -> Self {
        self.tuesday = Some(value);
        self
    }

    pub fn wednesday(
        mut self,
        value: Vec<UpdateAdGroupsRequestDeliveryScheduleWednesdayItem>,
    ) -> Self {
        self.wednesday = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdGroupsRequestDeliverySchedule`].
    pub fn build(self) -> Result<UpdateAdGroupsRequestDeliverySchedule, BuildError> {
        Ok(UpdateAdGroupsRequestDeliverySchedule {
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
