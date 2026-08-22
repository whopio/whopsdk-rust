pub use crate::prelude::*;

/// Cap on how often one person sees ads from this ad group. Only available on campaigns with the `awareness` objective.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdGroupsRequestFrequencyCap {
    /// Most times one person can be shown ads from this ad group within the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_impressions: Option<i64>,
    /// Length of the rolling window, in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_days: Option<i64>,
}

impl UpdateAdGroupsRequestFrequencyCap {
    pub fn builder() -> UpdateAdGroupsRequestFrequencyCapBuilder {
        <UpdateAdGroupsRequestFrequencyCapBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdGroupsRequestFrequencyCapBuilder {
    maximum_impressions: Option<i64>,
    per_days: Option<i64>,
}

impl UpdateAdGroupsRequestFrequencyCapBuilder {
    pub fn maximum_impressions(mut self, value: i64) -> Self {
        self.maximum_impressions = Some(value);
        self
    }

    pub fn per_days(mut self, value: i64) -> Self {
        self.per_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdGroupsRequestFrequencyCap`].
    pub fn build(self) -> Result<UpdateAdGroupsRequestFrequencyCap, BuildError> {
        Ok(UpdateAdGroupsRequestFrequencyCap {
            maximum_impressions: self.maximum_impressions,
            per_days: self.per_days,
        })
    }
}
