pub use crate::prelude::*;

/// The fields of a fee the caller may change. Only the keys sent are replaced.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestDisputeAlertRdr {
    /// The new amount per event in US dollars. `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    /// Changes for the other regions the fee varies by, keyed by region. Only accepted on a fee whose `regions` is non-empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<HashMap<String, UpdateFeesRequestDisputeAlertRdrRegionsValue>>,
}

impl UpdateFeesRequestDisputeAlertRdr {
    pub fn builder() -> UpdateFeesRequestDisputeAlertRdrBuilder {
        <UpdateFeesRequestDisputeAlertRdrBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestDisputeAlertRdrBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
    regions: Option<HashMap<String, UpdateFeesRequestDisputeAlertRdrRegionsValue>>,
}

impl UpdateFeesRequestDisputeAlertRdrBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn regions(
        mut self,
        value: HashMap<String, UpdateFeesRequestDisputeAlertRdrRegionsValue>,
    ) -> Self {
        self.regions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestDisputeAlertRdr`].
    pub fn build(self) -> Result<UpdateFeesRequestDisputeAlertRdr, BuildError> {
        Ok(UpdateFeesRequestDisputeAlertRdr {
            fixed: self.fixed,
            percentage: self.percentage,
            regions: self.regions,
        })
    }
}
