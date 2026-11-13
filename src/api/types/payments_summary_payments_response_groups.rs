pub use crate::prelude::*;

/// One entry per requested breakdown, keyed by the field it groups on. A field you did not ask for is absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SummaryPaymentsResponseGroups {
    /// How many of the matching payments are in each currency, keyed by three-letter code. Only currencies with at least one payment are present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<HashMap<String, i64>>,
    /// How many of the matching payments are in each substatus. Every substatus is present, including those with a count of zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substatus: Option<SummaryPaymentsResponseGroupsSubstatus>,
}

impl SummaryPaymentsResponseGroups {
    pub fn builder() -> SummaryPaymentsResponseGroupsBuilder {
        <SummaryPaymentsResponseGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryPaymentsResponseGroupsBuilder {
    currency: Option<HashMap<String, i64>>,
    substatus: Option<SummaryPaymentsResponseGroupsSubstatus>,
}

impl SummaryPaymentsResponseGroupsBuilder {
    pub fn currency(mut self, value: HashMap<String, i64>) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn substatus(mut self, value: SummaryPaymentsResponseGroupsSubstatus) -> Self {
        self.substatus = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryPaymentsResponseGroups`].
    pub fn build(self) -> Result<SummaryPaymentsResponseGroups, BuildError> {
        Ok(SummaryPaymentsResponseGroups {
            currency: self.currency,
            substatus: self.substatus,
        })
    }
}
